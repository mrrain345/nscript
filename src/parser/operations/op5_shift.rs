use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::shift_operator;

use super::op4_additive::additive_operation;

parser! {
  /// Shift operations (<<, >>)
  pub fn shift_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      additive_operation(), // allows to nest higher-order operations
      shift_operator().map(|op| move |l, r| {
        match op {
          "<<" => Expression::LeftShift(Box::new(l), Box::new(r)),
          ">>" => Expression::RightShift(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}