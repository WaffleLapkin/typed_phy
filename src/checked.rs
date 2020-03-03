//! Traits for checked operations similar to [`core::ops`]'s.
//! We can't use [`num`]'s `Checked*` traits because they assume `Rhs` and
//! `Output` to equal `Self`.
//!
//! [`core::ops`]: core::ops
//! [`num`]: https://rust-num.github.io/num/num_traits/ops/checked/index.html

use core::ops::{Add, Div, Mul, Sub};

/// Performs addition that returns `None` on underflow or overflow.
pub trait CheckedAdd<Rhs = Self>: Add<Rhs> {
    /// Adds two numbers, checking for underflow or overflow. If underflow or
    /// overflow happens, `None` is returned.
    #[must_use]
    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Performs subtraction that returns `None` on underflow or overflow.
pub trait CheckedSub<Rhs = Self>: Sub<Rhs> {
    /// Subs two numbers, checking for underflow or overflow. If underflow or
    /// overflow happens, `None` is returned.
    #[must_use]
    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Performs multiplication that returns `None` on underflow or overflow.
pub trait CheckedMul<Rhs = Self>: Mul<Rhs> {
    /// Multiplies two numbers, checking for underflow or overflow. If underflow
    /// or overflow happens, `None` is returned.
    #[must_use]
    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

/// Performs division that returns `None` on underflow, overflow and
/// division-by-zero.
pub trait CheckedDiv<Rhs = Self>: Div<Rhs> {
    /// Divides two numbers, checking for underflow, overflow and division by
    /// zero. If any of that happens, `None` is returned.
    #[must_use]
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! checked_impls {
    (impl $trait_name:ident by $method:ident for $( $t:ty ),+) => {
        $(
            impl $trait_name for $t {
                #[inline]
                fn $method(self, rhs: Self) -> Option<Self> {
                    Self::$method(self, rhs)
                }
            }
        )+
    }
}

checked_impls!(impl CheckedAdd by checked_add for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
checked_impls!(impl CheckedSub by checked_sub for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
checked_impls!(impl CheckedMul by checked_mul for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
checked_impls!(impl CheckedDiv by checked_div for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
