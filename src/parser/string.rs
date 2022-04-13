use super::Token;
use combine::parser::{char, repeat, token, choice};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::error::ParseError;

pub fn escaped<'a, I>() -> impl Parser<I, Output=char>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  return (char::char('\\'), token::any())
    .map(|(_, c)| match c {
      '"' => '"',
      '\'' => '\'',
      '\\' => '\\',
      'n' => '\n',
      'r' => '\r',
      't' => '\t',
      _ => c,
    });
}

pub fn string<'a, I>() -> impl Parser<I, Output=Token>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let string1 = (
    char::char('"'),
    repeat::many(
      choice::choice((
        escaped(),
        token::satisfy(|c| c != '"'),
      ))
    ),
    char::char('"'),
  ).map(|(_, s, _)| Token::String(s));

  let string2 = (
    char::char('\''),
    repeat::many(
      choice::choice((
        escaped(),
        token::satisfy(|c| c != '\''),
      ))
    ),
    char::char('\''),
  ).map(|(_, s, _)| Token::String(s));

  return choice::choice((string1, string2));
}