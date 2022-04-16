use combine::parser::{Parser, range, choice};
use combine::stream::RangeStream;
use super::separator;

pub fn integer<'src, I>() -> impl Parser<I, Output=i32> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // Parse a hexadecimal integer.
  let hex = range::range("0x")
    .with(range::take_while1(|c: char| c.is_digit(16)))
    .skip(separator())
    .map(|s| i32::from_str_radix(s, 16).unwrap() );

  // Parse a octal integer.
  let octal = range::range("0o")
    .with(range::take_while1(|c: char| c.is_digit(8)))
    .skip(separator())
    .map(|s| i32::from_str_radix(s, 8).unwrap() );

  // Parse a binary integer.
  let binary = range::range("0b")
    .with(range::take_while1(|c: char| c.is_digit(2)))
    .skip(separator())
    .map(|s| i32::from_str_radix(s, 2).unwrap() );

  // Parse a decimal integer.
  let dec = range::take_while1(|c: char| c.is_digit(10))
    .skip(separator())
    .map(|s: &str| s.parse::<i32>().unwrap() );

  
  choice::choice((hex, octal, binary, dec)).expected("integer")
}