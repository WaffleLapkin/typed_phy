use typenum::{P1, Z0, U60, U24};

use crate::{unit::Unit, prefixes::{MulBy, Kilo, Milli}};

/// Just integer.
pub type Dimensionless = Unit<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

// Base units

//                           Electric current
//                   Time ----*.     |     .*---- Thermodynamic temperature
//                 Mass ----*.  \    |    /   .*---- Amount of substance
//           Length ---- L,  M,  T,  I,  O,  N,  J ---- Luminous intensity
/// Metre. `m`
pub type Metre = Unit<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Kilogram. `kg`
pub type KiloGram = Unit<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
/// Second. `s`
pub type Second = Unit<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
/// Ampere. `A`
pub type Ampere = Unit<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
/// Kelvin. `K`
pub type Kelvin = Unit<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
/// Mole. `mol`
pub type Mole = Unit<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
/// Candela. `cd`
pub type Candela = Unit<Z0, Z0, Z0, Z0, Z0, Z0, P1>;

// Derived units
/// Radian. `rad`
pub type Radian = Unit![Metre / Metre];
/// Steradian. `sr`
pub type Steradian = Unit![Metre ^ 2 / Metre ^ 2];
/// Hertz. `Hz`
pub type Hertz = Unit![Dimensionless / Second];
/// Newton. `N`
pub type Newton = Unit![KiloGram * Metre / Second ^ 2];
/// Pascal. `Pa`
pub type Pascal = Unit![KiloGram / Metre / Second ^ 2];
/// Joule. `J`
pub type Joule = Unit![KiloGram * Metre ^ 2 / Second ^ 2];
/// Watt. `W`
pub type Watt = Unit![KiloGram * Metre ^ 2 / Second ^ 3];
// TODO

// Coherent derived units

/// Square metre. `A`
pub type SquareMetre = Unit![Metre ^ 2];
/// Cubic metre. `V`
pub type CubicMetre = Unit![Metre ^ 3];
/// Metre per second. `v`
pub type MetrePerSecond = Unit![Metre / Second];
// TODO

// Non-SI

/// minute. 60 seconds.
pub type Minute = MulBy<Second, U60>;
/// hour. 60 minutes.
pub type Hour = MulBy<Minute, U60>;
/// day. 24 hours.
pub type Day = MulBy<Hour, U24>;
/// Kilometre per hour. `km/h`
pub type KiloMetrePerHour = Unit![Kilo<Metre> / Hour];

// Etc
/// gram. `g`.
pub type Gram = Milli<KiloGram>; // I know, that's weird but in CI base unit is kilogram, not gram.
