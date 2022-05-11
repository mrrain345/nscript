use combine::{parser, RangeStream, optional};

use crate::{parser::{expressions::Expression, operations::operation}, nscript::Type};
use crate::tokenizer::*;

parser!{
  pub fn let_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("let").with((
      identifier(), // name
      optional( punctuator(":").with(type_()) ) // type
      .skip(punctuator("=")),
      operation(), // value
    ))
    .map(|(name, type_, value)| Expression::Let {
      name,
      type_: match type_ {
        Some(t) => Some(Type(t)),
        None => None,
      },
      value: Box::new(value),
    })
  }
}