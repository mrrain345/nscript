use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::relational_operator;

use super::op5_shift::shift_operation;

parser! {
  /// Relational operations (<, >, <=, >=)
  pub fn relational_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      shift_operation(), // allows to nest higher-order operations
      relational_operator().map(|op| move |l, r| {
        match op {
          "<" => Expression::LessThan(Box::new(l), Box::new(r)),
          ">" => Expression::GreaterThan(Box::new(l), Box::new(r)),
          "<=" => Expression::LessOrEqual(Box::new(l), Box::new(r)),
          ">=" => Expression::GreaterOrEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}