use combine::{parser, RangeStream, optional};

use crate::{parser::{expressions::Expression, operations::operation}, nscript::Type};
use crate::tokenizer::*;

parser!{
  pub fn var['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("var").with((
      identifier(), // name
      optional( punctuator(":").with(type_()) ), // type
      optional( punctuator("=").with(operation()) ), // value
    ))
    .map(|(name, type_, value)| Expression::Var {
      name,
      type_: match type_ {
        Some(t) => Some(Type(t)),
        None => None,
      },
      value: match value {
        Some(v) => Some(Box::new(v)),
        None => None,
      },
    })
  }
}