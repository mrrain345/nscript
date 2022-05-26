use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*};

use super::op6_relational::relational_operation;

parser! {
  /// Equality operator (==, !=)
  fn equality_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::Equal),
      operator(Operator::NotEqual),
    ))
  }
}

parser! {
  /// Equality operations (==, !=)
  pub fn equality_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    repeat::chainl1(
      relational_operation(), // allows to nest higher-order operations
      equality_operator().map(|op| move |l, r| {
        match op {
          Operator::Equal => Expression::Equal(Box::new(l), Box::new(r)),
          Operator::NotEqual => Expression::NotEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}