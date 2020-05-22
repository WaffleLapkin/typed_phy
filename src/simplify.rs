use core::ops::Div;

use crate::{fraction::Fraction, gcd::Gcd, Quantity, Unit};
use typenum::Quot;

/// Simplify fraction.
///
/// ## Examples
/// ```
/// use typed_phy::{fraction::Fraction, simplify::Simplify};
///
/// use typenum::{assert_type_eq, U12, U3, U32, U8};
///
/// type Simplified = <Fraction<U32, U12> as Simplify>::Output;
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
    #[allow(clippy::type_complexity)]
    type Output = Fraction<Quot<N, <N as Gcd<D>>::Output>, Quot<D, <N as Gcd<D>>::Output>>;

    #[inline]
    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}

impl<D, R> Simplify for Unit<D, R>
where
    R: Simplify,
{
    type Output = Unit<D, R::Output>;

    #[inline]
    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}

impl<S, U> Simplify for Quantity<S, U>
where
    U: Simplify,
{
    type Output = Quantity<S, U::Output>;

    #[inline]
    fn simplify(self) -> Self::Output {
        self.set_unit_unchecked()
    }
}
