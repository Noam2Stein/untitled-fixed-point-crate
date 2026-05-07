use fxp::{FixedPoint, PrimitiveInteger, fx, i32f8};

#[test]
fn test() {
    assert_eq!(fx!(1), i32f8::from_bits(256));
    assert_eq!(fx!(1f8), i32f8::from_bits(256));
    assert_eq!(fx!(1i32), i32f8::from_bits(256));
    assert_eq!(fx!(1i32f8), i32f8::from_bits(256));
    assert_eq!(fx!(1_), i32f8::from_bits(256));
    assert_eq!(fx!(1_f8), i32f8::from_bits(256));
    assert_eq!(fx!(1_i32), i32f8::from_bits(256));
    assert_eq!(fx!(1_i32f8), i32f8::from_bits(256));

    assert_eq!(fx!(1.3), i32f8::from_bits(333));
    assert_eq!(fx!(1.3f8), i32f8::from_bits(333));
    assert_eq!(fx!(1.3i32), i32f8::from_bits(333));
    assert_eq!(fx!(1.3i32f8), i32f8::from_bits(333));
    assert_eq!(fx!(1.3_), i32f8::from_bits(333));
    assert_eq!(fx!(1.3_f8), i32f8::from_bits(333));
    assert_eq!(fx!(1.3_i32), i32f8::from_bits(333));
    assert_eq!(fx!(1.3_i32f8), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3f8), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3i32), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3i32f8), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3_), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3_f8), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3_i32), i32f8::from_bits(333));
    assert_eq!(fx!(1_.3_i32f8), i32f8::from_bits(333));

    assert_eq!(fx!(1.2), i32f8::from_bits(307));
    assert_eq!(fx!(1.2f8), i32f8::from_bits(307));
    assert_eq!(fx!(1.2i32), i32f8::from_bits(307));
    assert_eq!(fx!(1.2i32f8), i32f8::from_bits(307));

    test_generic_t(i32f8::from_bits(256), i32f8::from_bits(333));
    test_generic_fract_bits(i32f8::from_bits(256), i32f8::from_bits(333));
    test_generic_t_fract_bits(i32f8::from_bits(256), i32f8::from_bits(333));
}

fn test_generic_t<T>(one: FixedPoint<T, 8>, one_point_three: FixedPoint<T, 8>)
where
    T: PrimitiveInteger,
{
    assert_eq!(fx!(1), one);
    assert_eq!(fx!(1f8), one);
    assert_eq!(fx!(1.3), one_point_three);
    assert_eq!(fx!(1.3f8), one_point_three);
}

fn test_generic_fract_bits<const FRACT_BITS: u32>(
    one: FixedPoint<i32, FRACT_BITS>,
    one_point_three: FixedPoint<i32, FRACT_BITS>,
) {
    assert_eq!(fx!(1), one);
    assert_eq!(fx!(1i32), one);
    assert_eq!(fx!(1.3), one_point_three);
    assert_eq!(fx!(1.3i32), one_point_three);
}

fn test_generic_t_fract_bits<T, const FRACT_BITS: u32>(
    one: FixedPoint<T, FRACT_BITS>,
    one_point_three: FixedPoint<T, FRACT_BITS>,
) where
    T: PrimitiveInteger,
{
    assert_eq!(fx!(1), one);
    assert_eq!(fx!(1.3), one_point_three);
}
