use combine::parser::{Parser, char, range};
use combine::stream::RangeStream;

use super::ignore_spaces;

pub fn punctuator<'src, I>(p: &'static str) -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range(p).map(|_| ())
  )
}

// pub fn any_punctuator<'src, I>() -> impl Parser<I, Output=char> + 'src
//   where I: RangeStream<Token=char, Range=&'src str> + 'src {

//   // Parser for punctuators
//   let punctuator = |c| char::char(c).map(move |_| c);
  
//   ignore_spaces(
//     choice::choice((
//       punctuator('('),
//       punctuator(')'),
//       punctuator('{'),
//       punctuator('}'),
//       punctuator('['),
//       punctuator(']'),
//       punctuator(','),
//       punctuator(':'),
//     ))
//   ).expected("punctuator")
// }