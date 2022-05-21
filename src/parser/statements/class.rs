use combine::{parser::repeat, RangeStream};
use combine::parser;

use crate::{parser::{Expression,Property}, nscript::{Type}};
use crate::tokenizer::*;

parser! {
  pub fn class['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("class").with(identifier()) // name
    .skip(punctuator("{"))
    .and(repeat::sep_end_by( // properties
      (
        identifier(), // name
        punctuator(":").with(type_()), // type
      ),
      punctuator(";")
    ))
    .skip(punctuator("}"))
    .map(|(name, properties): (_, Vec<(_, _)>)| {
      let props = properties
        .into_iter()
        .map(|(name, type_)| Property { name, type_: Type(type_), modifiers: None })
        .collect();

      Expression::Class { name, properties: props }
    })
  }
}