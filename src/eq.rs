use crate::unit::UnitTrait;

/// Represent equality of 2 units by equality of their exponents
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
{
}
