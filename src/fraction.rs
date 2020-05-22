use core::{
    fmt::{self, Write},
    marker::PhantomData,
    ops::{Div, Mul},
};

use typenum::{Prod, UInt, Unsigned, U0, U1};

use crate::{eq::FractionEq, from_int::FromUnsigned};

/// Fraction `Numerator / Denominator`
pub struct Fraction<Numerator, Denominator>(PhantomData<(Numerator, Denominator)>);

/// Default fraction. `1/1`
pub type One = Fraction<U1, U1>;

impl<N, D> Fraction<N, D> {
    /// Create new fraction
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<N, D> Default for Fraction<N, D> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

///
pub trait FractionTrait {
    ///
    type Numerator: Unsigned;

    ///
    type Divisor: Unsigned;

    /// Multiply integer by this fraction
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{fraction::FractionTrait, Frac};
    /// use typenum::{U5, U7};
    ///
    /// assert_eq!(<Frac![U5 / U7]>::mul(14), 10)
    /// ```
    #[inline]
    fn mul<I>(int: I) -> I
    where
        I: FromUnsigned + Mul<Output = I> + Div<Output = I>,
    {
        int * I::from_unsigned::<Self::Numerator>() / I::from_unsigned::<Self::Divisor>()
    }

    /// Divide integer by this fraction
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{fraction::FractionTrait, Frac};
    /// use typenum::{U5, U7};
    ///
    /// assert_eq!(<Frac![U5 / U7]>::div(10), 14)
    /// ```
    #[inline]
    fn div<I>(int: I) -> I
    where
        I: FromUnsigned + Mul<Output = I> + Div<Output = I>,
    {
        int * I::from_unsigned::<Self::Divisor>() / I::from_unsigned::<Self::Numerator>()
    }
}

impl<N, D> FractionTrait for Fraction<N, D>
where
    N: Unsigned,
    D: Unsigned,
{
    type Divisor = D;
    type Numerator = N;
}

/// `(n/d) / x = n/(d * x)`
impl<N, D, X, B> Div<UInt<X, B>> for Fraction<N, D>
where
    D: Mul<UInt<X, B>>,
{
    type Output = Fraction<N, Prod<D, UInt<X, B>>>;

    #[inline]
    fn div(self, _rhs: UInt<X, B>) -> Self::Output {
        Self::Output::new()
    }
}

/// `(n/d) * x = (n * x)/d`
impl<N, D, X, B> Mul<UInt<X, B>> for Fraction<N, D>
where
    N: Mul<UInt<X, B>>,
{
    type Output = Fraction<Prod<N, UInt<X, B>>, D>;

    #[inline]
    fn mul(self, _rhs: UInt<X, B>) -> Self::Output {
        Self::Output::new()
    }
}

/// `(n/d) * x = (n * x)/d`
impl<N, D> Mul<U0> for Fraction<N, D>
where
    N: Mul<U0>,
{
    type Output = Fraction<Prod<N, U0>, D>;

    #[inline]
    fn mul(self, _rhs: U0) -> Self::Output {
        Self::Output::new()
    }
}

/// `(n/d) / (a/b) = (n * b)/(d * a)`
impl<N, D, A, B> Div<Fraction<A, B>> for Fraction<N, D>
where
    N: Mul<B>,
    D: Mul<A>,
{
    type Output = Fraction<Prod<N, B>, Prod<D, A>>;

    #[inline]
    fn div(self, _rhs: Fraction<A, B>) -> Self::Output {
        Self::Output::new()
    }
}

/// `(n/d) * (a/b) = (n * a)/(d * b)`
impl<N, D, A, B> Mul<Fraction<A, B>> for Fraction<N, D>
where
    N: Mul<A>,
    D: Mul<B>,
{
    type Output = Fraction<Prod<N, A>, Prod<D, B>>;

    #[inline]
    fn mul(self, _rhs: Fraction<A, B>) -> Self::Output {
        Self::Output::new()
    }
}

impl<N, D, A, B> PartialEq<Fraction<A, B>> for Fraction<N, D>
where
    Self: FractionEq<Fraction<A, B>>,
{
    #[inline]
    fn eq(&self, _other: &Fraction<A, B>) -> bool {
        true
    }
}

impl<N, D> Eq for Fraction<N, D> where Self: FractionEq<Self> {}

impl<N, D> fmt::Debug for Fraction<N, D>
where
    N: Unsigned,
    D: Unsigned,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Fraction<{numerator}/{divisor}>",
            numerator = N::U64,
            divisor = D::U64,
        ))
    }
}

impl<N, D> fmt::Display for Fraction<N, D>
where
    N: Unsigned,
    D: Unsigned,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numerator = N::U64;
        let divisor = D::U64;

        if !f.alternate() {
            f.write_fmt(format_args!(
                "{numerator}/{divisor}",
                numerator = numerator,
                divisor = divisor,
            ))
        } else {
            if numerator == 0 {
                f.write_char('0')
            } else if divisor == 1 {
                f.write_fmt(format_args!("{}", numerator))
            } else if divisor == numerator {
                f.write_char('1')
            } else {
                // TODO: use gcd here?...
                f.write_fmt(format_args!(
                    "{numerator}/{divisor}",
                    numerator = numerator,
                    divisor = divisor,
                ))
            }
        }
}

#[cfg(test)]
mod tests {
    use core::ops::Mul;
    use typenum::{U0, U1, U3, U10, U100, U1000, U36};

    type U3600 = <U36 as Mul<U100>>::Output;

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", <Frac![U1]>::new()), "Fraction<1/1>");
        assert_eq!(format!("{:?}", <Frac![U1 / U1]>::new()), "Fraction<1/1>");
        assert_eq!(
            format!("{:?}", <Frac![U1000 / U3600]>::new()),
            "Fraction<1000/3600>"
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", <Frac![U10]>::new()), "10/1");
        assert_eq!(format!("{}", <Frac![U100 / U1]>::new()), "100/1");
        assert_eq!(format!("{}", <Frac![U3 / U3]>::new()), "3/3");
        assert_eq!(format!("{}", <Frac![U0 / U3]>::new()), "0/3");
        assert_eq!(format!("{}", <Frac![U1000 / U3600]>::new()), "1000/3600");
    }

    #[test]
    fn cooler_display() {
        assert_eq!(format!("{:#}", <Frac![U10]>::new()), "10");
        assert_eq!(format!("{:#}", <Frac![U100 / U1]>::new()), "100");
        assert_eq!(format!("{:#}", <Frac![U3 / U3]>::new()), "1");
        assert_eq!(format!("{:#}", <Frac![U0 / U3]>::new()), "0");
        assert_eq!(format!("{:#}", <Frac![U1000 / U3600]>::new()), "1000/3600");
    }
}
