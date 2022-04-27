use combine::parser::{Parser, range, choice};
use combine::stream::RangeStream;
use crate::tokenizer::{ignore_spaces, separator};

pub fn boolean<'src, I>() -> impl Parser<I, Output=bool> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let true_ = range::range("true")
    .with(separator())
    .map(|_| true);

  let false_ = range::range("false")
    .with(separator())
    .map(|_| false);

  ignore_spaces(
    choice::or(true_, false_).expected("boolean")
  )
}