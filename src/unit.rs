use core::{
    any::type_name,
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::{fraction::One, TypeOnly};

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
/// impl<L, M, T, I, O, N, J, R> Trait for Unit<L, M, T, I, O, N, J, R> {
///     /* ... */
/// }
/// ```
///
/// However, it may be later used for ratio (so kilometre != metre)
///
/// [`Unit`]: struct@Unit
pub trait UnitTrait {
    /// Length, base unit: metre
    type Length;

    /// Mass, base unit: kilogram
    type Mass;

    /// Time, base unit: second
    type Time;

    /// Electric current, base unit: ampere
    type ElectricCurrent;

    /// Thermodynamic temperature, base unit: kelvin
    type ThermodynamicTemperature;

    /// Amount of substance, base unit: mole
    type AmountOfSubstance;

    /// Luminous intensity, base unit: candela
    type LuminousIntensity;

    /// Ratio
    type Ratio;
}

#[rustfmt::skip] // I don't want assoc types to be reordered
impl<L, M, T, I, O, N, J, R> UnitTrait for Unit<L, M, T, I, O, N, J, R> {
    type Length = L;
    type Mass = M;
    type Time = T;
    type ElectricCurrent = I;
    type ThermodynamicTemperature = O;
    type AmountOfSubstance = N;
    type LuminousIntensity = J;
    type Ratio = R;
}

/// Represent unit at type level by storing exponents of the [base units]:
///
/// - `L`, Length
/// - `M`, Mass
/// - `T`, Time
/// - `I`, Electric current
/// - `O` Thermodynamic temperature
/// - `N` Amount of substance
/// - `J` Luminous intensity
///
/// Examples:
/// - `Unit<P1, Z0, Z0, Z0, Z0, Z0, Z0>` is `m¹ * kg⁰ * s⁰ * ...` is `m` is
///   metre (length).
/// - `Unit<Z0, Z0, P1, Z0, Z0, Z0, Z0>` is `m⁰ * kg⁰ * s¹ * ...` is `s` is
///   second (time).
/// - `Unit<P1, Z0, N1, Z0, Z0, Z0, Z0>` is `m¹ * kg⁰ * s⁻¹ * ...` is `m * s⁻¹`
///   is `m / s` metre per second (speed)
///
/// [base units]: https://en.wikipedia.org/wiki/SI_base_unit
#[allow(clippy::type_complexity)]
pub struct Unit<L, M, T, I, O, N, J, R = One>(TypeOnly<(L, M, T, I, O, N, J, R)>);

impl<L, M, T, I, O, N, J, R> Unit<L, M, T, I, O, N, J, R> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::default())
    }
}

// We need to use handwritten impls to prevent unnecessary bounds on generics
impl<L, M, T, I, O, N, J, R> Debug for Unit<L, M, T, I, O, N, J, R> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: add options to human-readable format
        f.pad(type_name::<Self>())
    }
}

impl<L, M, T, I, O, N, J, R> Clone for Unit<L, M, T, I, O, N, J, R> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<L, M, T, I, O, N, J, R> Copy for Unit<L, M, T, I, O, N, J, R> {}

/// This adds exponents at type-level. E.g.
/// `Unit<1, 0, -1, ...> + Unit<0, 0, 1, ...> = Unit<1, 0, 0, ...>`
///
/// It's used for multiplying quantities.
impl<U, L, M, T, I, O, N, J, R> Mul<U> for Unit<L, M, T, I, O, N, J, R>
where
    U: UnitTrait,
    L: Add<U::Length>,
    M: Add<U::Mass>,
    T: Add<U::Time>,
    I: Add<U::ElectricCurrent>,
    O: Add<U::ThermodynamicTemperature>,
    N: Add<U::AmountOfSubstance>,
    J: Add<U::LuminousIntensity>,
    R: Mul<U::Ratio>,
{
    #[allow(clippy::type_complexity)]
    type Output = Unit<
        <L as Add<U::Length>>::Output,
        <M as Add<U::Mass>>::Output,
        <T as Add<U::Time>>::Output,
        <I as Add<U::ElectricCurrent>>::Output,
        <O as Add<U::ThermodynamicTemperature>>::Output,
        <N as Add<U::AmountOfSubstance>>::Output,
        <J as Add<U::LuminousIntensity>>::Output,
        <R as Mul<U::Ratio>>::Output,
    >;

    #[inline]
    fn mul(self, _rhs: U) -> Self::Output {
        Unit::new()
    }
}

/// This subs exponents at type-level. E.g.
/// `Unit<1, 0, -1, ...> - Unit<0, 0, 1, ...> = Unit<1, 0, -2, ...>`
///
/// It's used for dividing quantities.
impl<U, L, M, T, I, O, N, J, R> Div<U> for Unit<L, M, T, I, O, N, J, R>
where
    U: UnitTrait,
    L: Sub<U::Length>,
    M: Sub<U::Mass>,
    T: Sub<U::Time>,
    I: Sub<U::ElectricCurrent>,
    O: Sub<U::ThermodynamicTemperature>,
    N: Sub<U::AmountOfSubstance>,
    J: Sub<U::LuminousIntensity>,
    R: Div<U::Ratio>,
{
    // Yeah, it's very complex, but I can't do anything with it :(
    #[allow(clippy::type_complexity)]
    type Output = Unit<
        <L as Sub<U::Length>>::Output,
        <M as Sub<U::Mass>>::Output,
        <T as Sub<U::Time>>::Output,
        <I as Sub<U::ElectricCurrent>>::Output,
        <O as Sub<U::ThermodynamicTemperature>>::Output,
        <N as Sub<U::AmountOfSubstance>>::Output,
        <J as Sub<U::LuminousIntensity>>::Output,
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

    use typenum::{N2, N3, N4, N5, N6, N7, N8, P1, P2, P3, P4, P5, P6, P7, P8, Z0};

    use crate::Unit;

    /// Test that `Unit` implement `Debug + Clone + Copy`
    /// even if generic parameters don't.
    #[test]
    #[allow(dead_code)]
    fn traits() {
        fn assert_bounds<T: Debug + Clone + Copy>(_: T) {}

        fn check<L, M, T, I, O, N, J /* no bounds */>() {
            // check that traits are implemented for any generics
            assert_bounds(Unit::<L, M, T, I, O, N, J>::new())
        }
    }

    #[test]
    fn div() {
        let _: Unit<Z0, Z0, Z0, Z0, Z0, Z0, Z0> =
            Unit::<P1, P1, P1, P1, P1, P1, P1>::new() / Unit::<P1, P1, P1, P1, P1, P1, P1>::new();

        let _: Unit<N8, N7, N6, N5, N4, N3, N2> =
            Unit::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() / Unit::<P8, P7, P6, P5, P4, P3, P2>::new();
    }

    #[test]
    fn mul() {
        let _: Unit<P1, P1, P1, P1, P1, P1, P1> =
            Unit::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() * Unit::<P1, P1, P1, P1, P1, P1, P1>::new();

        let _: Unit<P8, N7, P6, N5, P4, N3, P2> =
            Unit::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() * Unit::<P8, N7, P6, N5, P4, N3, P2>::new();
    }
}
