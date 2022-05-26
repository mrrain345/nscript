use combine::parser::{Parser, range};
use combine::stream::RangeStream;

use crate::tokenizer::{separator, ignore_spaces, Token};

pub fn null<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range("null").with(separator())
      .map(|_| Token::Null)
  )
}