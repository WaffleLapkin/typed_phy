use core::{
    any::type_name,
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
    ops::{Div, Mul},
};

use crate::{
    fraction::One,
    TypeOnly,
    DimensionsTrait,
    fraction::FractionTrait
};

/// Trait implemented for [`Unit`].
/// Mostly needed to simplify bound and write
/// ```
/// # use typed_phy::UnitTrait;
/// # trait Trait {}
/// impl<U: UnitTrait> Trait for U {
///     /* ... */
/// }
/// ```
/// Instead of
/// ```
/// # use typed_phy::Unit;
/// # trait Trait {}
/// impl<D, R> Trait for Unit<D, R> {
///     /* ... */
/// }
/// ```
///
/// However, it may be later used for ratio (so kilometre != metre)
///
/// [`Unit`]: struct@Unit
pub trait UnitTrait {
    ///
    type Dimensions: DimensionsTrait;

    /// Ratio
    type Ratio: FractionTrait;
}

impl<D: DimensionsTrait, R: FractionTrait> UnitTrait for Unit<D, R> {
    type Dimensions = D;

    type Ratio = R;
}

/// Represent unit at type level by storing exponents of the [base units] in [`Dimensions`] struct
/// and relation to the base unit in [`Fraction`] struct:
///
/// Examples:
/// - `Unit<Dimensions<1, 0, 0, 0, 0, 0, 0>, 1/1>` is `m¹ * kg⁰ * s⁰ * ...` is `m` is
///   metre (length).
/// - `Unit<Dimensions<0, 0, 1, 0, 0, 0, 0>, 1/1>` is `m⁰ * kg⁰ * s¹ * ...` is `s` is
///   second (time).
/// - `Unit<Dimensions<1, 0, -1, 0, 0, 0, 0>, 1/1>` is `m¹ * kg⁰ * s⁻¹ * ...` is `m * s⁻¹`
///   is `m / s` metre per second (speed)
/// - `Unit<Dimensions<1, 0, -1, 0, 0, 0, 0>, 1000/3600>` is `m¹ * kg⁰ * s⁻¹ * ... * (1000/3600)` is `m * s⁻¹ * (1000/3600)`
///   is `m / s` kilometre per hour (speed)
///
/// [base units]: https://en.wikipedia.org/wiki/SI_base_unit
/// [`Dimensions`]: crate::Dimensions
/// [`Fraction`]: crate::Fraction
pub struct Unit<D, R = One>(TypeOnly<(D, R)>);

impl<D, R> Unit<D, R> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::default())
    }
}

// We need to use handwritten impls to prevent unnecessary bounds on generics
impl<D, R> Debug for Unit<D, R> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: add options to human-readable format
        f.pad(type_name::<Self>())
    }
}

impl<D, R> Clone for Unit<D, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<D, R> Copy for Unit<D, R> {}

/// This adds exponents and multiplies ratios at type-level. E.g.
/// `Unit<1, 0, -1, ..., 1/10> * Unit<0, 0, 1, ..., 10/1> =
/// Unit<1, 0, 0, ..., 1/1>`
///
/// It's used for multiplying quantities.
impl<U, D, R> Mul<U> for Unit<D, R>
where
    U: UnitTrait,
    D: Mul<U::Dimensions>,
    R: Mul<U::Ratio>,
{
    #[allow(clippy::type_complexity)]
    type Output = Unit<
        <D as Mul<U::Dimensions>>::Output,
        <R as Mul<U::Ratio>>::Output,
    >;

    #[inline]
    fn mul(self, _rhs: U) -> Self::Output {
        Unit::new()
    }
}

/// This subs exponents and divides ratios at type-level. E.g.
/// `Unit<1, 0, -1, ..., 1/10> / Unit<0, 0, 1, ..., 10/1> =
/// Unit<1, 0, -2, ..., 1/100>`
///
/// It's used for dividing quantities.
impl<U, D, R> Div<U> for Unit<D, R>
where
    U: UnitTrait,
    D: Div<U::Dimensions>,
    R: Div<U::Ratio>,
{
    // Yeah, it's very complex, but I can't do anything with it :(
    #[allow(clippy::type_complexity)]
    type Output = Unit<
        <D as Div<U::Dimensions>>::Output,
        <R as Div<U::Ratio>>::Output,
    >;

    #[inline]
    fn div(self, _rhs: U) -> Self::Output {
        Unit::new()
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use crate::Unit;

    /// Test that `Unit` implement `Debug + Clone + Copy`
    /// even if generic parameters don't.
    #[test]
    #[allow(dead_code)]
    fn traits() {
        fn assert_bounds<T: Debug + Clone + Copy>(_: T) {}

        fn check<D, R /* no bounds */>() {
            // check that traits are implemented for any generics
            assert_bounds(Unit::<D, R>::new())
        }
    }
}
