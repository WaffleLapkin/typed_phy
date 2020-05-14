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
    // Are you sure you want to go through that hell of declarative macros?
    // Are you sure you want to see how I've reinvented the wheel (because decl-macroses in rust
    // are... terrible, when you want to do something not-so-simple)?
    //
    // If you don't really need this, please, go away and save save your mind from this horror
    //
    // No, really, you still have a chance to turn back.
    //
    // Well, I've warned you.

    // ## Basic idea
    //
    // This macro should turn `A * B` into `<A as core::ops::Mul<B>>::Output`, `A ^ 2` into
    // `<A as Mul<A>>::Output`, `A / B` into `<A as Mul<B>>::Output` and so on.
    //
    // To achieve this we will use something like `fold` - we will have an `$acc` macro "variable",
    // scan the input for operation (either `*` or `/`, op for short) and type (ty for short) and
    // push it onto "last ty stack" (you'll see later why we need this), then, if we have `$op` and
    // `$ty` on the stack we will apply the `$op` to `$acc` and `$ty` using `@ty_op` sub-command.
    //
    // When we'll scan the entire input, we will return the `$acc`.
    //
    // Let's go on to an example.
    // Let's say we have `Unit![A * B / C]` and go through it step by step:
    //
    // 1. start-branch "creates" the queue and calls the execution sub-command:
    //          the stack --- *.      .* --- to have starting `$acc` we've used `NoOpMul` that does
    //                          \__ _/                     nothing when multiplied and added mul op.
    //    `Unit![@exec [NoOpMul] [] * A * B / C]`
    //          /^^^^^ ^^^^^^^^^\   ^^^^^^^^^^^ ---- "the rest" - tokens we haven't handled yet
    // "sub-command"             \
    //                            * ---- `$acc`
    //
    // 2. the @exec sub-command tries to pop an op and a ty[^1]
    //    from "the rest" and push it onto the stack:
    //    `Unit![@exec [NoOpMul] [* A] * B / C]`
    //
    // 3. the @exec sub-command tries to yet again pop an op and a ty[^1] from "the rest" and push
    //    it onto the stack, but since we already have op+ty pair on stack, we apply the `$op` to the
    //    `$acc` and `$ty`:
    //    `Unit![@exec [Unit![@ty_op NoOpMul {*} A]] [* B] / C]`
    //                                       ^^^\
    //                                          /\
    //                                         Note:
    //                     we can't parse `tt` (`*`, `/`, etc) after `ty` fragment,
    //                          so we need to somehow escape the operation
    //
    // 4. `@ty_op` sub-command expands to the type operation:
    //    `Unit![@exec [<NoOpMul as Mul<A>>::Output] [* B] / C]`
    //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
    //                                              Note: `for<T> NoOpMul: Mul<T, Output = T>`,
    //                                                    so later we'll just replace this by A
    //                                                    for the sake of simplicity
    //
    // 5. next step (same as previous):
    //    `Unit![@exec [Unit![@ty_op A {*} B]] [/ C]]`
    //
    // 6. `ty_op` yet again does the operation:
    //    `Unit![@exec [<A as Mul<B>>::Output] [/ C]]`
    //
    // 7. we now have only `$acc` and stack, no more unfolded tokens left, so we do the last
    //    operation and "return" the result:
    //    `Unit![@ty_op <A as Mul<B>>::Output {/} C]`
    //    `<<A as Mul<B>>::Output as Div<C>>::Output`
    //
    // The things that were intentionally omitted:
    // - how we parse the types ([^1])
    // - full paths (like `$crate::Unit` or `core::ops::Mul`)
    // - the order of expansion - in the real world `Unit` will fully expand firstly and only then
    //   `@ty_op`(s) will expand.
    //
    // [^1]: because of macro-by-example limitations we can't do exactly this,
    //       but we'll cover this later

    // Shorthand. `Unit![A]` = `A`
    ($ty:ty) => {
        $ty
    };

    // Shorthand. `Unit![]` = `Dimensionless`
    () => {
        $crate::units::Dimensionless
    };

    // `@exec` (execute) sub-command.
    //
    // This sub-command does the most of the macro's work what it does is quite well explained in
    // the "## Basic Idea" paragraph. But here are some additional details.

    // Those next 7 branches expand exponents
    // 1) expand `* X ^ -n` => `/ X ^ n`
    // 2) expand `/ X ^ -n` => `* X ^ n`
    // 3..6) expand `$op ^ n` for n 1, 2, 3, 4
    // 7) compile error for exponents > 4
    (@exec [ $acc:ty ] [* $x:ty] ^ -$n:tt $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $acc ] [/ $x] ^ $n $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [/ $x:ty] ^ -$n:tt $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $acc ] [* $x] ^ $n $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [$op:tt $x:ty] ^ 1 $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit!(@ty_op $acc {$op} $x) ] [] $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [$op:tt $x:ty] ^ 2 $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit!(@ty_op $crate::Unit!(@ty_op $acc {$op} $x) {$op} $x) ] [] $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [$op:tt $x:ty] ^ 3 $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit!(@ty_op $crate::Unit!(@ty_op $crate::Unit!(@ty_op $acc {$op} $x) {$op} $x) {$op} $x) ] [] $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [$op:tt $x:ty] ^ 4 $( $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit!(@ty_op $crate::Unit!(@ty_op $crate::Unit!(@ty_op $crate::Unit!(@ty_op $acc {$op} $x) {$op} $x) {$op} $x) {$op} $x) ] [] $( $( $rest )+ )? ]
    };
    (@exec [ $acc:ty ] [$op:tt $x:ty] ^ $n:tt $( $( $rest:tt )+ )? ) => {
        compile_error!(
            concat!(
                "Expected exponent number in bounds [-4; 4], found `",
                stringify!($n),
                "`. Note: exponents greater that 4 or less than -4 are not currently supported"
            )
        )
    };

    // Those branches should be simpler (they are essentially one), but `tt` can't go after `ty`,
    // so instead of:
    // ```(@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $t:ty $( $rest:tt )* )```
    // We have those 8 branches. Why 8? Well, there are a lot of ways to write a type in rust:
    // 1)  `Type<...>` or `path::Type<...>`
    // 2) `<Ty as Tr>::Assoc` or `<Ty as Tr<...>>::Assoc` or`<Ty as path::Tr>::Assoc` or `<Ty as path::Tr<...>>::Assoc`
    // 3) `macro!(...)` or `path::macro!(...)`
    // 4) `macro![...]` or `path::macro![...]`
    // 5) `macro! { ... }` or `path::macro! { ... }`
    // 6..8) `Type` or `path::Type`
    (/* 1 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $new_ty_name:ident $( :: $new_ty_path:ident )* <$new_ty_gen:ty $(, $new_ty_gens:ty )* $(,)?> $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $new_ty_name $( :: $new_ty_path )* <$new_ty_gen $(, $new_ty_gens )*> ] $( $rest )* ]
    };
    (/* 2 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt <$s:ty as $Trait:ident $( :: $trait_path:ident )* $( <$trait_gen:ty $(, $trait_gens:ty )* $(,)?> )? >::$assoc:ident $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op <$s as $Trait $( :: $trait_path )* $( <$trait_gen $(, $trait_gens )* $(,)?> )? >::$assoc ] $( $rest )* ]
    };

    (/* 3 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $macro:ident $( :: $macro_path:ident )* !( $( $args:tt )* ) $( $rest:tt )*  ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $macro $( :: $macro_path )*!( $( $args )* ) ] $( $rest )* ]
    };
    (/* 4 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $macro:ident $( :: $macro_path:ident )* ![ $( $args:tt )* ] $( $rest:tt )*  ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $macro $( :: $macro_path )*![ $( $args )* ] ] $( $rest )* ]
    };
    (/* 5 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $macro:ident $( :: $macro_path:ident )* !{ $( $args:tt )* } $( $rest:tt )*  ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $macro $( :: $macro_path )*!{ $( $args )* } ] $( $rest )* ]
    };

    (/* 6 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $new_ty_name:ident $( :: $new_ty_path:ident )* $( * $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $new_ty_name $( :: $new_ty_path )* ] $( * $( $rest )+ )? ]
    };
    (/* 7 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $new_ty_name:ident $( :: $new_ty_path:ident )* $( / $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $new_ty_name $( :: $new_ty_path )* ] $( / $( $rest )+ )? ]
    };
    (/* 8 */ @exec [ $acc:ty ] [ $( $op:tt $prev:ty )? ] $x_op:tt $new_ty_name:ident $( :: $new_ty_path:ident )* $( ^ $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $crate::Unit![@ty_op $acc $( {$op} $prev )?] ] [$x_op $new_ty_name $( :: $new_ty_path )* ] $( ^ $( $rest )+ )? ]
    };

    // The work is done, return the result
    (@exec [ $res:ty ] [] ) => {
        $res
    };
    // Do the last operation and return the result
    (@exec [ $acc:ty ] [$op:tt $last:ty] ) => {
        $crate::Unit![@ty_op $acc {$op} $last]
    };

    // `@ty_op` (type operation) sub-command
    //
    //
    (@ty_op $a:ty) => {
        $a
    };
    (@ty_op $a:ty {*} $b:ty) => {
        <$a as core::ops::Mul<$b>>::Output
    };
    (@ty_op $a:ty {/} $b:ty) => {
        <$a as core::ops::Div<$b>>::Output
    };
    // Error on unknown operation with (at least a bit) readable error message
    (@ty_op $a:ty {$op:tt} $b:ty) => {
        compile_error!(
            concat!(
                "Expected one of supported operations (`*`, `/`), found: `",
                stringify!($op),
                "`"
            )
        )
    };

    // Unknown command (this also prevents infinite recursion when because of a typo/bug some
    // command doesn't handle input)
    (@ $( $anything:tt )*) => {
        compile_error!(
            concat!(
                "Expected type, found `@`. This is caused either by \
calling `typed_phy::Unit` with `@` at the start (instead of a type) or by the \
bug in the macro. In the second case please open an issue on github. Input: `",
                stringify!(@ $( $anything )*),
                "`"
            )
        )
    };

    // Early start (user of the method should call this branch)
    // Calls @replace sub-macro
    ($( $anything:tt )+) => {
        $crate::Unit![@exec [$crate::NoOpMul] [] * $($anything)+]
    };
}

/// Helper for `Unit` macro
///
/// This stru^W enum is needed to do things in a more generic way.
/// (so you always have a type to start from)
#[doc(hidden)]
pub enum NoOpMul {}

impl<T> core::ops::Mul<T> for NoOpMul {
    type Output = T;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        rhs
    }
}

// Only used in `Unit![X ^ -n]`
impl<T> core::ops::Div<T> for NoOpMul
where
    crate::units::Dimensionless: core::ops::Div<T>,
{
    type Output = <crate::units::Dimensionless as core::ops::Div<T>>::Output;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        crate::units::Dimensionless::new() / rhs
    }
}

#[test]
fn unit() {
    macro_rules! id {
        ($t:ty) => {
            $t
        };
    }

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

    type Simple = Unit![
        Kilo<Metre> ^ 2
        / Second
        * <Second as crate::Id>::This ^ 1
        * id!(KiloGram) ^ 2
        * id![KiloGram] ^ 2
        / id!{KiloGram} ^ 2
        * crate::units::KiloGram
        / crate::units::KiloGram
        * crate::units::KiloGram ^ -2
        / crate::prefixes::Kilo<Metre> ^ 2
    ];

    typenum::assert_type_eq!(<Simple as crate::simplify::Simplify>::Output, Dimensionless);
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
