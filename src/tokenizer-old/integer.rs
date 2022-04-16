use super::Token;
use combine::parser::{Parser, range, choice, combinator, char};
use combine::stream::RangeStream;
use combine::error::ParseError;

pub fn integer<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src,
  I::Error: ParseError<I::Token, I::Range, I::Position> {

  // Parse a hexadecimal integer.
  let hex = range::range("0x")
    .with(range::take_while1(|c: char| c.is_digit(16)))
    .map(|s| Token::Integer(i32::from_str_radix(s, 16).unwrap()) );

  // Parse a octal integer.
  let octal = range::range("0o")
    .with(range::take_while1(|c: char| c.is_digit(8)))
    .map(|s| Token::Integer(i32::from_str_radix(s, 8).unwrap()) );

  // Parse a binary integer.
  let binary = range::range("0b")
    .with(range::take_while1(|c: char| c.is_digit(2)))
    .map(|s| Token::Integer(i32::from_str_radix(s, 2).unwrap()) );

  // Parse a decimal integer.
  let dec = range::take_while1(|c: char| c.is_digit(10))
    .map(|s: &str| Token::Integer(s.parse::<i32>().unwrap()) );

  
  choice::choice((hex, octal, binary, dec))
    .skip(combinator::not_followed_by(char::alpha_num()))
}