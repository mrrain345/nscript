use combine::parser::{Parser, choice, combinator, repeat};
use combine::parser::combinator::{FnOpaque, no_partial};
use combine::stream::RangeStream;
use combine::{opaque, optional, between};

use crate::tokenizer::*;

use super::operations::operation;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Integer(i32),
  Number(f64),
  String(String),
  Bool(bool),
  Null,
  Identifier(String),
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),
  Mul(Box<Expression>, Box<Expression>),
  Div(Box<Expression>, Box<Expression>),
  Mod(Box<Expression>, Box<Expression>),
  Pow(Box<Expression>, Box<Expression>),
  Neg(Box<Expression>),
  Not(Box<Expression>),
  Array(Vec<Expression>),
  Call(String, Vec<Expression>),
  Return(Box<Expression>),

  Let { name: String, type_: Option<Type>, value: Box<Expression> },
  Var { name: String, type_: Option<Type>, value: Option<Box<Expression>> },
  Fn { name: String, params: ParamsList, return_type: Option<Type>, body: Block },
  Assign { name: String, value: Box<Expression> },
  If { condition: Box<Expression>, then: Block, else_: Option<Block> },
  While { condition: Box<Expression>, body: Block },
  For { index: Option<String>, index_type: Option<Type>, iter: String, iter_type: Option<Type>, generator: Box<Expression>, body: Block },
  Break,
  Continue,
  Block(Block),
  Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ParamsList(pub Vec<(String, String)>);

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Expression>);

pub fn expression<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    choice::choice((
      combinator::attempt(let_()),
      combinator::attempt(var()),
      combinator::attempt(call()),
      combinator::attempt(operation()),
    ))
  ))
}

pub fn let_<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src{

  opaque!(no_partial(
    keyword("let").with((
      identifier(),
      optional( punctuator(":").with(type_()) )
      .skip(punctuator("=")),
      operation(),
    ))
    .map(|(name, type_, value)| Expression::Let {
      name,
      type_: match type_ {
        Some(t) => Some(Type(t)),
        None => None,
      },
      value: Box::new(value),
    })
  ))
}

pub fn var<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    keyword("var").with((
      identifier(),
      optional( punctuator(":").with(type_()) ),
      optional( punctuator("=").with(operation()) ),
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
  ))
}

pub fn call<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    (
      identifier(),
      between(
        punctuator("("),punctuator(")"),
        repeat::sep_end_by(operation(), punctuator(","))
      ),
    )
    .map(|(name, params)| Expression::Call(name, params))
  ))
}