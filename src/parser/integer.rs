use super::Token;
use combine::parser::{char, range, choice, token};
use combine::parser::Parser;
use combine::stream::RangeStream;

pub fn integer<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // Parse a non decimal integer. (hex, octal, binary)
  let nondecimal = token::token('0').with(
      token::satisfy(|c| c == 'x' || c == 'o' || c == 'b')
      .map(|c| match c {
        'x' => 16,
        'o' => 8,
        'b' => 2,
        _ => unreachable!(),
      })
    ).then(|radix| {
      range::take_while1(move |c: char| c.is_digit(radix))
      .map(move |s: &str| Token::Integer(i32::from_str_radix(s, radix).unwrap()))
    }
  );

  // Parse a decimal integer.
  let decimal = range::take_while1(|c: char| c.is_digit(10)).map(|s: &str| {
    Token::Integer(s.parse::<i32>().unwrap())
  });

  return choice::choice((
    nondecimal,
    decimal,
  ));
}