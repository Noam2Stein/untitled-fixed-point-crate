#![expect(non_camel_case_types)]

use crate::FixedPoint;

/// An 8-bit signed fixed-point number with 7 fractional bits.
///
/// This has the range `-1..1` and precision `0.0078125`.
pub type i8f7 = FixedPoint<i8, 7>;

/// An 8-bit unsigned fixed-point number with 7 fractional bits.
///
/// This has the range `0..2` and precision `0.0078125`.
pub type u8f7 = FixedPoint<u8, 7>;

/// An 8-bit signed fixed-point number with 8 fractional bits.
///
/// This has the range `-0.5..0.5` and precision `0.00390625`.
pub type i8f8 = FixedPoint<i8, 8>;

/// An 8-bit unsigned fixed-point number with 8 fractional bits.
///
/// This has the range `0..1` and precision `0.00390625`.
pub type u8f8 = FixedPoint<u8, 8>;

/// A 16-bit signed fixed-point number with 8 fractional bits.
///
/// This has the range `-128..128` and precision `0.00390625`.
pub type i16f8 = FixedPoint<i16, 8>;

/// A 16-bit unsigned fixed-point number with 8 fractional bits.
///
/// This has the range `0..256` and precision `0.00390625`.
pub type u16f8 = FixedPoint<u16, 8>;

/// A 16-bit signed fixed-point number with 9 fractional bits.
///
/// This has the range `-64..64` and precision `0.001953125`.
pub type i16f9 = FixedPoint<i16, 9>;

/// A 16-bit unsigned fixed-point number with 9 fractional bits.
///
/// This has the range `0..128` and precision `0.001953125`.
pub type u16f9 = FixedPoint<u16, 9>;

/// A 16-bit signed fixed-point number with 10 fractional bits.
///
/// This has the range `-32..32` and precision `9.765625e-4`.
pub type i16f10 = FixedPoint<i16, 10>;

/// A 16-bit unsigned fixed-point number with 10 fractional bits.
///
/// This has the range `0..64` and precision `9.765625e-4`.
pub type u16f10 = FixedPoint<u16, 10>;

/// A 16-bit signed fixed-point number with 15 fractional bits.
///
/// This has the range `-1..1` and precision `3.0517578124999996e-5`.
pub type i16f15 = FixedPoint<i16, 15>;

/// A 16-bit unsigned fixed-point number with 15 fractional bits.
///
/// This has the range `0..2` and precision `3.0517578124999996e-5`.
pub type u16f15 = FixedPoint<u16, 15>;

/// A 16-bit signed fixed-point number with 16 fractional bits.
///
/// This has the range `-0.5..0.5` and precision `1.5258789062499998e-5`.
pub type i16f16 = FixedPoint<i16, 16>;

/// A 16-bit unsigned fixed-point number with 16 fractional bits.
///
/// This has the range `0..1` and precision `1.5258789062499998e-5`.
pub type u16f16 = FixedPoint<u16, 16>;

/// A 32-bit signed fixed-point number with 8 fractional bits.
///
/// This has the range `-8_388_608..8_388_608` and precision `0.00390625`.
pub type i32f8 = FixedPoint<i32, 8>;

/// A 32-bit unsigned fixed-point number with 8 fractional bits.
///
/// This has the range `0..16_777_216` and precision `0.00390625`.
pub type u32f8 = FixedPoint<u32, 8>;

/// A 32-bit signed fixed-point number with 9 fractional bits.
///
/// This has the range `-4_194_304..4_194_304` and precision `0.001953125`.
pub type i32f9 = FixedPoint<i32, 9>;

/// A 32-bit unsigned fixed-point number with 9 fractional bits.
///
/// This has the range `0..8_388_608` and precision `0.001953125`.
pub type u32f9 = FixedPoint<u32, 9>;

/// A 32-bit signed fixed-point number with 10 fractional bits.
///
/// This has the range `-2_097_152..2_097_152` and precision `9.765625e-4`.
pub type i32f10 = FixedPoint<i32, 10>;

/// A 32-bit unsigned fixed-point number with 10 fractional bits.
///
/// This has the range `0..4_194_304` and precision `9.765625e-4`.
pub type u32f10 = FixedPoint<u32, 10>;

/// A 32-bit signed fixed-point number with 16 fractional bits.
///
/// This has the range `-32_768..32_768` and precision `1.5258789062499998e-5`.
pub type i32f16 = FixedPoint<i32, 16>;

/// A 32-bit unsigned fixed-point number with 16 fractional bits.
///
/// This has the range `0..65_536` and precision `1.5258789062499998e-5`.
pub type u32f16 = FixedPoint<u32, 16>;

/// A 32-bit signed fixed-point number with 24 fractional bits.
///
/// This has the range `-128..128` and precision `5.9604644775390625e-8`.
pub type i32f24 = FixedPoint<i32, 24>;

/// A 32-bit unsigned fixed-point number with 24 fractional bits.
///
/// This has the range `0..256` and precision `5.9604644775390625e-8`.
pub type u32f24 = FixedPoint<u32, 24>;

/// A 32-bit signed fixed-point number with 31 fractional bits.
///
/// This has the range `-1..1` and precision `4.656612873077393e-10`.
pub type i32f31 = FixedPoint<i32, 31>;

/// A 32-bit unsigned fixed-point number with 31 fractional bits.
///
/// This has the range `0..2` and precision `4.656612873077393e-10`.
pub type u32f31 = FixedPoint<u32, 31>;

/// A 32-bit signed fixed-point number with 32 fractional bits.
///
/// This has the range `-0.5..0.5` and precision `2.3283064365386963e-10`.
pub type i32f32 = FixedPoint<i32, 32>;

/// A 32-bit unsigned fixed-point number with 32 fractional bits.
///
/// This has the range `0..1` and precision `2.3283064365386963e-10`.
pub type u32f32 = FixedPoint<u32, 32>;

/// A 64-bit signed fixed-point number with 8 fractional bits.
///
/// This has the range `-36_028_797_018_963_970..36_028_797_018_963_970` and
/// precision `0.00390625`.
pub type i64f8 = FixedPoint<i64, 8>;

/// A 64-bit unsigned fixed-point number with 8 fractional bits.
///
/// This has the range `0..72_057_594_037_927_940` and precision `0.00390625`.
pub type u64f8 = FixedPoint<u64, 8>;

/// A 64-bit signed fixed-point number with 9 fractional bits.
///
/// This has the range `-18_014_398_509_481_984..18_014_398_509_481_984` and
/// precision `0.001953125`.
pub type i64f9 = FixedPoint<i64, 9>;

/// A 64-bit unsigned fixed-point number with 9 fractional bits.
///
/// This has the range `0..36_028_797_018_963_970` and precision `0.001953125`.
pub type u64f9 = FixedPoint<u64, 9>;

/// A 64-bit signed fixed-point number with 10 fractional bits.
///
/// This has the range `-9_007_199_254_740_992..9_007_199_254_740_992` and
/// precision `9.765625e-4`.
pub type i64f10 = FixedPoint<i64, 10>;

/// A 64-bit unsigned fixed-point number with 10 fractional bits.
///
/// This has the range `0..18_014_398_509_481_984` and precision `9.765625e-4`.
pub type u64f10 = FixedPoint<u64, 10>;

/// A 64-bit signed fixed-point number with 16 fractional bits.
///
/// This has the range `-140_737_488_355_328..140_737_488_355_328` and precision
/// `1.5258789062499998e-5`.
pub type i64f16 = FixedPoint<i64, 16>;

/// A 64-bit unsigned fixed-point number with 16 fractional bits.
///
/// This has the range `0..281_474_976_710_656` and precision
/// `1.5258789062499998e-5`.
pub type u64f16 = FixedPoint<u64, 16>;

/// A 64-bit signed fixed-point number with 24 fractional bits.
///
/// This has the range `-549_755_813_888..549_755_813_888` and precision
/// `5.9604644775390625e-8`.
pub type i64f24 = FixedPoint<i64, 24>;

/// A 64-bit unsigned fixed-point number with 24 fractional bits.
///
/// This has the range `0..1_099_511_627_776` and precision
/// `5.9604644775390625e-8`.
pub type u64f24 = FixedPoint<u64, 24>;

/// A 64-bit signed fixed-point number with 32 fractional bits.
///
/// This has the range `-2_147_483_648..2_147_483_648` and precision
/// `2.3283064365386963e-10`.
pub type i64f32 = FixedPoint<i64, 32>;

/// A 64-bit unsigned fixed-point number with 32 fractional bits.
///
/// This has the range `0..4_294_967_296` and precision
/// `2.3283064365386963e-10`.
pub type u64f32 = FixedPoint<u64, 32>;

/// A 64-bit signed fixed-point number with 40 fractional bits.
///
/// This has the range `-8_388_608..8_388_608` and precision
/// `9.094947017729282e-13`.
pub type i64f40 = FixedPoint<i64, 40>;

/// A 64-bit unsigned fixed-point number with 40 fractional bits.
///
/// This has the range `0..16_777_216` and precision `9.094947017729282e-13`.
pub type u64f40 = FixedPoint<u64, 40>;

/// A 64-bit signed fixed-point number with 48 fractional bits.
///
/// This has the range `-32_768..32_768` and precision `3.5527136788005005e-15`.
pub type i64f48 = FixedPoint<i64, 48>;

/// A 64-bit unsigned fixed-point number with 48 fractional bits.
///
/// This has the range `0..65_536` and precision `3.5527136788005005e-15`.
pub type u64f48 = FixedPoint<u64, 48>;

/// A 64-bit signed fixed-point number with 56 fractional bits.
///
/// This has the range `-128..128` and precision `1.3877787807814457e-17`.
pub type i64f56 = FixedPoint<i64, 56>;

/// A 64-bit unsigned fixed-point number with 56 fractional bits.
///
/// This has the range `0..256` and precision `1.3877787807814457e-17`.
pub type u64f56 = FixedPoint<u64, 56>;

/// A 64-bit signed fixed-point number with 63 fractional bits.
///
/// This has the range `-1..1` and precision `1.0842021724855044e-19`.
pub type i64f63 = FixedPoint<i64, 63>;

/// A 64-bit unsigned fixed-point number with 63 fractional bits.
///
/// This has the range `0..2` and precision `1.0842021724855044e-19`.
pub type u64f63 = FixedPoint<u64, 63>;

/// A 64-bit signed fixed-point number with 64 fractional bits.
///
/// This has the range `-0.5..0.5` and precision `5.421010862427522e-20`.
pub type i64f64 = FixedPoint<i64, 64>;

/// A 64-bit unsigned fixed-point number with 64 fractional bits.
///
/// This has the range `0..1` and precision `5.421010862427522e-20`.
pub type u64f64 = FixedPoint<u64, 64>;

/// A 128-bit signed fixed-point number with 8 fractional bits.
///
/// This has the range
/// `-664_613_997_892_458_000_000_000_000_000_000_000..664_613_997_892_458_000_000_000_000_000_000_000` and precision `0.00390625`.
pub type i128f8 = FixedPoint<i128, 8>;

/// A 128-bit unsigned fixed-point number with 8 fractional bits.
///
/// This has the range `0..1_329_227_995_784_916_000_000_000_000_000_000_000`
/// and precision `0.00390625`.
pub type u128f8 = FixedPoint<u128, 8>;

/// A 128-bit signed fixed-point number with 9 fractional bits.
///
/// This has the range
/// `-332_306_998_946_229_000_000_000_000_000_000_000..332_306_998_946_229_000_000_000_000_000_000_000` and precision `0.001953125`.
pub type i128f9 = FixedPoint<i128, 9>;

/// A 128-bit unsigned fixed-point number with 9 fractional bits.
///
/// This has the range `0..664_613_997_892_458_000_000_000_000_000_000_000` and
/// precision `0.001953125`.
pub type u128f9 = FixedPoint<u128, 9>;

/// A 128-bit signed fixed-point number with 10 fractional bits.
///
/// This has the range
/// `-166_153_499_473_114_500_000_000_000_000_000_000..166_153_499_473_114_500_000_000_000_000_000_000` and precision `9.765625e-4`.
pub type i128f10 = FixedPoint<i128, 10>;

/// A 128-bit unsigned fixed-point number with 10 fractional bits.
///
/// This has the range `0..332_306_998_946_229_000_000_000_000_000_000_000` and
/// precision `9.765625e-4`.
pub type u128f10 = FixedPoint<u128, 10>;

/// A 128-bit signed fixed-point number with 16 fractional bits.
///
/// This has the range
/// `-2_596_148_429_267_414_000_000_000_000_000_000..2_596_148_429_267_414_000_000_000_000_000_000` and precision `1.5258789062499998e-5`.
pub type i128f16 = FixedPoint<i128, 16>;

/// A 128-bit unsigned fixed-point number with 16 fractional bits.
///
/// This has the range `0..5_192_296_858_534_828_000_000_000_000_000_000` and
/// precision `1.5258789062499998e-5`.
pub type u128f16 = FixedPoint<u128, 16>;

/// A 128-bit signed fixed-point number with 24 fractional bits.
///
/// This has the range
/// `-10_141_204_801_825_835_000_000_000_000_000..10_141_204_801_825_835_000_000_000_000_000` and precision `5.9604644775390625e-8`.
pub type i128f24 = FixedPoint<i128, 24>;

/// A 128-bit unsigned fixed-point number with 24 fractional bits.
///
/// This has the range `0..20_282_409_603_651_670_000_000_000_000_000` and
/// precision `5.9604644775390625e-8`.
pub type u128f24 = FixedPoint<u128, 24>;

/// A 128-bit signed fixed-point number with 32 fractional bits.
///
/// This has the range
/// `-39_614_081_257_132_170_000_000_000_000..39_614_081_257_132_170_000_000_000_000` and precision `2.3283064365386963e-10`.
pub type i128f32 = FixedPoint<i128, 32>;

/// A 128-bit unsigned fixed-point number with 32 fractional bits.
///
/// This has the range `0..79_228_162_514_264_340_000_000_000_000` and precision
/// `2.3283064365386963e-10`.
pub type u128f32 = FixedPoint<u128, 32>;

/// A 128-bit signed fixed-point number with 40 fractional bits.
///
/// This has the range
/// `-154_742_504_910_672_530_000_000_000..154_742_504_910_672_530_000_000_000`
/// and precision `9.094947017729282e-13`.
pub type i128f40 = FixedPoint<i128, 40>;

/// A 128-bit unsigned fixed-point number with 40 fractional bits.
///
/// This has the range `0..309_485_009_821_345_100_000_000_000` and precision
/// `9.094947017729282e-13`.
pub type u128f40 = FixedPoint<u128, 40>;

/// A 128-bit signed fixed-point number with 48 fractional bits.
///
/// This has the range
/// `-604_462_909_807_314_600_000_000..604_462_909_807_314_600_000_000` and
/// precision `3.5527136788005005e-15`.
pub type i128f48 = FixedPoint<i128, 48>;

/// A 128-bit unsigned fixed-point number with 48 fractional bits.
///
/// This has the range `0..1_208_925_819_614_629_200_000_000` and precision
/// `3.5527136788005005e-15`.
pub type u128f48 = FixedPoint<u128, 48>;

/// A 128-bit signed fixed-point number with 56 fractional bits.
///
/// This has the range
/// `-2_361_183_241_434_822_600_000..2_361_183_241_434_822_600_000` and
/// precision `1.3877787807814457e-17`.
pub type i128f56 = FixedPoint<i128, 56>;

/// A 128-bit unsigned fixed-point number with 56 fractional bits.
///
/// This has the range `0..4_722_366_482_869_645_000_000` and precision
/// `1.3877787807814457e-17`.
pub type u128f56 = FixedPoint<u128, 56>;

/// A 128-bit signed fixed-point number with 64 fractional bits.
///
/// This has the range `-9_223_372_036_854_776_000..9_223_372_036_854_776_000`
/// and precision `5.421010862427522e-20`.
pub type i128f64 = FixedPoint<i128, 64>;

/// A 128-bit unsigned fixed-point number with 64 fractional bits.
///
/// This has the range `0..18_446_744_073_709_552_000` and precision
/// `5.421010862427522e-20`.
pub type u128f64 = FixedPoint<u128, 64>;

/// A 128-bit signed fixed-point number with 72 fractional bits.
///
/// This has the range `-36_028_797_018_963_970..36_028_797_018_963_970` and
/// precision `2.117582368135751e-22`.
pub type i128f72 = FixedPoint<i128, 72>;

/// A 128-bit unsigned fixed-point number with 72 fractional bits.
///
/// This has the range `0..72_057_594_037_927_940` and precision
/// `2.117582368135751e-22`.
pub type u128f72 = FixedPoint<u128, 72>;

/// A 128-bit signed fixed-point number with 80 fractional bits.
///
/// This has the range `-140_737_488_355_328..140_737_488_355_328` and precision
/// `8.271806125530276e-25`.
pub type i128f80 = FixedPoint<i128, 80>;

/// A 128-bit unsigned fixed-point number with 80 fractional bits.
///
/// This has the range `0..281_474_976_710_656` and precision
/// `8.271806125530276e-25`.
pub type u128f80 = FixedPoint<u128, 80>;

/// A 128-bit signed fixed-point number with 88 fractional bits.
///
/// This has the range `-549_755_813_888..549_755_813_888` and precision
/// `3.2311742677852644e-27`.
pub type i128f88 = FixedPoint<i128, 88>;

/// A 128-bit unsigned fixed-point number with 88 fractional bits.
///
/// This has the range `0..1_099_511_627_776` and precision
/// `3.2311742677852644e-27`.
pub type u128f88 = FixedPoint<u128, 88>;

/// A 128-bit signed fixed-point number with 96 fractional bits.
///
/// This has the range `-2_147_483_648..2_147_483_648` and precision
/// `1.262177448353619e-29`.
pub type i128f96 = FixedPoint<i128, 96>;

/// A 128-bit unsigned fixed-point number with 96 fractional bits.
///
/// This has the range `0..4_294_967_296` and precision `1.262177448353619e-29`.
pub type u128f96 = FixedPoint<u128, 96>;

/// A 128-bit signed fixed-point number with 104 fractional bits.
///
/// This has the range `-8_388_608..8_388_608` and precision
/// `4.930380657631323e-32`.
pub type i128f104 = FixedPoint<i128, 104>;

/// A 128-bit unsigned fixed-point number with 104 fractional bits.
///
/// This has the range `0..16_777_216` and precision `4.930380657631323e-32`.
pub type u128f104 = FixedPoint<u128, 104>;

/// A 128-bit signed fixed-point number with 112 fractional bits.
///
/// This has the range `-32_768..32_768` and precision `1.925929944387236e-34`.
pub type i128f112 = FixedPoint<i128, 112>;

/// A 128-bit unsigned fixed-point number with 112 fractional bits.
///
/// This has the range `0..65_536` and precision `1.925929944387236e-34`.
pub type u128f112 = FixedPoint<u128, 112>;

/// A 128-bit signed fixed-point number with 120 fractional bits.
///
/// This has the range `-128..128` and precision `7.52316384526264e-37`.
pub type i128f120 = FixedPoint<i128, 120>;

/// A 128-bit unsigned fixed-point number with 120 fractional bits.
///
/// This has the range `0..256` and precision `7.52316384526264e-37`.
pub type u128f120 = FixedPoint<u128, 120>;

/// A 128-bit signed fixed-point number with 127 fractional bits.
///
/// This has the range `-1..1` and precision `5.877471754111438e-39`.
pub type i128f127 = FixedPoint<i128, 127>;

/// A 128-bit unsigned fixed-point number with 127 fractional bits.
///
/// This has the range `0..2` and precision `5.877471754111438e-39`.
pub type u128f127 = FixedPoint<u128, 127>;

/// A 128-bit signed fixed-point number with 128 fractional bits.
///
/// This has the range `-0.5..0.5` and precision `2.938735877055719e-39`.
pub type i128f128 = FixedPoint<i128, 128>;

/// A 128-bit unsigned fixed-point number with 128 fractional bits.
///
/// This has the range `0..1` and precision `2.938735877055719e-39`.
pub type u128f128 = FixedPoint<u128, 128>;

#[cfg(test)]
mod tests {
    use crate::{FixedPoint, i32f8, i32f16, i64f8, i64f16, u32f8, u32f16, u64f8, u64f16};

    #[test]
    fn test_aliases() {
        let _: u32f8 = FixedPoint::<u32, 8>::default();
        let _: i32f8 = FixedPoint::<i32, 8>::default();
        let _: u32f16 = FixedPoint::<u32, 16>::default();
        let _: i32f16 = FixedPoint::<i32, 16>::default();
        let _: u64f8 = FixedPoint::<u64, 8>::default();
        let _: i64f8 = FixedPoint::<i64, 8>::default();
        let _: u64f16 = FixedPoint::<u64, 16>::default();
        let _: i64f16 = FixedPoint::<i64, 16>::default();
    }
}
