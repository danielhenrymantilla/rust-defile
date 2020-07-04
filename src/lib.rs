#![no_std]
#![cfg_attr(feature = "nightly",
    feature(external_doc),
    doc(include = "../README.md"),
)]

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
///
/// ### Usage
///
/// Wrap the whole expression expansion (containing the call to the
/// submacro) within this macro, and prefix the metavariables you wish
/// to ungroup with `@`.
///
/// If you need to emit an `@` sigil (be it for the helper macro, or for actual
/// Rust syntax), simply escape the sigil by doubling it: `@@`.
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
///
/// ### Usage
///
/// Wrap the whole expression expansion (containing the call to the
/// submacro) within this macro, and prefix the metavariables you wish
/// to ungroup with `@`.
///
/// If you need to emit an `@` sigil (be it for the helper macro, or for actual
/// Rust syntax), simply escape the sigil by doubling it: `@@`.
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
