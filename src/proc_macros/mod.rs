extern crate proc_macro;

use ::proc_macro::{*,
    TokenTree as TT,
};

fn recursive_map(input: TokenStream)
  -> TokenStream
{
    let mut tokens = input.into_iter().peekable();
    let mut ret = TokenStream::new();
    while let Some(mut tt) = tokens.next() {
        ret.extend(::core::iter::once(match tt {
            | TT::Punct(ref p)
                if p.as_char() == '@'
            // We got a `@`, times to see whether…
            => match tokens.peek() {
                // …it prefixes a `None`-delimited group: time for `defile!` to
                // do its job!
                | Some(&TT::Group(ref g))
                    if g.delimiter() == Delimiter::None
                => {
                    // discard the outer `Group` layering, and only pass its
                    // innards
                    ret.extend(recursive_map(g.stream()));
                    drop(tokens.next());
                    continue;
                },

                // …it prefixes another `@`, which means we got `@@`: escape
                // case! Only pass a single `@` through.
                | Some(TT::Punct(ref p))
                    if p.as_char() == '@'
                => {
                    tokens.next().unwrap()
                }
                // …something ungrouped (maybe the metavar did not group it?)
                // this is kind of weird, and could stem from user-provided
                // `:tt`s. We nonetheless assume it's a macro-author-provided @
                // just in front of an ungrouped tt, for which there is nothing
                // to do. We "drop" the current `:tt = '@'`, and `continue`.
                | _ => continue,
            },
            // otherwise, deep-recurse if needed, else pass-through.
            | TT::Group(ref mut g) => {
                let (delimiter, its_span, inner_tokens) =
                    (g.delimiter(), g.span(), g.stream())
                ;
                *g = Group::new(delimiter, recursive_map(inner_tokens));
                g.set_span(its_span);
                tt
            },
            | _ => tt,
        }));
    }
    ret
}

#[proc_macro] pub
fn defile(input: TokenStream)
  -> TokenStream
{
    recursive_map(input)
}
