use std::{borrow::Cow, str::FromStr};

use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote_spanned;

#[proc_macro]
pub fn fx(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ParseTokensResult {
        literal,
        crate_path,
    } = match parse_tokens(tokens.into()) {
        Ok(ok) => ok,
        Err(error) => return error.into(),
    };

    let literal_str = literal.to_string();
    let ParseLiteralResult {
        number_str,
        inner_type,
        fract_bits,
    } = match parse_literal(&literal_str, literal.span()) {
        Ok(ok) => ok,
        Err(error) => return error.into(),
    };

    let ParseNumberResult {
        numerator,
        denominator,
    } = match parse_number(number_str, literal.span()) {
        Ok(ok) => ok,
        Err(error) => return error.into(),
    };

    if let Some(fract_bits) = fract_bits {
        let bits = match calculate_bits(
            numerator,
            denominator,
            fract_bits,
            number_str,
            literal.span(),
        ) {
            Ok(ok) => ok,
            Err(error) => return error.into(),
        };

        let mut fract_bits = Literal::u32_unsuffixed(fract_bits);
        fract_bits.set_span(literal.span());
        let mut bits = Literal::u128_unsuffixed(bits);
        bits.set_span(literal.span());

        if let Some(inner_type) = inner_type {
            quote_spanned! {
                literal.span() =>
                #crate_path::FixedPoint::<#inner_type, #fract_bits>::from_bits(#bits)
            }
        } else {
            quote_spanned! {
                literal.span() =>
                #crate_path::FixedPoint::<_, #fract_bits>::generic_from_literal_bits(#bits)
            }
        }
    } else {
        let mut numerator = Literal::u128_unsuffixed(numerator);
        numerator.set_span(literal.span());
        let mut denominator = Literal::u128_unsuffixed(denominator);
        denominator.set_span(literal.span());

        if let Some(inner_type) = inner_type {
            quote_spanned! {
                literal.span() =>
                #crate_path::FixedPoint::<#inner_type, _>::from_literal_fraction(
                    #numerator,
                    #denominator,
                )
            }
        } else {
            quote_spanned! {
                literal.span() =>
                #crate_path::FixedPoint::<_, _>::generic_from_literal_fraction(
                    #numerator,
                    #denominator,
                )
            }
        }
    }
    .into()
}

struct ParseTokensResult {
    literal: Literal,
    crate_path: TokenStream,
}

fn parse_tokens(tokens: TokenStream) -> Result<ParseTokensResult, TokenStream> {
    let mut tokens = tokens.into_iter();

    let literal = match tokens.next() {
        Some(TokenTree::Literal(literal)) => literal,
        Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
            return Err(compile_error(
                "negative sign should be outside. `fx!(-...)` should be `-fx!(...)",
                punct.span(),
            ));
        }
        Some(token) => {
            return Err(compile_error("expected a literal", token.span()));
        }
        None => return Err(compile_error("expected a literal", Span::call_site())),
    };

    let crate_path = tokens.collect::<TokenStream>();

    Ok(ParseTokensResult {
        literal,
        crate_path,
    })
}

struct ParseLiteralResult<'a> {
    number_str: &'a str,
    inner_type: Option<Ident>,
    fract_bits: Option<u32>,
}

fn parse_literal(str: &str, span: Span) -> Result<ParseLiteralResult<'_>, TokenStream> {
    let (number_str, suffix_str) = str.split_at(str.find(char::is_alphabetic).unwrap_or(str.len()));

    let (inner_type_str, fract_bits_str) = suffix_str.split_once('f').unwrap_or((suffix_str, ""));
    let fract_bits_str = if fract_bits_str.contains('_') {
        Cow::Owned(fract_bits_str.replace('_', ""))
    } else {
        Cow::Borrowed(fract_bits_str)
    };

    let inner_type = (!inner_type_str.is_empty())
        .then_some(inner_type_str)
        .map(|inner_type_str| Ident::new(inner_type_str, span));

    let fract_bits = if !fract_bits_str.is_empty() {
        let Ok(fract_bits) = u32::from_str(&fract_bits_str) else {
            return Err(compile_error(
                format!("invalid suffix `{suffix_str}` for fixed-point number literal"),
                span,
            ));
        };

        Some(fract_bits)
    } else {
        None
    };

    Ok(ParseLiteralResult {
        number_str,
        inner_type,
        fract_bits,
    })
}

struct ParseNumberResult {
    numerator: u128,
    denominator: u128,
}

fn parse_number(str: &str, span: Span) -> Result<ParseNumberResult, TokenStream> {
    let (integer_str, fraction_str) = str.split_once('.').unwrap_or((str, ""));

    let integer_str = integer_str.trim_start_matches('0');
    let fraction_str = fraction_str.trim_end_matches('0');

    let integer = parse_u128(integer_str, str, span)?;
    let fraction = parse_u128(fraction_str, str, span)?;

    let denominator_exponent = fraction_str.chars().filter(|&char| char != '_').count() as u32;
    let denominator = 10u128
        .checked_pow(denominator_exponent)
        .ok_or_else(|| compile_error(format!("literal `{str}` out of range for parser"), span))?;

    let numerator = integer
        .checked_mul(denominator)
        .and_then(|scaled_integer| fraction.checked_add(scaled_integer))
        .ok_or_else(|| compile_error(format!("literal `{str}` out of range for parser"), span))?;

    Ok(ParseNumberResult {
        numerator,
        denominator,
    })
}

fn calculate_bits(
    numerator: u128,
    denominator: u128,
    fract_bits: u32,
    number_str: &str,
    span: Span,
) -> Result<u128, TokenStream> {
    2u128
        .checked_pow(fract_bits)
        .and_then(|fract_bits_scale| numerator.checked_mul(fract_bits_scale))
        .and_then(|scaled_numerator| scaled_numerator.checked_add(denominator >> 1))
        .and_then(|scaled_numerator| scaled_numerator.checked_div(denominator))
        .ok_or_else(|| {
            compile_error(
                format!("literal `{number_str}` out of range for parser"),
                span,
            )
        })
}

fn parse_u128(str: &str, number_str: &str, span: Span) -> Result<u128, TokenStream> {
    let mut result = 0u128;

    for char in str.chars() {
        match char {
            '_' => {}
            '0'..='9' => {
                let digit = char as u128 - '0' as u128;
                result = result
                    .checked_mul(10)
                    .and_then(|result| result.checked_add(digit))
                    .ok_or_else(|| {
                        compile_error(format!("literal `{str}` out of range for parser"), span)
                    })?;
            }
            _ => {
                return Err(compile_error(
                    format!("invalid fixed-point number literal `{number_str}`"),
                    span,
                ));
            }
        }
    }

    Ok(result)
}

fn compile_error(message: impl AsRef<str>, span: Span) -> TokenStream {
    let mut message = Literal::string(message.as_ref());
    message.set_span(span);

    quote_spanned! { span => compile_error!(#message) }
}
