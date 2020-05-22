use core::ops::Div;

use typenum::{Gcd, Gcf, Quot};

use crate::{fraction::Fraction, Quantity, Unit};

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
    N: Div<Gcf<N, D>>,
    D: Div<Gcf<N, D>>,
{
    #[allow(clippy::type_complexity)]
    type Output = Fraction<Quot<N, Gcf<N, D>>, Quot<D, Gcf<N, D>>>;

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
