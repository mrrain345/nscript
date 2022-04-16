use combine::parser::{Parser, char, repeat, token, sequence};
use combine::stream::RangeStream;
use super::separator;

pub fn string<'src, I>() -> impl Parser<I, Output=String> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // Parser for escaped characters.
  let escaped = || (char::char('\\'), token::any())
    .map(|(_, c)| match c {
      'n' => '\n',
      'r' => '\r',
      't' => '\t',
      _ => c,
    });


  // Parser for a string literal.
  let string = |term| sequence::between(char::char(term), char::char(term),
    repeat::many(
      escaped().or(token::satisfy(move |c| c != term))
    ),
  );

  string('"').or(string('\''))
    .skip(separator())
    .expected("string")
}