use combine::{parser, RangeStream};

use crate::tokenizer;

use super::Expression;

parser! {
  pub fn identifier['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    tokenizer::identifier().map(|s| Expression::Identifier(s))
  }
}