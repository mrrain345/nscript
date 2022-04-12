use super::Token;
use combine::parser::{char, choice, range};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::error::ParseError;

pub fn literals<'a, I>() -> impl Parser<I, Output=Token<'a>>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let number =
    range::take_while1(|c: char| c.is_digit(10))
    .and(choice::optional(
      char::char('.')
      .and(range::take_while(|c: char| c.is_digit(10)))
    ))
    .map(|(a, o): (&str, Option<(char, &str)>)| {
      if let Some((_, b)) = o {
        let n = a.to_string() + "." + &b;
        Token::Number(n.parse::<f64>().unwrap())
      } else {
        Token::Integer(a.parse::<i32>().unwrap())
      }
    });

  let string =
    char::char('"')
    .and(range::take_while(|c: char| c != '"'))
    .and(char::char('"'))
    .map(|((_, s), _)| Token::String(s));

  return choice::choice((
    number,
    string,
    range::range("true").map(|_| Token::Boolean(true)),
    range::range("false").map(|_| Token::Boolean(false)),
    range::range("null").map(|_| Token::Null),
  ));
}