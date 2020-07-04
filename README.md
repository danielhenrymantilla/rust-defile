# `::defile`

Helper proc-macro to "ungroup" a captured metavariable (thus potentially breaking their hygiene, hence the name).

This is useful when using helper `macro_rules` macros, that need to parse using some special rule (_e.g._ `:expr`, `:path`, `:pat`), but that later want to further inspect the captured variable.

This is not something a `macro_rules!` can do on its own, since such so-called _metavariables_ are seen as an **opaque** single token (`:tt`) (the sequence of tokens captured in the metavariable have been _grouped_ (≈ parenthesized) but using invsibile parenthesis.

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

    - the token `42` not match `42`!

    - That being said, the expression `1 + 1` is viewed as a single indivisible
      token too.

      Indeed, that's kind of the point of this behavior: if we do `2 * $expr`
      where `$expr` captures `1 + 1` we expect the result to be `2 * (1 + 1)`
      instead of `2 * 1 + 1`!

But by doing:

```diff
  macro_rules! check_all_exprs {(
      $(
          $expr:expr // use :expr to be able to use `,` as a delimiter
      ),* $(,)?
- ) => (
+ ) => (::defile::item! {
      fn main () {
          $(
              println!("vvvvvvvvvvvvv");
              check_expr!(@$expr); // put `@` before a metavariable to ungroup it
              println!("^^^^^^^^^^^^^\n");
          )*
      }
- )}
+ })}
```

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
