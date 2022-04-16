use combine::parser::{Parser, range};
use combine::stream::RangeStream;
use super::separator;

pub fn null<'src, I>() -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  range::range("null").with(separator()).expected("null")
}