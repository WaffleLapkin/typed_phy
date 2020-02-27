//! Runtime representation of types (internal API, used for display impl(s))
use typenum::marker_traits::{Integer, Unsigned};

use crate::{fraction::FractionTrait, DimensionsTrait, UnitTrait};

#[derive(Eq, PartialEq)]
pub(crate) struct RtFraction {
    pub(crate) numerator: u64,
    pub(crate) divisor: u64,
}

#[derive(Eq, PartialEq)]
pub(crate) struct RtDimensions {
    pub(crate) length: i8,
    pub(crate) mass: i8,
    pub(crate) time: i8,
    pub(crate) electric_current: i8,
    pub(crate) thermodynamic_temperature: i8,
    pub(crate) amount_of_substance: i8,
    pub(crate) luminous_intensity: i8,
}

#[derive(Eq, PartialEq)]
pub(crate) struct RtUnit {
    pub(crate) dimensions: RtDimensions,
    pub(crate) ratio: RtFraction,
}

pub(crate) trait FractionRtExt: FractionTrait {
    const RT: RtFraction = RtFraction {
        numerator: Self::Numerator::U64,
        divisor: Self::Divisor::U64,
    };
}

impl<T> FractionRtExt for T where T: FractionTrait {}

pub(crate) trait DimensionsRtExt: DimensionsTrait {
    const RT: RtDimensions = RtDimensions {
        length: Self::Length::I8,
        mass: Self::Mass::I8,
        time: Self::Time::I8,
        electric_current: Self::ElectricCurrent::I8,
        thermodynamic_temperature: Self::ThermodynamicTemperature::I8,
        amount_of_substance: Self::AmountOfSubstance::I8,
        luminous_intensity: Self::LuminousIntensity::I8,
    };
}

impl<T> DimensionsRtExt for T where T: DimensionsTrait {}

pub(crate) trait UnitRtExt: UnitTrait {
    const RT: RtUnit = RtUnit {
        dimensions: Self::Dimensions::RT,
        ratio: Self::Ratio::RT,
    };
}

impl<T> UnitRtExt for T where T: UnitTrait {}
