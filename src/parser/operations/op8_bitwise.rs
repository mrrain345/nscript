use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::{bitwise_and_operator, bitwise_xor_operator, bitwise_or_operator};

use super::op7_equality::equality_operation;

parser! {
  /// Bitwise AND operation (&)
  pub fn bitwise_and_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      equality_operation(), // allows to nest higher-order operations
      bitwise_and_operator().map(|_| move |l, r| {
        Expression::BitwiseAnd(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Bitwise XOR operation (^)
  pub fn bitwise_xor_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_and_operation(), // allows to nest higher-order operations
      bitwise_xor_operator().map(|_| move |l, r| {
        Expression::BitwiseXor(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Bitwise OR operation (|)
  pub fn bitwise_or_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_xor_operation(), // allows to nest higher-order operations
      bitwise_or_operator().map(|_| move |l, r| {
        Expression::BitwiseOr(Box::new(l), Box::new(r))
      })
    )
  }
}