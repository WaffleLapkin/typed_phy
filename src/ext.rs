use crate::{
    prefixes::{Deci, Kilo},
    units::{
        Dimensionless, Hour, KiloGram, KiloMetrePerHour, Metre, MetrePerSecond, Minute, Second,
        SquareMetre,
    },
    Quantity,
};

/// Extension for integers for creating quantities of common units.
///
/// ## Examples
/// ```
/// use typed_phy::{
///     units::{MetrePerSecond, Second},
///     IntExt, Quantity,
/// };
///
/// let minute: Quantity<i32, Second> = 60.s();
/// assert_eq!(minute.into_inner(), 60);
///
/// let escape: Quantity<i32, MetrePerSecond> = 11200.mps();
/// assert_eq!(escape.into_inner(), 11200);
/// ```
#[allow(missing_docs)]
pub trait IntExt: Sized {
    #[inline]
    fn quantity<U>(self) -> Quantity<Self, U> {
        Quantity::new(self)
    }

    #[inline]
    fn dimensionless(self) -> Quantity<Self, Dimensionless> {
        self.quantity()
    }

    #[inline]
    fn m(self) -> Quantity<Self, Metre> {
        self.quantity()
    }

    #[inline]
    fn s(self) -> Quantity<Self, Second> {
        self.quantity()
    }

    #[inline]
    fn kg(self) -> Quantity<Self, KiloGram> {
        self.quantity()
    }

    #[inline]
    fn mps(self) -> Quantity<Self, MetrePerSecond> {
        self.quantity()
    }

    #[inline]
    fn sqm(self) -> Quantity<Self, SquareMetre> {
        self.quantity()
    }

    #[inline]
    fn km(self) -> Quantity<Self, Kilo<Metre>> {
        self.quantity()
    }

    #[inline]
    fn h(self) -> Quantity<Self, Hour> {
        self.quantity()
    }

    #[inline]
    fn min_(self) -> Quantity<Self, Minute> {
        self.quantity()
    }

    #[inline]
    fn kmph(self) -> Quantity<Self, KiloMetrePerHour> {
        self.quantity()
    }

    #[inline]
    fn dm(self) -> Quantity<Self, Deci<Metre>> {
        self.quantity()
    }

    // TODO: other shortcuts
}

// Signed
impl IntExt for i8 {}
impl IntExt for i16 {}
impl IntExt for i32 {}
impl IntExt for i64 {}
impl IntExt for isize {}

// Unsigned
impl IntExt for u8 {}
impl IntExt for u16 {}
impl IntExt for u32 {}
impl IntExt for u64 {}
impl IntExt for usize {}

// Float
impl IntExt for f32 {}
impl IntExt for f64 {}

// TODO i/u128 and BigInt support?
