use combine::parser::{Parser, char, range};
use combine::stream::RangeStream;

use super::ignore_spaces;

pub fn operator<'src, I>(op: &'static str) -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range(op).map(|_| ())
  )
}

pub fn multiplicative_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    char::char('*').or(char::char('/')).or(char::char('%'))
  )
}

pub fn additive_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    char::char('+').or(char::char('-'))
  )
}

// pub fn any_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
//   where I: RangeStream<Token=char, Range=&'src str> + 'src {

//   // Parser for operators
//   let operator = |op| range::range(op).map(move |_| op);
    
//   let comparison = choice::choice((
//     operator("=="),
//     operator("!="),
//     operator("<="),
//     operator(">="),
//     operator("<"),
//     operator(">"),
//   ));

//   let other = choice::choice((
//     operator("..."),
//     operator("."),
//     operator("??"),
//     operator("?."),
//     operator("?"),
//     operator("->"),
//     operator("=>"),
//   ));
  
//   let assignment = choice::choice((
//     operator("="),
//     operator("+="),
//     operator("-="),
//     operator("**="),
//     operator("*="),
//     operator("/="),
//     operator("%="),
//     operator("&="),
//     operator("|="),
//     operator("^="),
//   ));

//   let arithmetic = choice::choice((
//     operator("+"),
//     operator("-"),
//     operator("**"),
//     operator("*"),
//     operator("/"),
//     operator("%"),
//   ));

//   let logical = choice::choice((
//     operator("&&"),
//     operator("||"),
//     operator("!"),
//   ));

//   let bitwise = choice::choice((
//     operator("&"),
//     operator("|"),
//     operator("^"),
//     operator("~"),
//   ));


//   ignore_spaces(
//     choice::choice((
//       comparison,
//       other,
//       assignment,
//       arithmetic,
//       logical,
//       bitwise,
//     ))
//   ).expected("operator")
// }