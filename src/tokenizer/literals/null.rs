use combine::parser::{Parser, range};
use combine::stream::RangeStream;

use crate::tokenizer::{separator, ignore_spaces};

pub fn null<'src, I>() -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range("null").with(separator()).expected("null")
  )
}