use core::{
    fmt::{self, Debug},
    ops::{Div, Mul},
};

use crate::{
    fraction::{FractionTrait, One},
    rt::{RtDimensions, RtFraction, RtUnit, UnitRtExt},
    units::*,
    DimensionsTrait,
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

/// Represent unit at type level by storing exponents of the [base units] in
/// [`Dimensions`] struct and relation to the base unit in [`Fraction`] struct:
///
/// Examples:
/// - `Unit<Dimensions<1, 0, 0, 0, 0, 0, 0>, 1/1>` is `m¹ * kg⁰ * s⁰ * ...` is
///   `m` is metre (length).
/// - `Unit<Dimensions<0, 0, 1, 0, 0, 0, 0>, 1/1>` is `m⁰ * kg⁰ * s¹ * ...` is
///   `s` is second (time).
/// - `Unit<Dimensions<1, 0, -1, 0, 0, 0, 0>, 1/1>` is `m¹ * kg⁰ * s⁻¹ * ...` is
///   `m * s⁻¹` is `m / s` metre per second (speed)
/// - `Unit<Dimensions<1, 0, -1, 0, 0, 0, 0>, 1000/3600>` is `m¹ * kg⁰ * s⁻¹ *
///   ... * (1000/3600)` is `m * s⁻¹ * (1000/3600)` is `m / s` kilometre per
///   hour (speed)
///
/// [base units]: https://en.wikipedia.org/wiki/SI_base_unit
/// [`Dimensions`]: crate::Dimensions
/// [`Fraction`]: crate::Fraction
pub struct Unit<D, R = One>(phantasm::Invariant<(D, R)>);

impl<D, R> Unit<D, R> {
    /// Create new unit
    #[inline]
    pub const fn new() -> Self {
        Self(phantasm::Invariant)
    }
}

impl<D, R> Default for Unit<D, R> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<D, R> fmt::Debug for Unit<D, R>
where
    D: Debug + Default,
    R: Debug + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Unit<{dimensions:?}, {ratio:?}>",
            dimensions = D::default(),
            ratio = R::default(),
        ))
    }
}

impl<D, R> fmt::Display for Unit<D, R>
where
    D: DimensionsTrait,
    R: FractionTrait,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match try_get_simple_name::<D, R>() {
            Some(str) => f.write_str(str),
            None => {
                let RtUnit {
                    dimensions:
                        RtDimensions {
                            length,
                            mass,
                            time,
                            electric_current,
                            thermodynamic_temperature,
                            amount_of_substance,
                            luminous_intensity,
                        },
                    ratio: RtFraction { numerator, divisor },
                } = Self::RT;
                let mut first = true;

                macro_rules! push {
                    ($first:ident, $i:ident, $d:expr) => {
                        if $first {
                            match $i {
                                0 => {},
                                1 => {
                                    f.write_str($d)?;
                                    $first = false;
                                },
                                exp => {
                                    f.write_fmt(format_args!("{}^{}", $d, exp))?;
                                    $first = false;
                                },
                            }
                        } else {
                            match $i {
                                0 => {},
                                1 => f.write_fmt(format_args!(" * {}", $d))?,
                                exp => f.write_fmt(format_args!(" * {}^{}", $d, exp))?,
                            }
                        }
                    };
                }

                push!(first, length, "m");
                push!(first, mass, "kg");
                push!(first, time, "s");
                push!(first, electric_current, "A");
                push!(first, thermodynamic_temperature, "K");
                push!(first, amount_of_substance, "mol");
                push!(first, luminous_intensity, "cd");

                if numerator != 1 || divisor != 1 {
                    if first {
                        f.write_fmt(format_args!("(ratio: {}/{})", numerator, divisor))?;
                    } else {
                        f.write_fmt(format_args!(" (ratio: {}/{})", numerator, divisor))?;
                    }
                }

                Ok(())
            },
        }
    }
}

fn try_get_simple_name<D, R>() -> Option<&'static str>
where
    D: DimensionsTrait,
    R: FractionTrait,
{
    macro_rules! r#match {
            (
                $t:ty;
                simple { $( $unit:ty => $s:literal, )+ }
                coherent { $( $unit_:ty => $s_:literal, )+ }
            ) => {
                match <$t>::RT {
                    $(
                        <$crate::prefixes::Yotta::<$unit>>::RT => Some(concat!("Y", $s)),
                        <$crate::prefixes::Zetta::<$unit>>::RT => Some(concat!("Z", $s)),
                        <$crate::prefixes::Exa::<$unit>>::RT => Some(concat!("E", $s)),
                        <$crate::prefixes::Peta::<$unit>>::RT => Some(concat!("P", $s)),
                        <$crate::prefixes::Tera::<$unit>>::RT => Some(concat!("T", $s)),
                        <$crate::prefixes::Giga::<$unit>>::RT => Some(concat!("G", $s)),
                        <$crate::prefixes::Mega::<$unit>>::RT => Some(concat!("M", $s)),
                        <$crate::prefixes::Kilo::<$unit>>::RT => Some(concat!("k", $s)),
                        <$crate::prefixes::Hecto::<$unit>>::RT => Some(concat!("h", $s)),
                        <$crate::prefixes::Deca::<$unit>>::RT => Some(concat!("da", $s)),
                        <$unit>::RT => Some($s),
                        <$crate::prefixes::Deci::<$unit>>::RT => Some(concat!("d", $s)),
                        <$crate::prefixes::Centi::<$unit>>::RT => Some(concat!("c", $s)),
                        <$crate::prefixes::Milli::<$unit>>::RT => Some(concat!("m", $s)),
                        <$crate::prefixes::Micro::<$unit>>::RT => Some(concat!("μ", $s)),
                        <$crate::prefixes::Nano::<$unit>>::RT => Some(concat!("n", $s)),
                        <$crate::prefixes::Pico::<$unit>>::RT => Some(concat!("p", $s)),
                        <$crate::prefixes::Femto::<$unit>>::RT => Some(concat!("f", $s)),
                        <$crate::prefixes::Atto::<$unit>>::RT => Some(concat!("a", $s)),
                        <$crate::prefixes::Zepto::<$unit>>::RT => Some(concat!("z", $s)),
                        <$crate::prefixes::Yocto::<$unit>>::RT => Some(concat!("y", $s)),
                    )+
                    $(
                        <$unit_>::RT => Some($s_),
                    )+
                    _ => None,
                }
            };
        }

    // this is actually match on ~260 variants, yes
    r#match! {
        Unit<D, R>;
        // by "simple" I mean "units those have name and can be concatenated
        // with prefixes (milli/micro/kilo/etc)"
        simple {
            // Base units
            Metre => "m",
            // No kg (see below)
            Second => "s",
            Ampere => "A",
            Kelvin => "K",
            Mole => "mol",
            Candela => "cd",

            // The base unit is kg (kilogram), but when we are
            // writing we want to count ratio from gram
            Gram => "g",

            // Derived units
            // (No Radian/Steradian as they are dimensionless)
            Hertz => "Hz",
            Newton => "N",
            Pascal => "Pa",
            Joule => "J",
            Watt => "W",
        }
        coherent {
            // milli dimensionless (mdimless) and co. is something very strange :D
            Dimensionless => "dimless",

            // Coherent derived units
            SquareMetre => "m^2",
            CubicMetre => "m^3",
            MetrePerSecond => "m/s",

            // Non-SI
            Minute => "min",
            Hour => "h",
            Day => "d",
            KiloMetrePerHour => "km/h",
        }
    }
}

// We need to use handwritten impls to prevent unnecessary bounds on generics
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
    type Output = Unit<<D as Mul<U::Dimensions>>::Output, <R as Mul<U::Ratio>>::Output>;

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
    type Output = Unit<<D as Div<U::Dimensions>>::Output, <R as Div<U::Ratio>>::Output>;

    #[inline]
    fn div(self, _rhs: U) -> Self::Output {
        Unit::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prefixes::{Giga, Kilo, Milli, Nano, Yotta},
        units::*,
        Dimensions, Unit,
    };
    use typenum::{N1, N2, P1, Z0};

    macro_rules! assert_display_eq {
        ($T:ty, $s:expr $(,)?) => {
            assert_eq!(format!("{}", <$T>::new()), $s);
        };
    }

    #[test]
    fn display_base() {
        assert_display_eq!(Metre, "m");
        assert_display_eq!(Mole, "mol");
    }

    #[test]
    fn display_builtin() {
        assert_display_eq!(Hertz, "Hz");
        assert_display_eq!(Joule, "J");
        assert_display_eq!(Watt, "W");
        assert_display_eq!(Gram, "g");
    }

    #[test]
    fn display_coherent() {
        assert_display_eq!(CubicMetre, "m^3");
        assert_display_eq!(MetrePerSecond, "m/s");
        assert_display_eq!(Hour, "h");
        assert_display_eq!(Minute, "min");
        assert_display_eq!(KiloMetrePerHour, "km/h");
    }

    #[test]
    fn display_builtin_prefix() {
        assert_display_eq!(Kilo::<Hertz>, "kHz");
        assert_display_eq!(Yotta::<Joule>, "YJ");
        assert_display_eq!(Giga::<Watt>, "GW");
        assert_display_eq!(Kilo::<Gram>, "kg");
        assert_display_eq!(Milli::<Gram>, "mg");
        assert_display_eq!(Nano::<Metre>, "nm");
    }

    #[test]
    fn display_other() {
        assert_display_eq!(
            Unit::<Dimensions<P1, N2, P1, N1, N1, P1, P1>>,
            "m * kg^-2 * s * A^-1 * K^-1 * mol * cd",
        );
        assert_display_eq!(
            Milli::<Unit::<Dimensions<Z0, Z0, Z0, Z0, Z0, Z0, Z0>>>,
            "(ratio: 1/1000)",
        );
        assert_display_eq!(
            Milli::<Unit::<Dimensions<P1, N2, P1, N1, N1, P1, P1>>>,
            "m * kg^-2 * s * A^-1 * K^-1 * mol * cd (ratio: 1/1000)",
        );
    }
}
