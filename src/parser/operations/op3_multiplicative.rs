use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::op2_power::power_operation;

parser! {
  /// Multiplicative operator (*, /, %)
  fn multiplicative_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::Multiply),
      operator(Operator::Divide),
      operator(Operator::Modulo),
    ))
  }
}

parser! {
  /// Multiplicative operations (*, /, %)
  pub fn multiplicative_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    repeat::chainl1(
      power_operation(), // allows to nest higher-order operations
      multiplicative_operator().map(|op| move |l, r| {
        match op {
          Operator::Multiply => Expression::Mul(Box::new(l), Box::new(r)),
          Operator::Divide => Expression::Div(Box::new(l), Box::new(r)),
          Operator::Modulo => Expression::Modulo(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}