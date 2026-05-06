use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

#[expect(private_bounds)]
pub trait PrimitiveInteger:
    Sealed
    + 'static
    + Send
    + Sync
    + Debug
    + Display
    + Copy
    + Eq
    + Ord
    + Hash
    + Default
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ShlAssign
    + ShrAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + PrimitiveIntegerUtils<Unsigned = Self::UnsignedTy>
    + ThirdParty<Signed = Self::SignedTy, Unsigned = Self::UnsignedTy>
{
    type UnsignedTy: PrimitiveUnsigned;
    type SignedTy: PrimitiveSigned;
}

#[expect(private_bounds)]
pub trait PrimitiveSigned:
    PrimitiveInteger<SignedTy = Self> + Neg<Output = Self> + PrimitiveSignedUtils + ThirdPartySigned
{
}

#[expect(private_bounds)]
pub trait PrimitiveUnsigned:
    PrimitiveInteger<UnsignedTy = Self> + PrimitiveUnsignedUtils + ThirdPartyUnsigned
{
}

pub(crate) trait PrimitiveIntegerUtils {
    type Unsigned;

    fn abs_diff(self, other: Self) -> Self::Unsigned;
}

pub(crate) trait PrimitiveSignedUtils: PrimitiveInteger {
    #[cfg(not(feature = "num-primitive"))]
    fn cast_unsigned(self) -> Self::UnsignedTy;

    #[cfg(not(feature = "num-primitive"))]
    fn unsigned_abs(self) -> Self::UnsignedTy;

    #[cfg(not(feature = "num-primitive"))]
    fn abs(self) -> Self;
}

pub(crate) trait PrimitiveUnsignedUtils: PrimitiveInteger {
    #[cfg(not(feature = "num-primitive"))]
    fn cast_signed(self) -> Self::SignedTy;
}

trait Sealed {}

#[cfg(not(feature = "num-primitive"))]
trait ThirdParty {
    type Signed;
    type Unsigned;
}

#[cfg(feature = "num-primitive")]
trait ThirdParty: num_primitive::PrimitiveInteger {
    type Signed;
    type Unsigned;
}

#[cfg(not(feature = "num-primitive"))]
trait ThirdPartySigned {}

#[cfg(feature = "num-primitive")]
trait ThirdPartySigned:
    PrimitiveInteger + num_primitive::PrimitiveSigned<Unsigned = <Self as ThirdParty>::Unsigned>
{
}

#[cfg(not(feature = "num-primitive"))]
trait ThirdPartyUnsigned {}

#[cfg(feature = "num-primitive")]
trait ThirdPartyUnsigned:
    PrimitiveInteger + num_primitive::PrimitiveUnsigned<Signed = <Self as ThirdParty>::Signed>
{
}

macro_rules! impl_integer {
    ($T:ident, $Signed:ident, $Unsigned:ident) => {
        impl PrimitiveInteger for $T {
            type SignedTy = $Signed;
            type UnsignedTy = $Unsigned;
        }

        impl PrimitiveIntegerUtils for $T {
            type Unsigned = $Unsigned;

            #[inline]
            fn abs_diff(self, other: Self) -> Self::Unsigned {
                self.abs_diff(other)
            }
        }

        impl Sealed for $T {}

        impl ThirdParty for $T {
            type Signed = $Signed;
            type Unsigned = $Unsigned;
        }
    };
}
impl_integer!(i8, i8, u8);
impl_integer!(i16, i16, u16);
impl_integer!(i32, i32, u32);
impl_integer!(i64, i64, u64);
impl_integer!(i128, i128, u128);
impl_integer!(isize, isize, usize);
impl_integer!(u8, i8, u8);
impl_integer!(u16, i16, u16);
impl_integer!(u32, i32, u32);
impl_integer!(u64, i64, u64);
impl_integer!(u128, i128, u128);
impl_integer!(usize, isize, usize);

macro_rules! impl_signed {
    ($T:ident, $Unsigned:ident) => {
        impl PrimitiveSigned for $T {}

        impl PrimitiveSignedUtils for $T {
            #[cfg(not(feature = "num-primitive"))]
            #[inline]
            fn cast_unsigned(self) -> Self::UnsignedTy {
                self.cast_unsigned()
            }

            #[cfg(not(feature = "num-primitive"))]
            #[inline]
            fn unsigned_abs(self) -> Self::UnsignedTy {
                self.unsigned_abs()
            }

            #[cfg(not(feature = "num-primitive"))]
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
        }

        impl ThirdPartySigned for $T {}
    };
}
impl_signed!(i8, u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(i128, u128);
impl_signed!(isize, usize);

macro_rules! impl_unsigned {
    ($T:ident, $Signed:ident) => {
        impl PrimitiveUnsigned for $T {}

        impl PrimitiveUnsignedUtils for $T {
            #[cfg(not(feature = "num-primitive"))]
            #[inline]
            fn cast_signed(self) -> Self::SignedTy {
                self.cast_signed()
            }
        }

        impl ThirdPartyUnsigned for $T {}
    };
}
impl_unsigned!(u8, i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(u128, i128);
impl_unsigned!(usize, isize);

#[cfg(test)]
mod tests {
    use assert_impl_trait::assert_impl;

    use crate::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};

    assert_impl!(
        i8: PrimitiveSigned,
        i16: PrimitiveSigned,
        i32: PrimitiveSigned,
        i64: PrimitiveSigned,
        i128: PrimitiveSigned,
        isize: PrimitiveSigned,
        u8: PrimitiveUnsigned,
        u16: PrimitiveUnsigned,
        u32: PrimitiveUnsigned,
        u64: PrimitiveUnsigned,
        u128: PrimitiveUnsigned,
        usize: PrimitiveUnsigned,

        for<T: PrimitiveSigned> {
            T: PrimitiveInteger<SignedTy = T>,
        }
        for<T: PrimitiveUnsigned> {
            T: PrimitiveInteger<UnsignedTy = T>,
        }
    );

    #[cfg(feature = "num-primitive")]
    assert_impl!(
        for<T: PrimitiveInteger> {
            T: num_primitive::PrimitiveInteger,
        }
        for<T: PrimitiveSigned> {
            T: num_primitive::PrimitiveSigned<Unsigned = T::UnsignedTy>,
        }
        for<T: PrimitiveUnsigned> {
            T: num_primitive::PrimitiveUnsigned<Signed = T::SignedTy>,
        }
    );
}
