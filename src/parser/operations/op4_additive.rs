use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::op3_multiplicative::multiplicative_operation;

parser! {
  /// Additive operator (+, -)
  fn additive_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::Plus),
      operator(Operator::Minus),
    ))
  }
}

parser! {
  /// Additive operations (+, -)
  pub fn additive_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {
    
    repeat::chainl1(
      multiplicative_operation(), // allows to nest higher-order operations
      additive_operator().map(|op| move |l, r| {
        match op {
          Operator::Plus => Expression::Add(Box::new(l), Box::new(r)),
          Operator::Minus => Expression::Sub(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}