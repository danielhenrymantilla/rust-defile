extern crate proc_macros;

#[doc(hidden)] /** Not part of the public API **/
pub use ::proc_macros::{
    __item__,
// };

// #[::proc_macro_hack::proc_macro_hack]
// /// Same as `item!`, but usable in expression position
// pub use ::proc_macros::{
    __expr_hack__,
};

#[doc(hidden)] /** Not part of the public API **/ #[macro_export]
macro_rules! __as_item__ {(
    $item:item
) => (
    $item
)}

/// Macro to be used to expand to an item definition.
#[macro_export]
macro_rules! item {(
    $($input:tt)*
) => (
    $crate::__as_item__! {
        $crate::__item__! {
            $($input)*
        }
    }
)}

/// Macro to be used to expand to an expression.
#[macro_export]
macro_rules! expr {(
    $($input:tt)*
) => ({
    #[derive($crate::__expr_hack__)]
    enum __defile__Hack__ {
        __defile__Hack__ = (stringify!($($input)*), 42).1
    }
    __defile__Hack__!()
})}
