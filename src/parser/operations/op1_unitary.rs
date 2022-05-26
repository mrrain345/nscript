use combine::{Stream, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::{highest_operation, operation};

parser! {
  /// Unitary operator (- + ! ~)
  fn unitary_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::Minus),
      operator(Operator::Plus),
      operator(Operator::Not),
      operator(Operator::BitwiseNot),
    ))
  }
}

parser! {
  /// Unitary operation (- + ! ~)
  pub fn unitary_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    choice((
      highest_operation(), // allows to nest highest-order operations
      unitary_operator().and(operation()).map(|(op, expr)| {
        match op {
          Operator::Minus => Expression::Minus(Box::new(expr)),
          Operator::Plus => Expression::Plus(Box::new(expr)),
          Operator::Not => Expression::Not(Box::new(expr)),
          Operator::BitwiseAnd => Expression::BitwiseNot(Box::new(expr)),
          _ => unreachable!(),
        }
      }),
    ))
  }
}