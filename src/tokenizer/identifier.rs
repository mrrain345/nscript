use combine::parser::{Parser, combinator, char, token, repeat};
use combine::stream::RangeStream;
use super::{separator, ignore_spaces};

pub fn identifier<'src, I>() -> impl Parser<I, Output=String> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  ignore_spaces(
    combinator::recognize((
      token::satisfy(|c: char| c.is_alphabetic() || c == '_'),
      repeat::skip_many(token::satisfy(|c: char| c.is_alphanumeric() || c == '_')),
      separator(),
    ))
  )
}