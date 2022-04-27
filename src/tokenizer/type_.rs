use combine::parser::{Parser, combinator, char, token, repeat};
use combine::stream::RangeStream;
use super::{separator};

pub fn type_<'src, I>() -> impl Parser<I, Output=String> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  combinator::recognize((
    token::satisfy(|c: char| c.is_alphabetic() || c == '_'),
    repeat::skip_many(token::satisfy(|c: char| c.is_alphanumeric() || c == '_')),
    separator(),
  )).skip(char::spaces())
}