use combine::{Stream, parser::repeat, parser};
use crate::parser::{Expression, tokens::*};

use super::op1_unitary::unitary_operation;

parser! {
  /// Power operator (**)
  fn power_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    operator(Operator::Power)
  }
}

parser! {
  /// Power operation (**)
  pub fn power_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {
    
    repeat::chainr1(
      unitary_operation(), // allow to nest higher-order operations
      power_operator().map(|_| move |l, r| {
        Expression::Power(Box::new(l), Box::new(r))
      })
    )
  }
}