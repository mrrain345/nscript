use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::power_operator;

use super::op1_unitary::unitary_operation;

parser! {
  /// Power operation (**)
  pub fn power_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    repeat::chainr1(
      unitary_operation(), // allow to nest higher-order operations
      power_operator().map(|_| move |l, r| {
        Expression::Power(Box::new(l), Box::new(r))
      })
    )
  }
}