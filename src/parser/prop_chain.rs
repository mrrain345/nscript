use combine::{parser::repeat, RangeStream};
use combine::{parser, optional};

use crate::parser::Expression;
use crate::tokenizer::{punctuator, self};

use super::identifier::identifier;
use super::operations::operation;

parser! {
  pub fn prop_chain['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(), // object name
      optional(
        punctuator(".")
        .with(repeat::sep_by1(
          tokenizer::identifier(), // property name
          punctuator(".")
        ))
      )
    )
    .map(|(object, chain): (_, Option<Vec<_>>)| Expression::PropChain { object: Box::new(object), chain: chain.unwrap_or_default() })
  }
}