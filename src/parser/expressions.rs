use combine::parser::{Parser, choice, combinator};
use combine::parser::combinator::{FnOpaque, no_partial};
use combine::stream::RangeStream;
use combine::opaque;

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
  Index(Box<Expression>, Box<Expression>),
  Return(Box<Expression>),

  Let(Let),
  Var(Var),
  Fn(Fn),
  Assign(Assign),
  If(If),
  While(While),
  For(For),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Integer(i32),
  Number(f64),
  String(String),
  Bool(bool),
  Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Let {
  pub name: String,
  pub type_: Option<Type>,
  pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
  pub name: String,
  pub type_: Option<Type>,
  pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fn {
  pub name: String,
  pub params: ParamsList,
  pub return_type: Option<Type>,
  pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
  pub name: String,
  pub value: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
  pub condition: Box<Expression>,
  pub then: Block,
  pub else_: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
  pub condition: Box<Expression>,
  pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct For {
  pub index: Option<String>,
  pub index_type: Option<Type>,
  pub iter: String,
  pub iter_type: Option<Type>,
  pub generator: Box<Expression>,
  pub body: Block,
}



pub fn expression<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    choice::choice((
      combinator::attempt(operation()),
    ))
  ))
}