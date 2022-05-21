use combine::{parser::repeat, RangeStream, optional};
use combine::parser;

use crate::parser::expression;
use crate::{parser::{Expression, operations::operation}};
use crate::tokenizer::*;

parser! {
  pub fn if_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("if").with((
      operation(), // condition
      punctuator("{")
        .with(repeat::sep_end_by(expression(), terminator())) // then
        .skip(punctuator("}")),
      optional(
        keyword("else")
        .with(punctuator("{")
        .with(repeat::sep_end_by(expression(), terminator())) // else
        .skip(punctuator("}")))
      ),
    ))
    .map(|(condition, then, else_)| Expression::If {
      condition: Box::new(condition),
      then,
      else_: else_.unwrap_or_else(|| vec![]),
    })
  }
}