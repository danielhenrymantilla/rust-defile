#[test]
fn re_entrant() {
    macro_rules! ty {
        (
            i32
        ) => (
            "OK"
        );

        (
            ($A:ty, $B:ty $(,)?)
        ) => (
            [ty!($A), ty!($B)]
        );

        (
            (
                $T:ty $(,)?
            )
        ) => (
            ty! { $T }
        );

        ($T:ty) => (
            ::defile::defile! {
                ty!(@$T)
            }
        );
    }
    assert_eq!(
        ty! {
            ( (i32, i32) )
        },
        ["OK", "OK"],
    );
}
