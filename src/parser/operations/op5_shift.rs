use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::op4_additive::additive_operation;

parser! {
  /// Shift operator (<<, >>)
  fn shift_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::LeftShift),
      operator(Operator::RightShift),
    ))
  }
}

parser! {
  /// Shift operations (<<, >>)
  pub fn shift_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    repeat::chainl1(
      additive_operation(), // allows to nest higher-order operations
      shift_operator().map(|op| move |l, r| {
        match op {
          Operator::LeftShift => Expression::LeftShift(Box::new(l), Box::new(r)),
          Operator::RightShift => Expression::RightShift(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}