/// Declare (?) [`Unit`] _type_ depending on other units.
///
/// ## Examples
/// ```
/// use typed_phy::{
///     units::{Metre, MetrePerSecond, Second},
///     Id, Unit,
/// };
///
/// /// Assert that types `A` and `B` are equal
/// fn type_eq<A: Id<This = B>, B>() {}
///
/// /// mps = m/s = m¹ * s⁻¹
/// type MPS = Unit![Metre / Second];
/// type_eq::<MPS, MetrePerSecond>();
///
/// /// still simplifies to m/s
/// type MPS_ = Unit![Metre / Second * Second / Second * MetrePerSecond / MetrePerSecond];
/// type_eq::<MPS_, MetrePerSecond>();
/// ```
/// ```
/// # use typed_phy::{Unit, Quantity, IntExt, units::{Metre, Second}};
/// let _: Quantity<_, Unit![Metre / Second]> = 10.mps();
/// ```
/// ```
/// # use typed_phy::{Quantity, IntExt, Unit, units::{Metre, KiloGram, Second}};
/// use typed_phy::units::Watt;
/// // Exponents [-4; 4] are supported
/// let _: Quantity<_, Unit![KiloGram * Metre ^ 2 * Second ^ -3]> = 1.quantity::<Watt>();
/// let _: Quantity<_, Unit![KiloGram * Metre ^ 2 / Second ^ 3]> = 1.quantity::<Watt>();
/// let _: Quantity<_, Unit![Metre ^ 2]> = 10.sqm();
/// let _: Quantity<_, Unit![Metre ^ 4]> = 10.sqm() * 10.sqm();
/// let _: Quantity<_, Unit![Metre ^ -4]> = 1.dimensionless() / 10.sqm() / 10.sqm();
/// ```
///
/// [`Unit`]: struct@crate::Unit
#[macro_export]
#[allow(non_snake_case)]
macro_rules! Unit {
    ($t:ty => $op:tt $a:ident ^ -$n:tt $( $tts:tt )*) => {
        Unit![@exp $t => $op; $a ^ -$n; $( $tts )*]
    };
    ($t:ty => $op:tt $a:ident ^ $n:tt $( $tts:tt )*) => {
        Unit![@exp $t => $op; $a ^ $n; $( $tts )*]
    };
    ($t:ty => * $a:ident $( $tts:tt )*) => {
        Unit![<$t as core::ops::Mul<$a>>::Output => $( $tts )*]
    };
    ($t:ty => / $a:ident $( $tts:tt )*) => {
        Unit![<$t as core::ops::Div<$a>>::Output => $( $tts )*]
    };
    ($t:ident $( $tts:tt )*) => {
        Unit![$crate::units::Dimensionless => * $t $( $tts )*]
    };
    ($t:ty =>) => {
        $t
    };
    (@exp $t:ty => $op:tt; $a:ident ^ 4; $( $tts:tt )*) => {
        Unit![$t => $op $a $op $a $op $a $op $a $( $tts )*]
    };
    (@exp $t:ty => $op:tt; $a:ident ^ 3; $( $tts:tt )*) => {
        Unit![$t => $op $a $op $a $op $a $( $tts )*]
    };
    (@exp $t:ty => $op:tt; $a:ident ^ 2; $( $tts:tt )*) => {
        Unit![$t => $op $a $op $a $( $tts )*]
    };
    (@exp $t:ty => $op:tt; $a:ident ^ 1; $( $tts:tt )*) => {
        Unit![$t => $op $a $( $tts )*]
    };
    (@exp $t:ty => $op:tt; $a:ident ^ 0; $( $tts:tt )*) => {
        Unit![$t => $( $tts )*]
    };
    (@exp $t:ty => / ; $a:ident ^ -$lit:tt; $( $tts:tt )*) => {
        Unit![@exp $t => * ; $a ^ $lit; $( $tts )*]
    };
    (@exp $t:ty => * ; $a:ident ^ -$lit:tt; $( $tts:tt )*) => {
        Unit![@exp $t => / ; $a ^ $lit; $( $tts )*]
    };
}
