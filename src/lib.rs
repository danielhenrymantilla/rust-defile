#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "better-docs",
    doc = include_str!("../README.md"),
)]

/// Remove macro metavar ungrouping in the given tokens, otherwise, pass them
/// through.
///
/// ### Usage
///
/// Wrap the whole expansion (containing the call to the submacro) within this
/// macro, and prefix the metavariables you wish to ungroup with `@`.
///
/// If you need to emit an `@` sigil (be it for the helper macro, or for actual
/// Rust syntax), simply escape the sigil by doubling it: `@@`.
pub use ::defile_proc_macros::defile;
