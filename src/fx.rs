use core::mem::transmute_copy;

use crate::{FixedPoint, PrimitiveInteger, PrimitiveIntegerUtils};

#[macro_export]
macro_rules! fx {
    ($($tt:tt)*) => {{
        #[allow(clippy::zero_prefixed_literal)]
        #[allow(clippy::inconsistent_digit_grouping)]
        const {
            $crate::fxp_proc_macros::fx!($($tt)* $crate)
        }
    }};
}

impl<T, const FRACT_BITS: u32> FixedPoint<T, FRACT_BITS>
where
    T: PrimitiveInteger,
{
    #[doc(hidden)]
    pub const fn generic_from_literal_fraction(numerator: u128, denominator: u128) -> Self {
        Self::generic_from_literal_bits(
            numerator
                .strict_mul(2u128.strict_pow(FRACT_BITS))
                .strict_add(denominator >> 1)
                .strict_div(denominator),
        )
    }

    #[doc(hidden)]
    pub const fn generic_from_literal_bits(bits: u128) -> Self {
        if bits > T::MAX_AS_U128 {
            panic!("literal out of range");
        };

        // SAFETY: All types have matching sizes and are simple integers.
        Self(unsafe {
            match size_of::<T>() {
                1 => transmute_copy::<u8, T>(&(bits as u8)),
                2 => transmute_copy::<u16, T>(&(bits as u16)),
                4 => transmute_copy::<u32, T>(&(bits as u32)),
                8 => transmute_copy::<u64, T>(&(bits as u64)),
                16 => transmute_copy::<u128, T>(&bits),
                _ => panic!("unexpected integer size"),
            }
        })
    }
}

macro_rules! impl_integer {
    ($T:ident) => {
        impl<const FRACT_BITS: u32> FixedPoint<$T, FRACT_BITS> {
            #[doc(hidden)]
            pub const fn from_literal_fraction(numerator: u128, denominator: u128) -> Self {
                let bits = numerator
                    .strict_mul(2u128.strict_pow(FRACT_BITS))
                    .strict_add(denominator >> 1)
                    .strict_div(denominator);

                if bits > $T::MAX_AS_U128 {
                    panic!("literal out of range");
                }

                Self(bits as $T)
            }
        }
    };
}
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
