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
        $crate::Unit![@replace_inner [] [ $( $t )+ ]]
    };
    (@replace_inner [ $( $head:tt )* ] [ * $( $tail:tt )* ]) => {
        $crate::Unit!(@replace_inner [ $( $head )* {*} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ / $( $tail:tt )* ]) => {
        $crate::Unit!(@replace_inner [ $( $head )* {/} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ ^ $( $tail:tt )* ]) => {
        $crate::Unit!(@replace_inner [ $( $head )* {^} ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )* ] [ $it:tt $( $tail:tt )* ]) => {
        $crate::Unit!(@replace_inner [ $( $head )* $it ] [ $( $tail )* ])
    };
    (@replace_inner [ $( $head:tt )+ ] [] ) => {
        $crate::Unit![@prepare [] [{*} $( $head )+] ]
    };


    (@prepare [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} -$n:tt $( $tail:tt )*]) => {
        $crate::Unit![@exp [$( {$p_op} $p_t , )*] [{$op} $t {^} -$n] [ $( $tail )* ] ]
    };
    (@prepare [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} $n:tt $( $tail:tt )*]) => {
        $crate::Unit![@exp [$( {$p_op} $p_t , )*] [{$op} $t {^} $n] [ $( $tail )* ] ]
    };
    (@prepare [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty $( {$next_op:tt} $( $tail:tt )+ )?]) => {
        $crate::Unit![@prepare [$( {$p_op} $p_t , )* {$op} $t, ] [ $( {$next_op} $( $tail )+ )?] ]
    };

    (@prepare [ {*} $t:ty, $( {$t_op:tt} $t_t:ty , )* ] []) => {
        $crate::Unit![@exec $t $( {$t_op} $t_t )* ]
    };

    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{*} $t:ty {^} -$n:tt] [ $( $tail:tt )* ]) => {
        $crate::Unit![@exp [ $( {$p_op} $p_t , )* ] [{/} $t {^} $n] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{/} $t:ty {^} -$n:tt] [ $( $tail:tt )* ]) => {
        $crate::Unit![@exp [ $( {$p_op} $p_t , )* ] [{*} $t:ty {^} $n] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} 1] [ $( $tail:tt )* ]) => {
        $crate::Unit![@prepare [ $( {$p_op} $p_t , )* {$op} $t, ] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} 2] [ $( $tail:tt )* ]) => {
        $crate::Unit![@prepare [ $( {$p_op} $p_t , )* {$op} $t, {$op} $t, ] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} 3] [ $( $tail:tt )* ]) => {
        $crate::Unit![@prepare [ $( {$p_op} $p_t , )* {$op} $t, {$op} $t, {$op} $t, ] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} 4] [ $( $tail:tt )* ]) => {
        $crate::Unit![@prepare [ $( {$p_op} $p_t , )* {$op} $t, {$op} $t, {$op} $t, {$op} $t, ] [ $( $tail )* ] ]
    };
    (@exp [ $( {$p_op:tt} $p_t:ty , )* ] [{$op:tt} $t:ty {^} $n:tt] [ $( $tail:tt )* ]) => {
        compile_error!(concat!("Expected exponent number in bounds [-4; 4], found `", stringify!($n), "`"));
    };

    (@exec $a:ty {*} $b:ty $( {$next_op:tt} $( $tail:tt )+ )?) => {
        $crate::Unit![<$a as core::ops::Mul<$b>>::Output $( {$next_op} $( $tail )+ )?]
    };
    (@exec $a:ty {/} $b:ty $( {$next_op:tt} $( $tail:tt )+ )?) => {
        $crate::Unit![<$a as core::ops::Div<$b>>::Output $( {$next_op} $( $tail )+ )?]
    };

    // End
    (@exec $res:ty) => {
        $res
    };

    // Empty call = dimensionless
    () => {
        $crate::units::Dimensionless
    };

    // Unknown command
    (@ $( $anything:tt )*) => {
        compile_error!(concat!("Expected type, found `@`. This is caused either by \
calling `typed_phy::Unit` with `@` at the start (instead of a type) or by the \
bug in the macro. In the second case please open an issue on github. Input: ", stringify!(@ $( $anything )*)))
    };

    // Early start (user of the method should call this branch)
    // Calls @replace sub-macro
    ($( $anything:tt )+) => {
        $crate::Unit![@replace $($anything)+]
    };
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
        $crate::fraction::Fraction::<$crate::reexport::U1, $b>
    };
    ($a:ty) => {
        $crate::fraction::Fraction::<$a, $crate::reexport::U1>
    };
}
