use typenum::{Integer, Unsigned};

/// Type that can be created from compile-time integer
pub trait FromInteger {
    /// Create self from `I`
    fn from_integer<I: Integer>() -> Self;
}

/// Type that can be created from compile-time unsigned integer
pub trait FromUnsigned {
    /// Create self from `U`
    fn from_unsigned<U: Unsigned>() -> Self;
}

macro_rules! impls_int {
    (
        $( $Int:ident => $Const:ident),+ $(,)?
    ) => {
        $(
            impl FromInteger for $Int {
                #[inline]
                fn from_integer<I: Integer>() -> Self {
                    I::$Const
                }
            }
        )+
    };
}

macro_rules! impls_uint {
    (
        $( $Int:ident => $Const:ident),+ $(,)?
    ) => {
        $(
            impl FromUnsigned for $Int {
                #[inline]
                fn from_unsigned<I: Unsigned>() -> Self {
                    I::$Const
                }
            }
        )+
    };
}

impls_int! {
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
}

impls_uint! {
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,

    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
}
