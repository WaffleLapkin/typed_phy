use core::{
    any::type_name,
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::TypeOnly;

/// Trait implemented for [`Dimensions`].
/// Mostly needed to simplify bound and write
/// ```
/// # use typed_phy::DimensionsTrait;
/// # trait Trait {}
/// impl<U: DimensionsTrait> Trait for U {
///     /* ... */
/// }
/// ```
/// Instead of
/// ```
/// # use typed_phy::Dimensions;
/// # trait Trait {}
/// impl<L, M, T, I, O, N, J> Trait for Dimensions<L, M, T, I, O, N, J> {
///     /* ... */
/// }
/// ```
///
/// [`Dimesnsions`]: struct@Dimensions
pub trait DimensionsTrait {
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
}

#[rustfmt::skip] // I don't want assoc types to be reordered
impl<L, M, T, I, O, N, J> DimensionsTrait for Dimensions<L, M, T, I, O, N, J> {
    type Length = L;
    type Mass = M;
    type Time = T;
    type ElectricCurrent = I;
    type ThermodynamicTemperature = O;
    type AmountOfSubstance = N;
    type LuminousIntensity = J;
}

/// Represent dimensions of a unit at type level by storing exponents of the [base units]:
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
/// - `Dimensions<P1, Z0, Z0, Z0, Z0, Z0, Z0>` is `m¹ * kg⁰ * s⁰ * ...` is `m` is
///   metre (length).
/// - `Dimensions<Z0, Z0, P1, Z0, Z0, Z0, Z0>` is `m⁰ * kg⁰ * s¹ * ...` is `s` is
///   second (time).
/// - `Dimensions<P1, Z0, N1, Z0, Z0, Z0, Z0>` is `m¹ * kg⁰ * s⁻¹ * ...` is `m * s⁻¹`
///   is `m / s` metre per second (speed)
///
/// [base units]: https://en.wikipedia.org/wiki/SI_base_unit
#[allow(clippy::type_complexity)]
pub struct Dimensions<L, M, T, I, O, N, J>(TypeOnly<(L, M, T, I, O, N, J)>);

impl<L, M, T, I, O, N, J> Dimensions<L, M, T, I, O, N, J> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::default())
    }
}

// We need to use handwritten impls to prevent unnecessary bounds on generics
impl<L, M, T, I, O, N, J> Debug for Dimensions<L, M, T, I, O, N, J> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: add options to human-readable format
        f.pad(type_name::<Self>())
    }
}

impl<L, M, T, I, O, N, J> Clone for Dimensions<L, M, T, I, O, N, J> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<L, M, T, I, O, N, J> Copy for Dimensions<L, M, T, I, O, N, J> {}

/// This adds exponents at type-level. E.g.
/// `Dimensions<1, 0, -1, ...> * Dimensions<0, 0, 1, ...> = /// Dimensions<1, 0, 0, ...>`
///
/// It's used for multiplying quantities.
impl<U, L, M, T, I, O, N, J> Mul<U> for Dimensions<L, M, T, I, O, N, J>
where
    U: DimensionsTrait,
    L: Add<U::Length>,
    M: Add<U::Mass>,
    T: Add<U::Time>,
    I: Add<U::ElectricCurrent>,
    O: Add<U::ThermodynamicTemperature>,
    N: Add<U::AmountOfSubstance>,
    J: Add<U::LuminousIntensity>,
{
    #[allow(clippy::type_complexity)]
    type Output = Dimensions<
        <L as Add<U::Length>>::Output,
        <M as Add<U::Mass>>::Output,
        <T as Add<U::Time>>::Output,
        <I as Add<U::ElectricCurrent>>::Output,
        <O as Add<U::ThermodynamicTemperature>>::Output,
        <N as Add<U::AmountOfSubstance>>::Output,
        <J as Add<U::LuminousIntensity>>::Output,
    >;

    #[inline]
    fn mul(self, _rhs: U) -> Self::Output {
        Dimensions::new()
    }
}

/// This subs exponents and divides ratios at type-level. E.g.
/// `Dimensions<1, 0, -1, ..., 1/10> / Dimensions<0, 0, 1, ..., 10/1> =
/// Dimensions<1, 0, -2, ..., 1/100>`
///
/// It's used for dividing quantities.
impl<U, L, M, T, I, O, N, J> Div<U> for Dimensions<L, M, T, I, O, N, J>
where
    U: DimensionsTrait,
    L: Sub<U::Length>,
    M: Sub<U::Mass>,
    T: Sub<U::Time>,
    I: Sub<U::ElectricCurrent>,
    O: Sub<U::ThermodynamicTemperature>,
    N: Sub<U::AmountOfSubstance>,
    J: Sub<U::LuminousIntensity>,
{
    // Yeah, it's very complex, but I can't do anything with it :(
    #[allow(clippy::type_complexity)]
    type Output = Dimensions<
        <L as Sub<U::Length>>::Output,
        <M as Sub<U::Mass>>::Output,
        <T as Sub<U::Time>>::Output,
        <I as Sub<U::ElectricCurrent>>::Output,
        <O as Sub<U::ThermodynamicTemperature>>::Output,
        <N as Sub<U::AmountOfSubstance>>::Output,
        <J as Sub<U::LuminousIntensity>>::Output,
    >;

    #[inline]
    fn div(self, _rhs: U) -> Self::Output {
        Dimensions::new()
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use typenum::{N2, N3, N4, N5, N6, N7, N8, P1, P2, P3, P4, P5, P6, P7, P8, Z0};

    use super::Dimensions;

    /// Test that `Dimensions` implement `Debug + Clone + Copy`
    /// even if generic parameters don't.
    #[test]
    #[allow(dead_code)]
    fn traits() {
        fn assert_bounds<T: Debug + Clone + Copy>(_: T) {}

        fn check<L, M, T, I, O, N, J /* no bounds */>() {
            // check that traits are implemented for any generics
            assert_bounds(Dimensions::<L, M, T, I, O, N, J>::new())
        }
    }

    #[test]
    fn div() {
        let _: Dimensions<Z0, Z0, Z0, Z0, Z0, Z0, Z0> =
            Dimensions::<P1, P1, P1, P1, P1, P1, P1>::new() / Dimensions::<P1, P1, P1, P1, P1, P1, P1>::new();

        let _: Dimensions<N8, N7, N6, N5, N4, N3, N2> =
            Dimensions::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() / Dimensions::<P8, P7, P6, P5, P4, P3, P2>::new();
    }

    #[test]
    fn mul() {
        let _: Dimensions<P1, P1, P1, P1, P1, P1, P1> =
            Dimensions::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() * Dimensions::<P1, P1, P1, P1, P1, P1, P1>::new();

        let _: Dimensions<P8, N7, P6, N5, P4, N3, P2> =
            Dimensions::<Z0, Z0, Z0, Z0, Z0, Z0, Z0>::new() * Dimensions::<P8, N7, P6, N5, P4, N3, P2>::new();
    }
}
