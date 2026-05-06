#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"
---

fn main() {
    for size in [8, 16, 32, 64, 128] {
        let mut fract_bits = (8..=size).step_by(8).collect::<Vec<_>>();
        fract_bits.push(size - 1);
        for common_fract_bits in [9, 10] {
            if common_fract_bits <= size {
                fract_bits.push(common_fract_bits);
            }
        }
        fract_bits.sort();

        for fract_bits in fract_bits {
            let precision = 2.0_f64.powi(fract_bits).recip();
            let unsigned_range_end = 2.0_f64.powi(size) * precision;
            let signed_range_end = unsigned_range_end / 2.0;
            let signed_range_start = -signed_range_end;

            let a_or_an = if size == 8 { "An" } else { "A" };
            let precision = format_number(precision);
            let signed_range_start = format_number(signed_range_start);
            let signed_range_end = format_number(signed_range_end);
            let unsigned_range_end = format_number(unsigned_range_end);

            print_documentation(&format!(
                "{} {}",
                format!("{a_or_an} {size}-bit signed fixed-point number"),
                format!("with {fract_bits} fractional bits."),
            ));
            println!("///");
            print_documentation(&format!(
                "{} {}",
                format!("This has the range `{signed_range_start}..{signed_range_end}`"),
                format!("and precision `{precision}`."),
            ));
            println!("pub type i{size}f{fract_bits} = FixedPoint<i{size}, {fract_bits}>;");
            println!();

            print_documentation(&format!(
                "{} {}",
                format!("{a_or_an} {size}-bit unsigned fixed-point number"),
                format!("with {fract_bits} fractional bits."),
            ));
            println!("///");
            print_documentation(&format!(
                "This has the range `0..{unsigned_range_end}` and precision `{precision}`."
            ));
            println!("pub type u{size}f{fract_bits} = FixedPoint<u{size}, {fract_bits}>;");
            println!();
        }
    }
}

fn print_documentation(mut documentation: &str) {
    while !documentation.is_empty() {
        // The `/// ` prefix takes 4 characters out of the expected 80.
        let line_len = if documentation.len() <= 76 {
            documentation.len()
        } else {
            documentation
                .match_indices(' ')
                .filter(|&(space_index, _)| space_index <= 76)
                .last()
                .map(|(space_index, _)| space_index)
                .unwrap_or(documentation.len())
        };

        let line = &documentation[..line_len];
        documentation = &documentation[line_len..].trim();

        println!("/// {line}");
    }
}

fn format_number(x: f64) -> String {
    if x.log10() < -3.0 {
        let log10 = x.abs().log10().floor();
        let scaled = x / 10.0_f64.powf(log10);
        format!("{scaled}e{log10}")
    } else {
        let sign = if x.is_sign_negative() { "-" } else { "" };

        let mut integral = String::new();
        for digit in x.abs().floor().to_string().chars().rev() {
            if integral.len() % 4 == 3 {
                integral = "_".to_string() + &integral;
            }
            integral = digit.to_string() + &integral;
        }

        let fract = x.abs().fract().to_string();
        if let Some(fract) = fract.get(2..) {
            format!("{sign}{integral}.{fract}")
        } else {
            format!("{sign}{integral}")
        }
    }
}
