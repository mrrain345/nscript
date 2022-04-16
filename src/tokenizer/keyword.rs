use combine::parser::{Parser, char, range};
use combine::stream::RangeStream;

use super::separator;

pub fn keyword<'src, I>(key: &'static str) -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  range::range(key).with(separator())
}