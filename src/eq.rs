/// Represent equality of 2 units by equality of their exponents (dimensions)
/// and equality of their ratios
pub trait UnitEq<Rhs>: sealed::UnitEq<Rhs> {}

impl<U: sealed::UnitEq<Rhs>, Rhs> UnitEq<Rhs> for U {}

/// Represent equality of 2 units by equality of their exponents (dimensions)
/// and equality of their ratios
pub trait DimensionsEq<Rhs>: sealed::DimensionsEq<Rhs> {}

impl<D: sealed::DimensionsEq<Rhs>, Rhs> DimensionsEq<Rhs> for D {}

/// Represent equality of 2 fractions
pub trait FractionEq<Rhs>: sealed::FractionEq<Rhs> {}

impl<T: sealed::FractionEq<Rhs>, Rhs> FractionEq<Rhs> for T {}

mod sealed {
    use crate::{fraction::Fraction, DimensionsTrait, UnitTrait};
    use core::ops::Mul;

    pub trait UnitEq<Rhs> {}

    impl<U, Rhs> UnitEq<Rhs> for U
    where
        U: UnitTrait,
        Rhs: UnitTrait,
        U::Dimensions: super::DimensionsEq<Rhs::Dimensions>,
        U::Ratio: super::FractionEq<Rhs::Ratio>,
    {
    }

    pub trait DimensionsEq<Rhs> {}

    impl<D, Rhs> DimensionsEq<Rhs> for D
    where
        D: DimensionsTrait,
        Rhs: DimensionsTrait<
            Length = D::Length,
            Mass = D::Mass,
            Time = D::Time,
            ElectricCurrent = D::ElectricCurrent,
            ThermodynamicTemperature = D::ThermodynamicTemperature,
            AmountOfSubstance = D::AmountOfSubstance,
            LuminousIntensity = D::LuminousIntensity,
        >,
    {
    }

    pub trait FractionEq<Rhs> {}

    // `A / B = U / V <=> A*V = U*B`
    impl<A, B, U, V> FractionEq<Fraction<A, B>> for Fraction<U, V>
    where
        A: Mul<V>,
        U: Mul<B, Output = A::Output>,
    {
    }
}
