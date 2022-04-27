use combine::parser::{Parser, char, range};
use combine::stream::RangeStream;

use super::{ignore_spaces};

pub fn keyword<'src, I>(key: &'static str) -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  ignore_spaces(
    range::range(key).map(|_| ())
  )
}