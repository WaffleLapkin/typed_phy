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
    // To achieve this we will scan the input token by token and push them onto "queue".
    // Then, if we have type, operation (either `*` or `/`) and another type, we will call `ty_op`
    // macro to do the operation.
    // Then, if we doesn't have any token left anymore and we have the only token in your queue,
    // that is the result.
    //
    // Let's go on to an example. Let's say we have Unit![A * B / C] and go through it step by step:
    //
    // 1. start-branch "creates" the queue and calls the execution sub-command:
    //    `Unit![@exec []  A * B / C]`
    //          /^^^^^ ^^\ ^^^^^^^^^ ---- "the rest" - tokens we haven't handled yet
    // "sub-command"      \
    //                     * ---- "the queue"
    //
    // 2. the @exec sub-command tries to pop a type[^1] from "the rest" and push it onto the queue:
    //    `Unit![@exec [A] * B / C]`
    //
    // 3. the @exec sub-command tries to pop an operation from "the rest" and push it onto the queue:
    //    `Unit![@exec [A {*}] B / C]`
    //                    ^^^\
    //                        * Note: that we can't parse `tt` (`*`, `/`, etc) after `ty` fragment,
    //                          so we need to somehow escape the operation
    //
    // 4. same as 2 - pop type, push type:
    //    `Unit![@exec [A {*} B] / C]`
    //
    // 5. the queue has "type, operation and another type" so the call `ty_op`:
    //    `Unit![@exec [ty_op!(A {*} B)] / C]`
    //
    // 6. `ty_op` does the operation:
    //    `Unit![@exec [<A as Mul<B>>::Output] / C]`
    //                        ^^^\
    //                            * For the sale of simplicity the full paths are omitted
    //
    // 7. the same goes further:
    //    `Unit![@exec [<A as Mul<B>>::Output {/}] C]`
    //
    // 8. `Unit![@exec [<A as Mul<B>>::Output {/} C]]`
    // 9. `Unit![@exec [ty_op!(<A as Mul<B>>::Output {/} C)]]`
    // 10. `Unit![@exec [<<A as Mul<B>>::Output as Div<C>>::Output]]`
    // 11. The queue has the only type and "the rest" is empty, yay! We did it! Whe result branch
    //     returns the type:
    //     `<<A as Mul<B>>::Output as Div<C>>::Output`
    //
    // The things that were intentionally omitted:
    // - how we parse the types ([^1])
    // - full paths (like `$crate::Unit` or `core::ops::Mul`)
    // - some brackets those were added while trying to fix the macro (maybe they are usefull,
    //   though the don't change much and I hope to remove them)
    // - the order of expansion - in the real world `Unit` will fully expand firstly and only then
    //   `ty_op`(s) will expand.
    //
    // [^1]: because of macro-by-example limitations we can't do exactly this,
    //       but we'll cover this later TODO when "later"



    // `@exec` (execute) sub-command.
    //
    // This sub-command does the most of the macro's work what it does is quite well explained in
    // the "## Basic Idea" paragraph. But here are some additional details.

    // Those branches should be simpler (they are essentially one), but `tt` can go after `ty`,
    // so instead of:
    // ```(@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $t:ty $( $rest:tt )* )```
    // We have those 8 branches. Why 8? Well, there are a lot of ways to write a type in rust:
    // 1) `Type` or `path::Type`[^2]
    // 2)  `Type<...>` or `path::Type<...>`
    // 3) `<Ty as Tr>::Assoc` or `<Ty as Tr<...>>::Assoc`
    // 4) `macro![...]`
    // 5) `macro!(...)`
    // 6) `macro! { ... }`
    //
    // [^2]:
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $new_ty_name:ident $( :: $new_ty_path:ident )* <$new_ty_gen:ty $(, $new_ty_gens:ty )* $(,)?> $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($new_ty_name $( :: $new_ty_path )* <$new_ty_gen $(, $new_ty_gens )*>)] $( $rest )* ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?] <$s:ty as $Trait:ident $( :: $trait_path:ident )* $( <$trait_gen:ty $(, $trait_gens:ty )* $(,)?> )? >::$assoc:ident $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? (<$s as $Trait $( :: $trait_path )* $( <$trait_gen $(, $trait_gens )* $(,)?> )? >::$assoc)] $( $rest )* ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $macro:ident $( :: $macro_path:ident )* !( $( $args:tt )* ) $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($macro $( :: $macro_path )*!( $( $args )* ))] $( $rest )* ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $macro:ident $( :: $macro_path:ident )* ![ $( $args:tt )* ] $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($macro $( :: $macro_path )*![ $( $args )* ])] $( $rest )* ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?] $macro:ident $( :: $macro_path:ident )* !{ $( $args:tt )* } $( $rest:tt )* ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($macro $( :: $macro_path )*!{ $( $args )* })] $( $rest )* ]
    };

    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?]  $new_ty_name:ident $( :: $new_ty_path:ident )* $( * $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($new_ty_name $( :: $new_ty_path )*) ] $( * $( $rest )+ )? ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?]  $new_ty_name:ident $( :: $new_ty_path:ident )* $( / $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($new_ty_name $( :: $new_ty_path )*) ] $( / $( $rest )+ )? ]
    };
    (@exec [ $( ($s_ty:ty) {$s_op:tt} )?]  $new_ty_name:ident $( :: $new_ty_path:ident )* $( ^ $( $rest:tt )+ )? ) => {
        $crate::Unit![@exec [ $( ($s_ty) {$s_op} )? ($new_ty_name $( :: $new_ty_path )*) ] $( ^ $( $rest )+ )? ]
    };

    (@exec [ ($a_ty:ty) {$op:tt} ($b_ty:ty) ] ^ -$n:tt $( $rest:tt )* ) => {
        $crate::Unit![@exec [ ($a_ty) {$op} ($crate::Unit![@exp $b_ty {^} -$n]) ] $( $rest )* ]
    };
    (@exec [ ($a_ty:ty) {$op:tt} ($b_ty:ty) ] ^ $n:tt $( $rest:tt )* ) => {
        $crate::Unit![@exec [ ($a_ty) {$op} ($crate::Unit![@exp $b_ty {^} $n]) ] $( $rest )* ]
    };
    (@exec [ ($b_ty:ty) ] ^ -$n:tt $( $rest:tt )* ) => {
        $crate::Unit![@exec [ ($crate::Unit![@exp $b_ty {^} -$n]) ] $( $rest )* ]
    };
    (@exec [ ($b_ty:ty) ] ^ $n:tt $( $rest:tt )* ) => {
        $crate::Unit![@exec [ ($crate::Unit![@exp $b_ty {^} $n]) ] $( $rest )* ]
    };

    (@exec [ ($s_ty:ty) ] $new_op:tt $( $rest:tt )* ) => {
        $crate::Unit![@exec [ ($s_ty) {$new_op} ] $( $rest )* ]
    };

    (@exec [ ($a_ty:ty) {*} ($b_ty:ty) ] $( $rest:tt )* ) => {
        $crate::Unit![@exec [ (<$a_ty as core::ops::Mul<$b_ty>>::Output) ] $( $rest )* ]
    };
    (@exec [ ($a_ty:ty) {/} ($b_ty:ty) ] $( $rest:tt )* ) => {
        $crate::Unit![@exec [ (<$a_ty as core::ops::Div<$b_ty>>::Output) ] $( $rest )* ]
    };
    (@exec [ ($a_ty:ty) {$op:tt} ($b_ty:ty) ] $( $rest:tt )* ) => {
        // TODO: unsupported operation error
        compile_error!(stringify!($op))
    };


    (@exec [ $res:ty ] ) => {
        $res
    };

    // END OF `@exec` sub-command

    (@exp $a_ty:ty {^} $( - )? 0) => {
        $crate::Unit![$a_ty / $a_ty]
    };
    (@exp $a_ty:ty {^} -$n:tt) => {
        $crate::Unit![$crate::Unit![@exp_inner $a_ty {/}^ $n] / $crate::Unit!(@untype $a_ty) / $crate::Unit!(@untype $a_ty)]
    };
    (@exp $a_ty:ty {^} $n:tt) => {
        $crate::Unit![@exp_inner $a_ty {*}^ $n]
    };
    (@exp_inner $a_ty:ty {$op:tt}^ 1) => {
        $a_ty
    };
    (@exp_inner $a_ty:ty {$op:tt}^ 2) => {
        $crate::Unit![$crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty)]
    };
    (@exp_inner $a_ty:ty {$op:tt}^ 3) => {
        $crate::Unit![$crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty)]
    };
    (@exp_inner $a_ty:ty {$op:tt}^ 4) => {
        $crate::Unit![$crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty) $op $crate::Unit!(@untype $a_ty)]
    };
    (@exp_inner $a_ty:ty {$op:tt}^ $n:tt) => {
        compile_error!(
            concat!(
                "Expected exponent number in bounds [-4; 4], found `",
                stringify!($n),
                "`. Note: exponents greater that 4 or less than -4 are not currently supported"
            )
        )
    };

    (@untype $( $tts:tt )*) => {
        $( $tts )*
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
        $crate::Unit![@exec [] $($anything)+]
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
