#![no_std]

use core::ops::{Add, Neg, Sub};

pub use aliases::*;
pub use primitive_traits::*;

mod aliases;
mod primitive_traits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FixedPoint<T, const FRACT_BITS: u32>(pub T);

impl<T, const FRACT_BITS: u32> FixedPoint<T, FRACT_BITS>
where
    T: PrimitiveInteger,
{
    #[inline]
    pub fn abs_diff(self, other: Self) -> FixedPoint<T::UnsignedTy, FRACT_BITS> {
        FixedPoint(self.0.abs_diff(other.0))
    }
}

impl<T, const FRACT_BITS: u32> FixedPoint<T, FRACT_BITS>
where
    T: PrimitiveSigned,
{
    #[inline]
    pub fn cast_unsigned(self) -> FixedPoint<T::UnsignedTy, FRACT_BITS> {
        FixedPoint(self.0.cast_unsigned())
    }

    #[inline]
    pub fn unsigned_abs(self) -> FixedPoint<T::UnsignedTy, FRACT_BITS> {
        FixedPoint(self.0.unsigned_abs())
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl<T, const FRACT_BITS: u32> FixedPoint<T, FRACT_BITS>
where
    T: PrimitiveUnsigned,
{
    #[inline]
    pub fn cast_signed(self) -> FixedPoint<T::SignedTy, FRACT_BITS> {
        FixedPoint(self.0.cast_signed())
    }
}

impl<T, const FRACT_BITS: u32> Neg for FixedPoint<T, FRACT_BITS>
where
    T: Neg,
{
    type Output = FixedPoint<T::Output, FRACT_BITS>;

    #[inline]
    fn neg(self) -> Self::Output {
        FixedPoint(-self.0)
    }
}

impl<T, const FRACT_BITS: u32> Add for FixedPoint<T, FRACT_BITS>
where
    T: Add,
{
    type Output = FixedPoint<T::Output, FRACT_BITS>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        FixedPoint(self.0 + rhs.0)
    }
}

impl<T, const FRACT_BITS: u32> Sub for FixedPoint<T, FRACT_BITS>
where
    T: Sub,
{
    type Output = FixedPoint<T::Output, FRACT_BITS>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        FixedPoint(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::FixedPoint;

    #[test]
    fn test_abs_diff() {
        assert_eq!(
            FixedPoint::<i32, 8>(-5).abs_diff(FixedPoint(10)),
            FixedPoint(15_u32)
        );
        assert_eq!(
            FixedPoint::<u32, 8>(5).abs_diff(FixedPoint(10)),
            FixedPoint(5_u32)
        );
    }

    #[test]
    fn test_cast_unsigned() {
        assert_eq!(FixedPoint::<i8, 8>(5).cast_unsigned(), FixedPoint(5_u8));
        assert_eq!(FixedPoint::<i8, 8>(-1).cast_unsigned(), FixedPoint(255_u8));
    }

    #[test]
    fn test_unsigned_abs() {
        assert_eq!(FixedPoint::<i32, 8>(5).unsigned_abs(), FixedPoint(5_u32));
        assert_eq!(FixedPoint::<i32, 8>(-5).unsigned_abs(), FixedPoint(5_u32));
        assert_eq!(FixedPoint::<i8, 8>(-128).unsigned_abs(), FixedPoint(128_u8));
    }

    #[test]
    fn test_abs() {
        assert_eq!(FixedPoint::<i32, 8>(5).abs(), FixedPoint(5_i32));
        assert_eq!(FixedPoint::<i32, 8>(-5).abs(), FixedPoint(5_i32));
    }

    #[test]
    fn test_cast_signed() {
        assert_eq!(FixedPoint::<u8, 8>(5).cast_signed(), FixedPoint(5_i8));
        assert_eq!(FixedPoint::<u8, 8>(255).cast_signed(), FixedPoint(-1_i8));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-FixedPoint::<i32, 8>(5), FixedPoint(-5_i32));
        assert_eq!(-FixedPoint::<i32, 8>(-5), FixedPoint(5_i32));
    }

    #[test]
    fn test_add() {
        assert_eq!(FixedPoint::<u32, 8>(5) + FixedPoint(10), FixedPoint(15_u32));
        assert_eq!(FixedPoint::<i32, 8>(5) + FixedPoint(10), FixedPoint(15_i32));
    }

    #[test]
    fn test_sub() {
        assert_eq!(FixedPoint::<u32, 8>(10) - FixedPoint(5), FixedPoint(5_u32));
        assert_eq!(FixedPoint::<i32, 8>(10) - FixedPoint(5), FixedPoint(5_i32));
        assert_eq!(FixedPoint::<i32, 8>(-1) - FixedPoint(5), FixedPoint(-6_i32));
    }
}
