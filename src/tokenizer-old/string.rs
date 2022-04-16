use super::Token;
use combine::parser::{Parser, char, repeat, token, choice, sequence};
use combine::stream::RangeStream;

pub fn string<'src, I>() -> impl Parser<I, Output=Token>
  where I: RangeStream<Token=char, Range=&'src str> {

  // Parser for escaped characters.
  let escaped = || (char::char('\\'), token::any())
    .map(|(_, c)| match c {
      'n' => '\n',
      'r' => '\r',
      't' => '\t',
      _ => c,
    });


  // Parser for a string literal.
  let string = |term| (
    sequence::between(char::char(term), char::char(term),
      repeat::many(
        escaped().or(token::satisfy(move |c| c != term))
      ),
    )
  ).map(|s| Token::String(s));


  choice::or(string('"'), string('\''))
}