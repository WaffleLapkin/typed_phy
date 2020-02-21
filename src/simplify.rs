use core::ops::Div;

use crate::{
    gcd::Gcd,
    fraction::Fraction,
    Unit
};

/// Simplify fraction.
///
/// ## Examples
/// ```
/// use typed_phy::{
///     fraction::Fraction,
///     simplify::Simplify,
/// };
///
/// use typenum::{U3, U8, U12, U32, assert_type_eq};
///
/// type Simplified = <Fraction::<U32, U12> as Simplify>::Output;
/// assert_type_eq!(Simplified, Fraction::<U8, U3>);
/// ```
pub trait Simplify {
    /// Result of the simplification
    type Output;

    /// Simplify fraction
    fn simplify(self) -> Self::Output;
}

impl<N, D> Simplify for Fraction<N, D>
where
    N: Gcd<D>,
    N: Div<<N as Gcd<D>>::Output>,
    D: Div<<N as Gcd<D>>::Output>,
{
    type Output = Fraction<
        <N as Div<<N as Gcd<D>>::Output>>::Output,
        <D as Div<<N as Gcd<D>>::Output>>::Output,
    >;

    #[inline]
    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}
