use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::additive_operator;

use super::op3_multiplicative::multiplicative_operation;

parser! {
  /// Additive operations (+, -)
  pub fn additive_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    repeat::chainl1(
      multiplicative_operation(), // allows to nest higher-order operations
      additive_operator().map(|op| move |l, r| {
        match op {
          '+' => Expression::Add(Box::new(l), Box::new(r)),
          '-' => Expression::Sub(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}