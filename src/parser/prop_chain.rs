use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::parser::expressions::Expression;
use crate::tokenizer::{identifier, punctuator};

use super::operations::operation;

parser! {
  pub fn prop_chain['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier().skip(punctuator(".")), // object name
      repeat::sep_by1(
        identifier(), // property name
        punctuator("."))
    )
    .map(|(object, chain): (_, Vec<_>)| Expression::PropChain { object: Box::new(Expression::Identifier(object)), chain })
  }
}