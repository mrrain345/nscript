use combine::parser::repeat;
use combine::{parser, RangeStream, optional};

use crate::nscript::ParamsList;
use crate::parser::{Expression, expression};
use crate::nscript::Type;
use crate::tokenizer::*;

parser! {
  pub fn fn_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("fn").with((
      identifier().skip(punctuator("(")), // name
      repeat::sep_end_by( // args
        identifier()
          .skip(punctuator(":"))
          .and(type_())
          .map(|(name, type_)| (name, Type(type_))),
        punctuator(","),
      )
      .skip(punctuator(")")),
      optional(punctuator("->") // return type
        .with(type_()))
        .skip(punctuator("{"))
        .map(|type_| type_.map(Type)),
      repeat::sep_end_by(expression(), terminator()) // body
        .skip(punctuator("}")),
    ))
    .map(|(name, args, type_, body)| Expression::Fn {
      name,
      args: ParamsList(args),
      return_type: type_.unwrap_or_else(|| Type("null".to_string())),
      body,
    })
  }
}