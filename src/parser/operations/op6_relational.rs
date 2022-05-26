use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::op5_shift::shift_operation;

parser! {
  /// Relational operator (<, >, <=, >=)
  fn relational_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::LessThan),
      operator(Operator::GreaterThan),
      operator(Operator::LessOrEqual),
      operator(Operator::GreaterOrEqual),
    ))
  }
}

parser! {
  /// Relational operations (<, >, <=, >=)
  pub fn relational_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    repeat::chainl1(
      shift_operation(), // allows to nest higher-order operations
      relational_operator().map(|op| move |l, r| {
        match op {
          Operator::LessThan => Expression::LessThan(Box::new(l), Box::new(r)),
          Operator::GreaterThan => Expression::GreaterThan(Box::new(l), Box::new(r)),
          Operator::LessOrEqual => Expression::LessOrEqual(Box::new(l), Box::new(r)),
          Operator::GreaterOrEqual => Expression::GreaterOrEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}