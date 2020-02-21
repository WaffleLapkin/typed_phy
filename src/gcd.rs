#![allow(clippy::type_complexity)]

use typenum::{Unsigned, Z0, UInt, B0, U2, B1, Max, Min, U0};
use core::ops::{Div, Mul, Sub};

/// Type-level operator that counts `gcd` (Greatest Common Divisor) for to typenum's integers using
/// [Binary GCD algorithm]\:
///
///  1. `gcd(0, v) = v`, `gcd(u, 0) = u`
///  2. If `u` and `v` are both even, then `gcd(u, v) = 2·gcd(u/2, v/2)`
///  3. If `u` is even and `v` is odd, `then gcd(u, v) = gcd(u/2, v)`. Similarly, if `u` is odd and `v` is even, then `gcd(u, v) = gcd(u, v/2)`
///  4. If `u` and `v` are both odd, `gcd(u, v) = gcd((max − min)/2, min)` where `min = min(u, v)`, `max = max(u, v)`
///
/// ## Examples
///
/// ```
/// use typenum::{U5, U10, U17, U0};
/// use typed_phy::gcd::Gcd;
/// use typenum::marker_traits::Unsigned;
///
/// assert_eq!(<U10 as Gcd<U5>>::Output::I32, 5);
/// assert_eq!(<U10 as Gcd<U10>>::Output::I32, 10);
/// assert_eq!(<U5 as Gcd<U10>>::Output::I32, 5);
/// assert_eq!(<U17 as Gcd<U5>>::Output::I32, 1);
/// assert_eq!(<U0 as Gcd<U5>>::Output::I32, 5);
/// assert_eq!(<U10 as Gcd<U0>>::Output::I32, 10);
/// ```
///
/// [Binary GCD algorithm]: https://en.wikipedia.org/wiki/Binary_GCD_algorithm
pub trait Gcd<N> {
    /// Greatest Common Divisor of `Self` and `N`
    type Output;
}

/// `gcd(0, v) = v`
impl<V: Unsigned, B> Gcd<UInt<V, B>> for Z0 {
    type Output = UInt<V, B>;
}

/// `gcd(u, 0) = u`
impl<U: Unsigned, B> Gcd<Z0> for UInt<U, B> {
    type Output = UInt<U, B>;
}

/// `gcd(0, v) = v`
impl<V: Unsigned, B> Gcd<UInt<V, B>> for U0 {
    type Output = UInt<V, B>;
}

/// `gcd(u, 0) = u`
impl<U: Unsigned, B> Gcd<U0> for UInt<U, B> {
    type Output = UInt<U, B>;
}

type Even<U> = UInt<U, B0>;
type Odd<U> = UInt<U, B1>;

/// `u` and `v` are both even, then `gcd(u, v) = 2·gcd(u/2, v/2)`
impl<M: Unsigned, N: Unsigned> Gcd<Even<N>> for Even<M>
where
    Even<M>: Div<U2>,
    Even<N>: Div<U2>,
    <Even<M> as Div<U2>>::Output: Gcd<<Even<N> as Div<U2>>::Output>,
    <<Even<M> as Div<U2>>::Output as Gcd<<Even<N> as Div<U2>>::Output>>::Output: Mul<U2>,
{
    type Output = <<<Even<M> as Div<U2>>::Output as Gcd<<Even<N> as Div<U2>>::Output>>::Output as Mul<U2>>::Output;
}

/// `u` is even and `v` is odd, `then gcd(u, v) = gcd(u/2, v)`
impl<U: Unsigned, V: Unsigned> Gcd<Odd<V>> for Even<U>
where
    Even<U>: Div<U2>,
    <Even<U> as Div<U2>>::Output: Gcd<Odd<V>>,
{
    type Output = <<Even<U> as Div<U2>>::Output as Gcd<Odd<V>>>::Output;
}

/// `u` is odd and `v` is even, then `gcd(u, v) = gcd(u, v/2)`
impl<U: Unsigned, V: Unsigned> Gcd<Even<V>> for Odd<U>
where
    Even<V>: Div<U2>,
    Odd<U>: Gcd<<Even<V> as Div<U2>>::Output>,
{
    type Output = <Odd<U> as Gcd<<Even<V> as Div<U2>>::Output>>::Output;
}

/// `u` and `v` are both odd, `gcd(u, v) = gcd((max − min)/2, min)` where `min = min(u, v)`, `max = max(u, v)`
impl<U: Unsigned, V: Unsigned> Gcd<Odd<V>> for Odd<U>
where
    Odd<U>: Max<Odd<V>> + Min<Odd<V>> ,
    <Odd<U> as Max<Odd<V>>>::Output: Sub<<Odd<U> as Min<Odd<V>>>::Output>,
    <<Odd<U> as Max<Odd<V>>>::Output as Sub<<Odd<U> as Min<Odd<V>>>::Output>>::Output: Div<U2>,
    <<<Odd<U> as Max<Odd<V>>>::Output as Sub<<Odd<U> as Min<Odd<V>>>::Output>>::Output as Div<U2>>::Output: Gcd<<Odd<U> as Min<Odd<V>>>::Output>,
{
    type Output = <<<<Odd<U> as Max<Odd<V>>>::Output as Sub<<Odd<U> as Min<Odd<V>>>::Output>>::Output as Div<U2>>::Output as Gcd<<Odd<U> as Min<Odd<V>>>::Output>>::Output;
}
