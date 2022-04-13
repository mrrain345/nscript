use super::Token;
use combine::parser::{char, repeat, token, choice};
use combine::parser::Parser;
use combine::stream::RangeStream;

pub fn escaped<'src, I>() -> impl Parser<I, Output=char>
  where I: RangeStream<Token=char, Range=&'src str> {

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

pub fn string<'src, I>() -> impl Parser<I, Output=Token>
  where I: RangeStream<Token=char, Range=&'src str> {

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