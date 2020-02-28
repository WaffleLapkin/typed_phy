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
    // TODO: audit&document this macro

    // Start of @replace sub-macro.
    // It replaces *, / and ^ with {*}, {/} and {^}
    // and calls @prepare
    (@replace $( $t:tt )+) => {
        Unit![@replace_inner [] [ $( $t )+ ]]
    };
    (@replace_inner [ $( $head:tt )* ] [ * $( $tail:tt )* ]) => {
        Unit!(@replace_inner [ $( $head )* {*} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ / $( $tail:tt )* ]) => {
        Unit!(@replace_inner [ $( $head )* {/} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ ^ $( $tail:tt )* ]) => {
        Unit!(@replace_inner [ $( $head )* {^} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ $it:tt $( $tail:tt )* ]) => {
        Unit!(@replace_inner [ $( $head )* $it ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )+ ] [] ) => {
        Unit![@prepare $( $head )+ ]
    };

    // @prepare sub-macro
    (@prepare $( $tail:tt )+) => {
        Unit![$crate::units::Dimensionless => {*} $( $tail )+]
    };


    ($t:ty => {$op:tt} $a:ty {^} -$n:tt $( $tts:tt )*) => {
        Unit![@exp $t => {$op}; $a {^} -$n; $( $tts )*]
    };
    ($t:ty => {$op:tt} $a:ty {^} $n:tt $( $tts:tt )*) => {
        Unit![@exp $t => {$op}; $a {^} $n; $( $tts )*]
    };
    ($t:ty => {*} $a:ty $( {$next:tt} $( $tts:tt )+ )?) => {
        Unit![<$t as core::ops::Mul<$a>>::Output => $( {$next} $( $tts )+ )?]
    };
    ($t:ty => {/} $a:ty $( {$next:tt} $( $tts:tt )+ )?) => {
        Unit![<$t as core::ops::Div<$a>>::Output => $( {$next} $( $tts )+ )?]
    };

    // Start of @exp sub-macro
    (@exp $t:ty => {$op:tt}; $a:ty {^} 4; $( $tts:tt )*) => {
        Unit![$t => {$op} $a {$op} $a {$op} $a {$op} $a $( $tts )*]
    };
    (@exp $t:ty => {$op:tt}; $a:ty {^} 3; $( $tts:tt )*) => {
        Unit![$t => {$op} $a {$op} $a {$op} $a $( $tts )*]
    };
    (@exp $t:ty => {$op:tt}; $a:ty {^} 2; $( $tts:tt )*) => {
        Unit![$t => {$op} $a {$op} $a $( $tts )*]
    };
    (@exp $t:ty => {$op:tt}; $a:ty {^} 1; $( $tts:tt )*) => {
        Unit![$t => {$op} $a $( $tts )*]
    };
    (@exp $t:ty => {$op:tt}; $a:ty {^} 0; $( $tts:tt )*) => {
        Unit![$t => $( $tts )*]
    };
    (@exp $t:ty => {/} ; $a:ty {^} -$lit:tt; $( $tts:tt )*) => {
        Unit![@exp $t => {*} ; $a {^} $lit; $( $tts )*]
    };
    (@exp $t:ty => {*} ; $a:ty {^} -$lit:tt; $( $tts:tt )*) => {
        Unit![@exp $t => {/} ; $a {^} $lit; $( $tts )*]
    };
    (@exp $t:ty => {$op:tt} ; $a:ty {^} $lit:tt; $( $tts:tt )*) => {
        compiler_error!("Only exponents from -4 to 4 are supported")
    };

    // End
    ($t:ty =>) => {
        $t
    };

    // Empty call = dimensionless
    () => {
        $crate::units::Dimensionless
    };

    // Early start (user of the method should call this branch)
    // Calls @replace sub-macro
    ($( $anything:tt )+) => {
        Unit![@replace $($anything)+]
    }
}

#[test]
fn unit() {
    use core::ops::Mul;

    use typenum::{N1, P1, U100, U1000, U36, Z0};

    use crate::{
        fraction::Fraction,
        prefixes::Kilo,
        units::{Dimensionless, Hour, KiloGram, Metre, Second, Watt},
        Dimensions, IntExt, Quantity, Unit,
    };

    type U3600 = <U36 as Mul<U100>>::Output;

    typenum::assert_type_eq!(
        Unit![Kilo<Metre> / Hour],
        Unit<Dimensions<P1, Z0, N1, Z0, Z0, Z0, Z0>, Fraction<U1000, U3600>>
    );

    typenum::assert_type_eq!(Unit![], Dimensionless);

    // was broken in first version of the Unit! macro with types support
    #[allow(clippy::type_complexity)]
    let _: Quantity<_, Unit![KiloGram * Metre ^ 2 * Second ^ -3]> = 1.quantity::<Watt>();
    // TODO: more tests
}

/// Shortcut for creating [`Fraction`], see it's doc for more.
///
/// [`Fraction`]: crate::fraction::Fraction
#[macro_export]
#[allow(non_snake_case)]
macro_rules! Frac {
    ($a:ident / $b:ty) => {
        $crate::fraction::Fraction::<$a, $b>
    };
    (/ $b:ty) => {
        $crate::fraction::Fraction::<typenum::U1, $b>
        //                           ^^^^^^^^^^^ TODO: crate reexport
    };
    ($a:ty) => {
        $crate::fraction::Fraction::<$a, typenum::U1>
        //                              ^^^^^^^^^^^ TODO: crate reexport
    };
}
