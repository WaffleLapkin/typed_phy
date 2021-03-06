use core::{
    cmp::Ordering,
    fmt::{self, Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex},
    iter::Sum,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

// #[cfg(feature = "nightly")]
// use core::iter::Step;

use typenum::{Prod, Quot};

use crate::{
    checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub},
    fraction::{FractionTrait, One},
    from_int::FromUnsigned,
    id::Id,
    unit::UnitTrait,
    units::Dimensionless,
    Unit,
};

#[rustfmt::skip] // this is needed to prevent md table breakage. (see https://github.com/rust-lang/rustfmt/issues/4210)
/// Base type of the whole lib
///
/// Represent quantity of unit `U` that is stored in storage (integer) `S`.
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
///
/// ## Operations
///
/// There are a plenty of arithmetic operations those can be done with `Quantity`! All (most?) of
/// them are present in the following table (given `a`, `b` quantities of the same unit, `b'`
/// quantity of (maybe) a different unit and `s` storage integer type):
///
/// | Trait                                           | rhs Unit | Output                          | Call way            | Description                                                                                  |
/// |-------------------------------------------------|----------|---------------------------------|---------------------|----------------------------------------------------------------------------------------------|
/// | [`Add`](core::ops::Add)                         | Same     | `Self`                          | `a + b`             | sum of 2 quantities, works only with the same units                                          |
/// | [`Sub`](core::ops::Sub)                         | Same     | `Self`                          | `a - b`             | diff of 2 quantities, works only with the same units                                         |
/// | [`Mul`](core::ops::Mul)                         | Any      | `Quantity<S, U * URhs>`         | `a * b'`            | production of 2 quantities, changes unit                                                     |
/// | [`Div`](core::ops::Div)                         | Any      | `Quantity<S, U / URhs>`         | `a / b'`            | quotation of 2 quantities, changes unit                                                      |
/// | [`Mul`](core::ops::Mul)`<S>`                    | n/a      | `Self`                          | `a * s`             | production of quantity and an integer                                                        |
/// | [`Div`](core::ops::Div)`<S>`                    | n/a      | `Self`                          | `a / s`             | quotation of quantity and an integer                                                         |
/// | [`Neg`](core::ops::Neg)                         | n/a      | `Self`                          | `-a`                | negation of quantity                                                                         |
/// | [`CheckedAdd`](crate::checked::CheckedAdd)      | Same     | `Option<Self>`                  | `a.checked_add(b)`  | sum of 2 quantities, works only with the same units, checks for overflow and underflow       |
/// | [`CheckedSub`](crate::checked::CheckedSub)      | Same     | `Option<Self>`                  | `a.checked_sub(b)`  | diff of 2 quantities, works only with the same units, checks for overflow and underflow      |
/// | [`CheckedMul`](crate::checked::CheckedMul)      | Any      | `Option<Quantity<S, U * URhs>>` | `a.checked_mul(b')` | production of 2 quantities, changes unit, checks for overflow and underflow                  |
/// | [`CheckedDiv`](crate::checked::CheckedDiv)      | Any      | `Option<Quantity<S, U / URhs>>` | `a.checked_div(b')` | quotation of 2 quantities, changes unit, checks for overflow, underflow and division by zero |
/// | [`CheckedMul`](crate::checked::CheckedMul)`<S>` | n/a      | `Option<Self>`                  | `a.checked_mul(s)`  | production of quantity and an integer, checks for overflow and underflow                     |
/// | [`CheckedDiv`](crate::checked::CheckedDiv)`<S>` | n/a      | `Option<Self>`                  | `a.checked_div(s)`  | quotation of quantity and an integer, checks for overflow, underflow and division by zero    |
/// | [`AddAssign`](core::ops::AddAssign)             | Same     | `()`                            | `a += b`            | adds one quantity to another mutating the destination (`a`)                                  |
/// | [`SubAssign`](core::ops::SubAssign)             | Same     | `()`                            | `a -= b`            | subtracts one quantity from another mutating the destination (`a`)                           |
/// | [`MulAssign`](core::ops::MulAssign)`<S>`        | n/a      | `()`                            | `a *= s`            | multiplies quantity by an integer mutating the destination (`a`)                             |
/// | [`DivAssign`](core::ops::DivAssign)`<S>`        | n/a      | `()`                            | `a /= s`            | divides quantity by an integer mutating the destination (`a`)                                |
/// | [`Rem`](core::ops::Rem)                         | Any      | `Quantity<S, U / URhs>`         | `a % b'`            | remainder of the division of 2 quantities                                                    |
/// | [`Rem`](core::ops::Rem)`<S>`                    | n/a      | `Self`                          | `a % s`             | remainder of the division quantity by an integer                                             |
/// | [`RemAssign`](core::ops::RemAssign)`<S>`        | n/a      | `()`                            | `a %= s`            | sets `a` to the remainder of division `a` by an integer                                    |
// to edit such a big table, it's recommended to use smt like https://www.tablesgenerator.com/markdown_tables
///
/// ## Formatting
///
/// `Quantity` implements pretty much every trait from [`core::fmt`](core::fmt) (ofc except
/// `Pointer` and `Write`). The `Quantity` consists of the storage `S` and unit `U`,
/// both of them are icluded in the output. The storage `S` will be always formatted with the same
/// trait as `Quantity`. **However** the unit `U` will be formatted with `Debug`, if `Quantity` is
/// formatted with `Debug`, and with `Display` otherwise.
///
/// See [`Unit`s](crate::Unit#formatting) docs for info about formatting units.
///
/// ```rust
/// use typed_phy::IntExt;
///
/// let quantity = 10.m();
///
/// assert_eq!(
///     format!("{:?}", quantity),
///     "Quantity<_, Unit<Dimensions<1, 0, 0, 0, 0, 0, 0>, Fraction<1/1>>>(10)"
/// ); // Debug
/// assert_eq!(format!("{}", quantity), "10 m"); // Display
///
/// assert_eq!(format!("{:b}", quantity), "1010 m"); // Binary
/// assert_eq!(format!("{:x}", quantity), "a m"); // LowerHex
/// assert_eq!(format!("{:X}", quantity), "A m"); // UpperHex
/// assert_eq!(format!("{:o}", quantity), "12 m"); // Octal
///
/// let quantity = 1020.0.m();
/// assert_eq!(format!("{:e}", quantity), "1.02e3 m"); // LowerExp
/// assert_eq!(format!("{:E}", quantity), "1.02E3 m"); // UpperExp
/// ```
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "deser", serde(transparent))]
#[derive(Hash)]
pub struct Quantity<S, U> {
    storage: S,
    // TODO: think a bit more about the serialization. Currently only the Inner storage is
    //       (de)serialized, but maybe we should also serialize the exponents?...
    #[cfg_attr(feature = "deser", serde(skip))]
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
        Self::new(f(self.storage))
    }

    /// Sets unit to the same unit. It may seem useless, but it (hopefully) can
    /// help IDE understand right type of the expression (e.g. with type
    /// alias)
    #[inline]
    pub fn r#as<T>(self) -> Quantity<S, T>
    where
        Self: Id<This = Quantity<S, T>>,
    {
        self.id_cast()
    }

    pub(crate) fn set_unit_unchecked<T>(self) -> Quantity<S, T> {
        Quantity::new(self.storage)
    }
}

impl<S, U> Quantity<S, U>
where
    U: UnitTrait,
{
    /// Set unit.
    ///
    /// This function changes only the ratio, this won't change the dimensions.
    ///
    /// This function **doesn't** change the underlying value. (So `1000 m`
    /// becomes `1000 km`, not `1 km`)
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{prefixes::Kilo, units::Metre, IntExt, Quantity};
    ///
    /// let km: Quantity<_, Kilo<Metre>> = 1.km();
    /// let m: Quantity<_, Metre> = km.set_unit();
    /// assert_eq!(m, 1.m());
    /// assert_eq!(2.km().set_unit::<Metre>(), 2.m());
    /// ```
    /// ```compile_fail,E0271
    /// # use typed_phy::{IntExt, units::Second};
    /// 1.m().set_unit::<Second>();
    /// ```
    #[inline]
    pub fn set_unit<T>(self) -> Quantity<S, T>
    where
        T: UnitTrait<Dimensions = U::Dimensions>,
    {
        Quantity::new(self.storage)
    }

    /// Set ratio.
    ///
    /// This function **doesn't** change the underlying value. (So `1000 m`
    /// becomes `1000 km`, not `1 km`).
    #[inline]
    pub fn set_ratio<T>(self) -> Quantity<S, Unit<U::Dimensions, T>> {
        Quantity::new(self.storage)
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
    /// Changes ratio _saving_ the quantity. (So `1000 m` becomes `1 km`, not
    /// `1000 km`)
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{Frac, IntExt};
    ///
    /// use typenum::{U1, U10};
    ///
    /// assert_eq!(10.km().into_ratio::<Frac![U1 / U10]>(), 100_000.dm());
    /// ```
    #[inline]
    pub fn into_ratio<T>(self) -> Quantity<S, Unit<U::Dimensions, T>>
    where
        T: FractionTrait,
    {
        self.into_unit()
    }

    /// Convert self to other unit _saving_ the quantity. (So `1000 m` becomes
    /// `1 km`, not `1000 km`)
    ///
    /// Both units must have the same dimensions.
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::{
    ///     prefixes::{Deci, Kilo},
    ///     units::{Hour, Metre, Minute},
    ///     IntExt,
    /// };
    ///
    /// assert_eq!(10.km().into_unit::<Deci<Metre>>(), 100_000.dm());
    /// assert_eq!(100_000.dm().into_unit::<Kilo<Metre>>(), 10.km());
    ///
    /// assert_eq!(3600.s().into_unit::<Hour>(), 1.h());
    /// assert_eq!(5.h().into_unit::<Minute>(), 300.min_());
    /// ```
    #[inline]
    pub fn into_unit<T>(self) -> Quantity<S, T>
    where
        T: UnitTrait<Dimensions = U::Dimensions>,
    {
        Quantity::new(T::Ratio::div(U::Ratio::mul(self.storage)))
    }

    /// Same as [`into_unit`], but converts to 'base' unit (with ratio = 1)
    ///
    /// ## Examples
    ///
    /// ```
    /// use typed_phy::IntExt;
    ///
    /// assert_eq!(10.km().into_base(), 10_000.m());
    /// assert_eq!(10.dm().into_base(), 1.m());
    /// assert_eq!(10.h().into_base(), 36000.s());
    /// assert_eq!(10.min_().into_base(), 600.s());
    /// assert_eq!((100.m() * 3.km()).into_base(), 300_000.sqm());
    /// ```
    ///
    /// [`into_unit`]: Self::into_unit
    #[inline]
    pub fn into_base(self) -> Quantity<S, Unit<U::Dimensions, One>> {
        self.into_unit()
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
    type Output = Quantity<S, Prod<U0, U1>>;

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
    type Output = Quantity<S, Quot<U0, U1>>;

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
    S: CheckedDiv<Output = S>,
{
    #[inline]
    fn checked_div(self, rhs: S) -> Option<Self::Output> {
        self.storage.checked_div(rhs).map(Self::new)
    }
}

impl<S, U> AddAssign for Quantity<S, U>
where
    S: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Quantity<S, U>) {
        self.storage.add_assign(rhs.storage);
    }
}

impl<S, U> SubAssign for Quantity<S, U>
where
    S: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Quantity<S, U>) {
        self.storage.sub_assign(rhs.storage);
    }
}

impl<S, U> MulAssign<S> for Quantity<S, U>
where
    S: MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, rhs: S) {
        self.storage.mul_assign(rhs);
    }
}

impl<S, U> DivAssign<S> for Quantity<S, U>
where
    S: DivAssign,
{
    #[inline]
    fn div_assign(&mut self, rhs: S) {
        self.storage.div_assign(rhs);
    }
}

impl<S, U> Rem<S> for Quantity<S, U>
where
    S: Rem,
{
    type Output = Quantity<S::Output, U>;

    #[inline]
    fn rem(self, rhs: S) -> Self::Output {
        Self::Output::new(self.storage % rhs)
    }
}

impl<S, U> RemAssign<S> for Quantity<S, U>
where
    S: RemAssign,
{
    #[inline]
    fn rem_assign(&mut self, rhs: S) {
        self.storage.rem_assign(rhs)
    }
}

impl<S, U0, U1> Rem<Quantity<S, U1>> for Quantity<S, U0>
where
    S: Rem,
    U0: UnitTrait + Div<U1>,
    U1: UnitTrait,
{
    type Output = Quantity<S::Output, Quot<U0, U1>>;

    #[inline]
    fn rem(self, rhs: Quantity<S, U1>) -> Self::Output {
        Self::Output::new(self.storage % rhs.storage)
    }
}

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

impl<S, U> Binary for Quantity<S, U>
where
    S: Binary,
    U: Display + Default, /* Not sure about this, but I don't quite see
                           * the reasons to implement Binary for Unit anyway */
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:b} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> LowerExp for Quantity<S, U>
where
    S: LowerExp,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:e} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> LowerHex for Quantity<S, U>
where
    S: LowerHex,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:x} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> Octal for Quantity<S, U>
where
    S: Octal,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:o} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> UpperExp for Quantity<S, U>
where
    S: UpperExp,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:E} {unit}",
            value = self.storage,
            unit = U::default(),
        ))
    }
}

impl<S, U> UpperHex for Quantity<S, U>
where
    S: UpperHex,
    U: Display + Default,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{value:X} {unit}",
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

// TODO: `From` impl to change ratio
impl<S, U> From<S> for Quantity<S, U> {
    #[inline]
    fn from(i: S) -> Self {
        Self::new(i)
    }
}

impl<S, U> Sum for Quantity<S, U>
where
    Self: Add<Output = Self> + Default,
{
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), Self::add)
    }
}

// #[cfg(feature = "nightly")]
// impl<S, U> Step for Quantity<S, U>
// where
//     S: Step,
// {
//     #[inline]
//     fn steps_between(start: &Self, end: &Self) -> Option<usize> {
//         <_>::steps_between(&start.storage, &end.storage)
//     }
//
//     #[inline]
//     fn replace_one(&mut self) -> Self {
//         Self::new(self.storage.replace_one())
//     }
//
//     #[inline]
//     fn replace_zero(&mut self) -> Self {
//         Self::new(self.storage.replace_zero())
//     }
//
//     #[inline]
//     fn add_one(&self) -> Self {
//         Self::new(self.storage.add_one())
//     }
//
//     #[inline]
//     fn sub_one(&self) -> Self {
//         Self::new(self.storage.sub_one())
//     }
//
//     #[inline]
//     fn add_usize(&self, n: usize) -> Option<Self> {
//         self.storage.add_usize(n).map(Self::new)
//     }
// }

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
            "42 m * kg^-2 * s * A^-1 * K^-1 * mol * cd (ratio: 15 / 71)",
        );
    }

    #[test]
    #[cfg_attr(not(feature = "deser"), ignore)]
    fn serde() {
        #[cfg(feature = "deser")] // won't compile without (De)Serialize traits derived
        serde_test::assert_tokens(&(10.m() / 5.s()), &[serde_test::Token::I32(2)])
    }

    #[test]
    fn iter_traits() {
        #[cfg(nightly)]
        let iter = 1.s()..=10.s();
        #[cfg(not(nightly))]
        let iter = (1..=10).map(<_>::s);

        // Sum of first n elements of arithmetic progression is equal to `n(a1 + an)/2`
        // `10 * (1 + 10) / 2 == 55`
        assert_eq!(iter.sum::<Quantity<_, _>>(), 55.s());
    }

    #[test]
    fn rem() {
        assert_eq!(10.s() % 3, 1.s());
        assert_eq!(10.mps() % 4.m(), 2.quantity::<Hertz>());

        let mut var = 20.s();
        var %= 8;
        assert_eq!(var, 4.s());
    }
}
