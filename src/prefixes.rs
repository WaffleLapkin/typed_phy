use core::ops::{Mul, Div};

use typenum::{U1, Pow, U10, U3, U2, U6, U9, U12, U15, U18, U21, U24};

use crate::{
    Unit,
    UnitTrait,
};

/// Multiplies ratio of `U` by `X`
pub(crate) type MulPow10<U, P> = MulBy<U, <U10 as Pow<P>>::Output>;
/// Divides ratio of `U` by `X`
pub(crate) type DivPow10<U, P> = DivBy<U, <U10 as Pow<P>>::Output>;


/// yotta prefix. `Y`. (Base 10: `10^24`, decimal: `1000000000000000000000000`, word: septillion/quadrillion, adoption: 1991)
pub type Yotta<U> = MulPow10<U, U24>;
/// zetta prefix. `Z`. (Base 10: `10^21`, decimal: `1000000000000000000000`, word: sextillion/trilliard, adoption: 1991)
pub type Zetta<U> = MulPow10<U, U21>;
/// exa prefix. `E`. (Base 10: `10^18`, decimal: `1000000000000000000`, word: quintillion/trillion, adoption: 1975)
pub type Exa<U> = MulPow10<U, U18>;
/// peta prefix. `P`. (Base 10: `10^15`, decimal: `1000000000000000`, word: quadrillion/billiard, adoption: 1975)
pub type Peta<U> = MulPow10<U, U15>;
/// tera prefix. `T`. (Base 10: `10^12`, decimal: `1000000000000`, word: trillion/billion, adoption: 1960)
pub type Tera<U> = MulPow10<U, U12>;
/// giga prefix. `G`. (Base 10: `10^9`, decimal: `1000000000`, word: billion/milliard, adoption: 1960)
pub type Giga<U> = MulPow10<U, U9>;
/// mega prefix. `M`. (Base 10: `10^6`, decimal: `1000000`, word: million, adoption: 1873)
pub type Mega<U> = MulPow10<U, U6>;
/// kilo prefix. `k`. (Base 10: `10^3`, decimal: `1000`, word: thousand, adoption: 1795)
pub type Kilo<U> = MulPow10<U, U3>;
/// hecto prefix. `h`. (Base 10: `10^2`, decimal: `100`, word: hundred, adoption: 1795)
pub type Hecto<U> = MulPow10<U, U2>;
/// deca prefix. `da`. (Base 10: `10^1`, decimal: `10`, word: ten, adoption: 1795)
pub type Deca<U> = MulPow10<U, U1>;

/// deci prefix. `d`. (Base 10: `10^-1`, decimal: `0.1`, word: tenth, adoption: 1795)
pub type Deci<U> = DivPow10<U, U1>;
/// centi prefix. `c`. (Base 10: `10^-2`, decimal: `0.01`, word: hundredth, adoption: 1795)
pub type Centi<U> = DivPow10<U, U2>;
/// milli prefix. `m`. (Base 10: `10^-3`, decimal: `0.001`, word: thousandth, adoption: 1795)
pub type Milli<U> = DivPow10<U, U3>;
/// micro prefix. `μ`. (Base 10: `10^-6`, decimal: `0.000001`, word: millionth, adoption: 1873)
pub type Micro<U> = DivPow10<U, U6>;
/// nano prefix. `n`. (Base 10: `10^-9`, decimal: `0.000000001`, word: billionth/milliardth, adoption: 1960)
pub type Nano<U> = DivPow10<U, U9>;
/// pico prefix. `p`. (Base 10: `10^-12`, decimal: `0.000000000001`, word: trillionth/billionth, adoption: 1960)
pub type Pico<U> = DivPow10<U, U12>;
/// femto prefix. `f`. (Base 10: `10^-15`, decimal: `0.000000000000001`, word: quadrillionth/billiardth, adoption: 1964)
pub type Femto<U> = DivPow10<U, U15>;
/// atto prefix. `a`. (Base 10: `10^-18`, decimal: `0.000000000000000001`, word: quintillionth/trillionth, adoption: 1964)
pub type Atto<U> = DivPow10<U, U18>;
/// zepto prefix. `z`. (Base 10: `10^-21`, decimal: `0.000000000000000000001`, word: sextillionth/trilliardth, adoption: 1991)
pub type Zepto<U> = DivPow10<U, U21>;
/// yocto prefix. `y`. (Base 10: `10^-24`, decimal: `0.000000000000000000000001`, word: septillionth/quadrillionth, adoption: 1991)
pub type Yocto<U> = DivPow10<U, U24>;


/// Multiplies ratio of `U` by `X`
pub(crate) type MulBy<U, X> = Unit<
    <U as UnitTrait>::Length,
    <U as UnitTrait>::Mass,
    <U as UnitTrait>::Time,
    <U as UnitTrait>::ElectricCurrent,
    <U as UnitTrait>::ThermodynamicTemperature,
    <U as UnitTrait>::AmountOfSubstance,
    <U as UnitTrait>::LuminousIntensity,
    <<U as UnitTrait>::Ratio as Mul<Frac![X / U1]>>::Output,
>;

/// Divides ratio of `U` by `X`
pub(crate) type DivBy<U, X> = Unit<
    <U as UnitTrait>::Length,
    <U as UnitTrait>::Mass,
    <U as UnitTrait>::Time,
    <U as UnitTrait>::ElectricCurrent,
    <U as UnitTrait>::ThermodynamicTemperature,
    <U as UnitTrait>::AmountOfSubstance,
    <U as UnitTrait>::LuminousIntensity,
    <<U as UnitTrait>::Ratio as Div<Frac![X / U1]>>::Output,
>;
