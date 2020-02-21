use crate::unit::UnitTrait;

/// Represent equality of 2 units by equality of their exponents (dimensions) and equality of their ratios
pub trait UnitEq<Rhs>
where
    Self: UnitTrait,
    Rhs: UnitTrait<
        Length = Self::Length,
        Mass = Self::Mass,
        Time = Self::Time,
        ElectricCurrent = Self::ElectricCurrent,
        ThermodynamicTemperature = Self::ThermodynamicTemperature,
        AmountOfSubstance = Self::AmountOfSubstance,
        LuminousIntensity = Self::LuminousIntensity,
    >,
    Self::Ratio: FractionEq<Rhs::Ratio>,
{
}

impl<U, Rhs> UnitEq<Rhs> for U
where
    U: UnitTrait,
    Rhs: UnitTrait<
        Length = U::Length,
        Mass = U::Mass,
        Time = U::Time,
        ElectricCurrent = U::ElectricCurrent,
        ThermodynamicTemperature = U::ThermodynamicTemperature,
        AmountOfSubstance = U::AmountOfSubstance,
        LuminousIntensity = U::LuminousIntensity,
    >,
    U::Ratio: FractionEq<Rhs::Ratio>,
{
}

/// Represent equality of 2 fractions
pub trait FractionEq<Rhs>: sealed::FractionEq<Rhs> {}

impl<T, Rhs> FractionEq<Rhs> for T
where
    T: sealed::FractionEq<Rhs>,
{}

mod sealed {
    use crate::fraction::Fraction;
    use core::ops::Mul;

    pub trait FractionEq<Rhs> {}

    impl<A, B, U, V> FractionEq<Fraction<A, B>> for Fraction<U, V>
    where
        A: Mul<V>,
        U: Mul<B, Output = A::Output>,
    {}
}
