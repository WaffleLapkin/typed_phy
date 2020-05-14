//! This is a lib for working with typed physical quantities.
//! It ensures at compile time that you couldn't add meters
//! to seconds or do other weird stuff.
//!
//! Only SI is implemented (at least for now).
//!
//! ## WARNING
//!
//! WIP WIP wip WiP wIp Work in progress Wip wORk In PrOgReSss wIp work in
//! progress
//!
//! ## Project goals
//!
//! - **Correctness** - don't allow wrong things like adding metres to watts
//! - **zero-cost** - at all costs.
//! - **Simplicity** - I know it sounds strange for a lib that uses [`typenum`],
//!   but I want to keep things simple as much as I can. I don't want to solve
//!   _all_ problems. I just want to make SI typed physical quantities.
//! - **Readable docs** - so anyone could read it and understand how the lib
//!   works
//! - **`no_std`** - No std, yes embedded
//!
//! ### Non-goals
//!
//! - Support non-SI
//!
//! ## See also
//!
//! - [`uom`](https://docs.rs/uom)
//!
//! ## Type errors
//!
//! This lib is based on [`typenum`] which type errors aren't readable at all.
//! If you got weird type error consider using [`tnfilt`]
//!
//! [`typenum`]: https://docs.rs/typenum
//! [`tnfilt`]: https://github.com/auxoncorp/tnfilt
//!
//! ## Tune idea
//!
//! At least for now, idea doesn't understand [`typenum`] and [`Unit!`] macro
//! well. To tune idea that it would understand it, you need:
//!
//! - [switch to nightly plugin](https://intellij-rust.github.io/install.html)
//! - enable `org.rust.cargo.evaluate.build.scripts` in `Help | Find Action | Experimental Features`
//!   dialog (see pr [`#4734`](https://github.com/intellij-rust/intellij-rust/pull/4734))
//! - Invalidate caches and restart (in `File | Invalidate caches and restart`
//!   dialog)
//!
//! Now idea should understand types of this lib.
//!
//! [`Unit!`]: macro@Unit
#![cfg_attr(not(test), no_std)]
// For running tests from readme
#![cfg_attr(all(doctest, feature = "nightly"), feature(external_doc))]
//explain TODO
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
// I hate missing docs
#![deny(missing_docs)]
// And I like inline
#![warn(clippy::missing_inline_in_public_items)]

#[macro_use]
mod macros;
pub use macros::NoOpMul;

mod rt;

pub mod checked;
/// Type-level fraction (`A / B`)
pub mod fraction;
/// Trait for integers
pub mod from_int;
/// Type-level gcd (greatest common divisor)
pub mod gcd;
/// Unit prefixes
pub mod prefixes;
/// Simplify fractions
pub mod simplify;
/// Aliases to units
pub mod units;

/* private, but reexported */
mod dimensions;
mod eq;
mod ext;
mod id;
mod quantity;
mod unit;

pub use self::{
    dimensions::{Dimensions, DimensionsTrait},
    eq::{FractionEq, UnitEq},
    ext::IntExt,
    id::Id,
    quantity::Quantity,
    unit::{Unit, UnitTrait},
};

/// UI tests to see weird type errors.
///
/// Those test may seem useless, but I want to see errors that user can
/// easily face.
///
/// Also, it would be cool to add comments explaining errors and ways to resolve
/// them. This could help users.
#[test]
#[cfg(test)]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

/// Run tests from readme
#[cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#[cfg(doctest)]
pub struct ReadmeDocTests;

/// Reexport for macros
#[doc(hidden)]
pub mod reexport {
    pub use typenum::U1;
}
