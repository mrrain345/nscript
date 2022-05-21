use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::equality_operator;

use super::op6_relational::relational_operation;

parser! {
  /// Equality operations (==, !=)
  pub fn equality_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      relational_operation(), // allows to nest higher-order operations
      equality_operator().map(|op| move |l, r| {
        match op {
          "==" => Expression::Equal(Box::new(l), Box::new(r)),
          "!=" => Expression::NotEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}