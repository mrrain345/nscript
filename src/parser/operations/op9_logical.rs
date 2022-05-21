use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::{logical_and_operator, logical_or_operator};

use super::op8_bitwise::bitwise_or_operation;

parser! {
  /// Logical AND operation (&&)
  pub fn logical_and_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_or_operation(), // allows to nest higher-order operations
      logical_and_operator().map(|_| move |l, r| {
        Expression::And(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Logical OR operation (||)
  pub fn logical_or_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      logical_and_operation(), // allows to nest higher-order operations
      logical_or_operator().map(|_| move |l, r| {
        Expression::Or(Box::new(l), Box::new(r))
      })
    )
  }
}