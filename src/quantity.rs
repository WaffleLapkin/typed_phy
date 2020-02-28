use core::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{
    checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub},
    eq::DimensionsEq,
    fraction::{FractionTrait, One},
    from_int::FromUnsigned,
    id::Id,
    unit::UnitTrait,
    units::Dimensionless,
    Unit,
};

/// Base type of the whole lib
///
/// Represent quantity of unit `U` that is stored in storage (integer) `S`.
///
/// Note that you can
/// - add/sub quantities of the same unit
/// - mul/div quantities by other quantities (even of different units)
/// - mul/div quantities by integers
/// (but in any case you need same storage type)
///
/// ## Examples
///
/// ```
/// use typed_phy::{
///     units::{Metre, SquareMetre},
///     IntExt, Quantity,
/// };
///
/// let x = Quantity::<i32, Metre>::new(10);
/// let y = Quantity::<i32, Metre>::new(20);
///
/// let sum = x + y;
/// assert_eq!(sum, 30.m());
///
/// let diff = x - y;
/// assert_eq!(diff, -10.m());
///
/// let doubled = x * 2;
/// assert_eq!(doubled, 20.m());
///
/// let mult = x * y;
/// assert_eq!(mult, 200.quantity::<SquareMetre>());
/// ```
pub struct Quantity<S, U> {
    storage: S,
    _unit: PhantomData<U>,
}

impl<S, U> Quantity<S, U> {
    /// Creates new quantity from the given value.
    ///
    /// Note: in most cases it's more convenient to use methods of [`IntExt`].
    ///
    /// [`IntExt`]: crate::IntExt
    ///
    /// ## Examples
    /// ```
    /// use typed_phy::{units::Metre, Quantity};
    ///
    /// let x = Quantity::<i32, Metre>::new(10);
    /// let y = Quantity::<_, Metre>::new(20);
    ///
    /// assert_eq!(x + y, Quantity::new(30));
    /// ```
    #[inline]
    pub const fn new(value: S) -> Self {
        Self {
            storage: value,
            _unit: PhantomData,
        }
    }

    /// Return inner value.
    ///
    /// Note: it's recommended to only use this method if you need to pass value
    /// to a function (e.g. from some of your dependencies) that works with
    /// integers. Careless usage of this method can lead to bugs (e.g. with
    /// wrong units).
    ///
    /// ## Examples
    /// ```
    /// # pub mod lib { pub fn square_perimeter(x: u32, y: u32) -> u32 { (x + y) * 2 } }
    /// // TODO: example that is more sensitive to units
    ///
    /// use typed_phy::{units::Metre, IntExt, Quantity};
    ///
    /// // Imagine that you get it from somewhere (e.g. sensors)
    /// let (x, y) = (10.m(), 3.m());
    ///
    /// // bad
    /// assert_eq!(x.into_inner() * y.into_inner(), 30);
    /// // use instead:
    /// assert_eq!(x * y, 30.sqm());
    ///
    /// // ok
    /// let perimeter = lib::square_perimeter(x.into_inner(), y.into_inner()).m();
    /// assert_eq!(perimeter, 26.m());
    /// ```
    ///
    /// See also: [`value`](Quantity::value)
    #[inline]
    pub fn into_inner(self) -> S {
        self.storage
    }

    /// Applies the given function to the raw value.
    ///
    /// Actually not sure if this function even need to exist.
    #[inline]
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(S) -> S,
    {
        Self {
            storage: f(self.storage),
            ..self
        }
    }

    /// Sets unit to the same unit. It may seem useless, but it (hopefuly) can
    /// help IDE understand right type of the expression (e.g. with type
    /// alias)
    #[inline]
    pub fn r#as<T>(self) -> Quantity<S, T>
    where
        Self: Id<This = Quantity<S, T>>,
    {
        self.id_cast()
    }

    pub(crate) fn set_unit_unchecked<U0>(self) -> Quantity<S, U0> {
        Quantity {
            storage: self.storage,
            _unit: PhantomData,
        }
    }
}

impl<S> Quantity<S, Dimensionless> {
    /// Same as [`into_inner`], but work only for dimensionless quantities.
    /// Refer to [`into_inner`]'s docs for more.
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::IntExt;
    ///
    /// let it = 10.m() / 2.m();
    /// assert_eq!(it.value(), 5);
    /// ```
    ///
    /// ```compile_fail,E0599
    /// use typed_phy::IntExt;
    ///
    /// let it = 10.m() + 2.m();
    /// // error[E0599]: no method named `value` found for struct `typed_phy::Quantity<{integer}, typed_phy::Unit<typenum::int::PInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0>>` in the current scope
    /// //   --> src/quantity.rs:156:12
    /// //    |
    /// // 12 | let _ = it.value();
    /// //    |            ^^^^^ method not found in `typed_phy::Quantity<{integer}, typed_phy::Unit<typenum::int::PInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>>, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0, typenum::int::Z0>>`
    /// let _ = it.value();
    /// ```
    /// (yeah, error is weird, but it says that metre isn't dimensionless)
    ///
    /// [`into_inner`]: Quantity::into_inner
    #[inline]
    pub fn value(self) -> S {
        self.storage
    }
}

impl<S, U> Quantity<S, U>
where
    U: UnitTrait,
    U::Ratio: FractionTrait,
    S: FromUnsigned + Mul<Output = S> + Div<Output = S>,
{
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{
    ///     prefixes::{Deci, Kilo},
    ///     units::{Hour, Metre, Minute},
    ///     IntExt,
    /// };
    ///
    /// assert_eq!(10.km().to::<Deci<Metre>>(), 100_000.dm());
    /// assert_eq!(100_000.dm().to::<Kilo<Metre>>(), 10.km());
    ///
    /// assert_eq!(3600.s().to::<Hour>(), 1.h());
    /// assert_eq!(5.h().to::<Minute>(), 300.min_());
    /// ```
    #[inline]
    pub fn to<T>(self) -> Quantity<S, T>
    where
        T: UnitTrait,
        U::Dimensions: DimensionsEq<T::Dimensions>,
    {
        Quantity::new(T::Ratio::div(U::Ratio::mul(self.storage)))
    }

    /// ## Examples
    ///
    /// ```
    /// use typed_phy::IntExt;
    ///
    /// assert_eq!(10.km().to_base(), 10_000.m());
    /// assert_eq!(10.dm().to_base(), 1.m());
    ///
    /// assert_eq!(10.min_().to_base(), 600.s());
    /// ```
    #[inline]
    #[allow(clippy::wrong_self_convention)] // TODO: better name
    #[allow(clippy::type_complexity)]
    pub fn to_base(self) -> Quantity<S, Unit<U::Dimensions, One>> {
        self.to()
    }
}

impl<S, U> Default for Quantity<S, U>
where
    S: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(S::default())
    }
}

/// Addition between 2 quantities of the same unit (`U`) and storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(20.s() + 10.s(), 30.s())
/// ```
impl<S, U> Add for Quantity<S, U>
where
    S: Add<Output = S>,
{
    type Output = Quantity<S, U>;

    #[inline]
    fn add(self, rhs: Quantity<S, U>) -> Self::Output {
        self.map(|s| s + rhs.storage)
    }
}

/// Subtraction between 2 quantities of the same unit (`U`) and storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(20.s() - 10.s(), 10.s())
/// ```
impl<S, U> Sub for Quantity<S, U>
where
    S: Sub<Output = S>,
{
    type Output = Quantity<S, U>;

    #[inline]
    fn sub(self, rhs: Quantity<S, U>) -> Self::Output {
        self.map(|s| s - rhs.storage)
    }
}

/// Multiplication between 2 quantities of the same storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(20.m() * 10.m(), 200.sqm()) // TODO example with different units
/// ```
impl<S, U0, U1> Mul<Quantity<S, U1>> for Quantity<S, U0>
where
    S: Mul<Output = S>,
    U0: UnitTrait + Mul<U1>,
    U1: UnitTrait,
{
    type Output = Quantity<S, <U0 as Mul<U1>>::Output>;

    #[inline]
    fn mul(self, rhs: Quantity<S, U1>) -> Self::Output {
        self.map(|s| s * rhs.storage).set_unit_unchecked()
    }
}

/// Division between 2 quantities of the same storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(20.m() / 10.s(), 2.mps())
/// ```
impl<S, U0, U1> Div<Quantity<S, U1>> for Quantity<S, U0>
where
    S: Div<Output = S>,
    U0: UnitTrait + Div<U1>,
    U1: UnitTrait,
{
    type Output = Quantity<S, <U0 as Div<U1>>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<S, U1>) -> Self::Output {
        self.map(|s| s / rhs.storage).set_unit_unchecked()
    }
}

/// Multiplication between quantity and integer.
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(1.m() * 10, 10.m())
/// ```
impl<S, U> Mul<S> for Quantity<S, U>
where
    S: Mul<Output = S>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: S) -> Self::Output {
        self.map(|s| s * rhs)
    }
}

/// Division between quantity and integer.
///
/// ## Examples
/// ```
/// use typed_phy::IntExt;
/// assert_eq!(20.m() / 2, 10.m())
/// ```
impl<S, U> Div<S> for Quantity<S, U>
where
    S: Div<Output = S>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: S) -> Self::Output {
        self.map(|s| s / rhs)
    }
}

impl<S, U> Neg for Quantity<S, U>
where
    S: Neg,
{
    type Output = Quantity<S::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Quantity::new(-self.storage)
    }
}

/// Addition between 2 quantities of the same unit (`U`) and storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedAdd, IntExt};
/// assert_eq!(20.s().checked_add(10.s()), Some(30.s()));
/// assert_eq!(i32::max_value().s().checked_add(10.s()), None);
/// ```
impl<S, U> CheckedAdd for Quantity<S, U>
where
    S: CheckedAdd<Output = S>,
{
    #[inline]
    fn checked_add(self, rhs: Quantity<S, U>) -> Option<Self::Output> {
        self.storage.checked_add(rhs.storage).map(Self::new)
    }
}

/// Subtraction between 2 quantities of the same unit (`U`) and storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedSub, IntExt};
/// assert_eq!(20.s().checked_sub(10.s()), Some(10.s()));
/// assert_eq!((-2.s()).checked_sub(i32::max_value().s()), None);
/// ```
impl<S, U> CheckedSub for Quantity<S, U>
where
    S: CheckedSub<Output = S>,
{
    #[inline]
    fn checked_sub(self, rhs: Quantity<S, U>) -> Option<Self::Output> {
        self.storage.checked_sub(rhs.storage).map(Self::new)
    }
}

/// Multiplication between 2 quantities of the same storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedMul, IntExt};
/// assert_eq!(20.m().checked_mul(10.m()), Some(200.sqm())); // TODO example with different units
/// assert_eq!(20.m().checked_mul(107374199.m()), None);
/// ```
impl<S, U0, U1> CheckedMul<Quantity<S, U1>> for Quantity<S, U0>
where
    S: CheckedMul<Output = S>,
    U0: UnitTrait + Mul<U1>,
    U1: UnitTrait,
{
    #[inline]
    fn checked_mul(self, rhs: Quantity<S, U1>) -> Option<Self::Output> {
        self.storage.checked_mul(rhs.storage).map(Quantity::new)
    }
}

/// Division between 2 quantities of the same storage (`S`).
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedDiv, IntExt};
/// assert_eq!(20.m().checked_div(10.s()), Some(2.mps()));
/// assert_eq!(20.m().checked_div(0.s()), None);
/// ```
impl<S, U0, U1> CheckedDiv<Quantity<S, U1>> for Quantity<S, U0>
where
    S: CheckedDiv<Output = S>,
    U0: UnitTrait + Div<U1>,
    U1: UnitTrait,
{
    #[inline]
    fn checked_div(self, rhs: Quantity<S, U1>) -> Option<Self::Output> {
        self.storage.checked_div(rhs.storage).map(Quantity::new)
    }
}

/// Multiplication between quantity and integer.
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedMul, IntExt};
/// assert_eq!(1.m().checked_mul(10), Some(10.m()));
/// assert_eq!(i32::max_value().m().checked_mul(10), None);
/// ```
impl<S, U> CheckedMul<S> for Quantity<S, U>
where
    S: CheckedMul<Output = S>,
{
    #[inline]
    fn checked_mul(self, rhs: S) -> Option<Self::Output> {
        self.storage.checked_mul(rhs).map(Self::new)
    }
}

/// Division between quantity and integer.
///
/// ## Examples
/// ```
/// use typed_phy::{checked::CheckedDiv, IntExt};
/// assert_eq!(20.m().checked_div(2), Some(10.m()));
/// assert_eq!(20.m().checked_div(0), None);
/// ```
impl<S, U> CheckedDiv<S> for Quantity<S, U>
where
    S: CheckedDiv<S, Output = S>,
{
    #[inline]
    fn checked_div(self, rhs: S) -> Option<Self::Output> {
        self.storage.checked_div(rhs).map(Self::new)
    }
}

// We need to use handwritten impl to prevent unnecessary bounds on generics
impl<S, U> Debug for Quantity<S, U>
where
    S: Debug,
    U: Debug + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Quantity<_, {unit:?}>({value:?})",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> Display for Quantity<S, U>
where
    S: Display,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> Clone for Quantity<S, U>
where
    S: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.storage.clone())
    }
}

impl<S, U> Copy for Quantity<S, U> where S: Copy {}

impl<S, U> Eq for Quantity<S, U> where S: Eq {}

impl<S0, S1, U> PartialEq<Quantity<S1, U>> for Quantity<S0, U>
where
    S0: PartialEq<S1>,
{
    #[inline]
    fn eq(&self, other: &Quantity<S1, U>) -> bool {
        self.storage.eq(&other.storage)
    }
}

impl<S, U> Ord for Quantity<S, U>
where
    S: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.storage.cmp(&other.storage)
    }
}

impl<S0, S1, U> PartialOrd<Quantity<S1, U>> for Quantity<S0, U>
where
    S0: PartialOrd<S1>,
{
    #[inline]
    fn partial_cmp(&self, other: &Quantity<S1, U>) -> Option<Ordering> {
        self.storage.partial_cmp(&other.storage)
    }
}

#[cfg(test)]
mod tests {
    use typenum::{N1, N2, P1, U15, U71};

    use crate::{prefixes::*, units::*, Dimensions, IntExt, Quantity, Unit};

    macro_rules! assert_display_eq {
        ($T:ty, $s:expr $(,)?) => {
            assert_eq!(format!("{}", Quantity::<_, $T>::new(42)), $s);
        };
    }

    #[test]
    fn simple() {
        let length = 20.m() + 4.m();
        let time = 2.s() * 3;

        let speed = length / time;

        assert_eq!(speed, 4.mps());
    }

    #[test]
    fn display() {
        assert_display_eq!(Metre, "42 m");
        assert_display_eq!(Kilo::<Hertz>, "42 kHz");
        assert_display_eq!(Pico::<Second>, "42 ps");
        assert_display_eq!(
            Unit::<Dimensions<P1, N2, P1, N1, N1, P1, P1>, Frac![U15 / U71]>,
            "42 m * kg^-2 * s * A^-1 * K^-1 * mol * cd (ratio: 15/71)",
        );
    }
}
