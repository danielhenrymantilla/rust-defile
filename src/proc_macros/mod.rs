extern crate proc_macro;

use ::proc_macro::{*,
    TokenTree as TT,
};

fn map (input: TokenStream)
  -> TokenStream
{
    let mut tokens = input.into_iter();
    let mut ret = TokenStream::new();
    while let Some(tt) = tokens.next() {
        ret.extend(Some(match tt {
            | TT::Punct(ref p)
                if p.as_char() == '@'
            => match tokens.next() {
                | Some(TT::Group(ref group))
                    if group.delimiter() == Delimiter::None
                => {
                    ret.extend(map(group.stream()));
                    continue;
                },
                | _ => {
                    tt
                },
            },
            | TT::Group(group) => {
                Group::new(group.delimiter(),  map(group.stream()))
                    .into()
            },
            | _ => {
                tt
            },
        }));
    }
    ret
}

#[proc_macro] pub
fn __item__ (input: TokenStream)
  -> TokenStream
{
    map(input)
}

/** Not part of the public API **/ #[doc(hidden)]
#[proc_macro_derive(__expr_hack__)] pub
fn __expr_hack__ (input: TokenStream)
  -> TokenStream
{
    // enum
    // EnumName
    // {
    //     VariantName
    //     =
    //     (
    //         stringify
    //         !
    //         (
    //             <input>
    //         )
    // , 0).1,}

    let mut tokens = input.into_iter();
    // `enum EnumName`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `{ <tokens> }`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream().into_iter();
    // `VariantName =`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <tokens> )`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream().into_iter();
    // `stringify !`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <input> )`
    let input = if let Some(TT::Group(it)) = tokens.next() { it } else {
        panic!()
    }.stream();
    let ret = map(input);
    let span = Span::call_site();
    vec![
        TT::Ident(Ident::new("macro_rules", span)),
        TT::Punct(Punct::new('!', Spacing::Alone)),
        TT::Ident(Ident::new("__defile__Hack__", span)),
        TT::Group(Group::new(
            Delimiter::Brace,
            vec![
                TT::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                TT::Punct(Punct::new('=', Spacing::Joint)),
                TT::Punct(Punct::new('>', Spacing::Alone)),
                TT::Group(Group::new(
                    Delimiter::Parenthesis,
                    ret,
                )),
            ].into_iter().collect(),
        )),
    ].into_iter().collect()
}
