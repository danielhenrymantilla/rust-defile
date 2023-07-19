# `::defile`

Helper proc-macro to "ungroup" a captured metavariable (thus potentially breaking their hygiene, hence the name).

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/defile.rs)
[![Latest version](https://img.shields.io/crates/v/defile.svg)](
https://crates.io/crates/defile)
[![Documentation](https://docs.rs/defile/badge.svg)](
https://docs.rs/defile)
[![MSRV](https://img.shields.io/badge/MSRV-1.54.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/defile.svg)](
https://github.com/danielhenrymantilla/defile.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/defile.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/defile.rs/actions)
[![no_std](https://img.shields.io/badge/no_std-compatible-success.svg)](
https://github.com/rust-secure-code/safety-dance/)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

This is useful when using helper `macro_rules` macros, that need to parse using some special rule (_e.g._ `:expr`, `:path`, `:pat`), but that later want to further inspect the captured variable.

This is not something a `macro_rules!` can do on its own, since such so-called _metavariables_ are seen as an **opaque** single token (`:tt`) (the sequence of tokens captured in the metavariable have been _grouped_ (≈ parenthesized) but using invisible parenthesis.

## Example

```rust
macro_rules! check_expr {
    (
        42
    ) => ({
        println!("Got `42`!");
    });

    (
        $($tt:tt)*
    ) => ({
        println!("Did not get `42`. Instead, got the following tokens:\n[");
        $(
            println!("    `{}`,", stringify!($tt));
        )*
        println!("]");
    });
}

macro_rules! check_all_exprs {(
    $(
        $expr:expr // use :expr to be able to use `,` as a delimiter
    ),* $(,)?
) => (
    fn main () {
        $(
            println!("vvvvvvvvvvvvv");
            check_expr!($expr);
            println!("^^^^^^^^^^^^^\n");
        )*
    }
)}

check_all_exprs!(42, 1 + 1);
```

outputs:

```text
vvvvvvvvvvvvv
Did not get `42`. Instead, got the following tokens:
[
    `42`,
]
^^^^^^^^^^^^^

vvvvvvvvvvvvv
Did not get `42`. Instead, got the following tokens:
[
    `1 + 1`,
]
^^^^^^^^^^^^^
```

  - That is:

    - The token `42` does not match `42`!

    - That being said, the expression `1 + 1` is viewed as a single indivisible
      token too.

      Indeed, that's kind of the point of this behavior: if we do `2 * $expr`
      where `$expr` captures `1 + 1` we expect the result to be `2 * (1 + 1)`
      instead of `2 * 1 + 1`!

But by doing:

<pre><code>macro_rules! check_all_exprs {(
    $(
        $expr:expr // use :expr to be able to use `,` as a delimiter
    ),* $(,)?
) => (<span style="color: lightgreen">::defile::defile! {</span> // 👈
    fn main () {
        $(
            println!("vvvvvvvvvvvvv");
            check_expr!(<span style="color: lightgreen">@</span>$expr);
//                      👆
            println!("^^^^^^^^^^^^^\n");
        )*
    }
<span style="color: lightgreen">}</span>)}</code></pre>

we do get:

```text
vvvvvvvvvvvvv
Got `42`!
^^^^^^^^^^^^^

vvvvvvvvvvvvv
Did not get `42`. Instead, got the following tokens:
[
    `1`,
    `+`,
    `1`,
]
^^^^^^^^^^^^^
```

  - `42` has matched the literal 42, but be aware that this has also resulted
    in `1 + 1` getting split. So, if you were to `defile` expressions such as
    `2 * @$expr`, you may not obtain the expected result! Use with caution.

## Caveats

Currently (`1.45.0`), there are several bugs regarding the interaction between
`macro_rules!` macros and procedural macros, that may lead to `defile!` and any
other helper procedural macro to split groups that are not `@`-prefixed.

Hopefully those bugs are solved, making the actual implementation of `defile!`
meaningful.
