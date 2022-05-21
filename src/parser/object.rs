use combine::{parser::repeat, RangeStream};
use combine::{parser, between};

use crate::parser::{Expression,PropertyValue};
use crate::tokenizer::{identifier, punctuator};

use super::operations::operation;

parser! {
  pub fn object['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(), // class name
      between(
        punctuator("{"),
        punctuator("}"),
        repeat::sep_end_by(
          identifier().skip(punctuator("=")).and(operation()), // properties
          punctuator(",")
        )
      ),
    )
    .map(|(name, props): (_, Vec<(_, _)>)| {
      let props = props
        .into_iter()
        .map(|(name, value)| PropertyValue { name, type_: None, value, modifiers: None })
        .collect();

      Expression::Object { name, properties: props }
    })
  }
}