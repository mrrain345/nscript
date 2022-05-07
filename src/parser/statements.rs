use combine::parser::{repeat, combinator, choice};
use combine::stream::RangeStream;
use combine::{parser, optional};

use crate::nscript::{Type, ParamsList};

use super::expressions::{Expression, expression};
use super::operations::operation;
use super::tokenizer::*;

parser!{
  pub fn statement['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      combinator::attempt(if_()),
      combinator::attempt(let_()),
      combinator::attempt(var()),
      combinator::attempt(fn_()),
      combinator::attempt(return_()),
    ))
  }
}

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

parser! {
  pub fn return_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("return").with(
      optional(operation()), // value
    )
    .map(|value| Expression::Return (
      Box::new(value.unwrap_or(Expression::Null))
    ))
  }
}