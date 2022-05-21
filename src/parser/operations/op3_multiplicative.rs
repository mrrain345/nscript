use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::multiplicative_operator;

use super::op2_power::power_operation;

parser! {
  /// Multiplicative operations (*, /, %)
  pub fn multiplicative_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      power_operation(), // allows to nest higher-order operations
      multiplicative_operator().map(|op| move |l, r| {
        match op {
          '*' => Expression::Mul(Box::new(l), Box::new(r)),
          '/' => Expression::Div(Box::new(l), Box::new(r)),
          '%' => Expression::Modulo(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}