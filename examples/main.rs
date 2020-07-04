macro_rules! check_expr {
    (
        42
    ) => (
        println! { "Got `42`!" }
    );

    (
        $($token_tree:tt)*
    ) => ({
        println!("Did not get `42`. Instead, got the following tokens:\n[");
        $(
            println!("    `{}`,", stringify!($token_tree));
        )*
        println!("]");
    });
}

macro_rules! check_all_exprs {(
    $(
        $expr:expr
    ),* $(,)?
) => (::defile::__item__! {
    fn main () {
        $(
            println!("vvvvvvvvvvvvv");
            check_expr!($expr);
            println!("^^^^^^^^^^^^^\n");
        )*
    }
})}

check_all_exprs!(42, 1 + 1);
