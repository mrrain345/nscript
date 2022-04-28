use combine::parser::{choice, combinator, repeat};
use combine::stream::RangeStream;
use combine::{optional, between, parser};

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
  Modulo(Box<Expression>, Box<Expression>),
  Power(Box<Expression>, Box<Expression>),
  // Minus(Box<Expression>),
  // Plus(Box<Expression>),

  // Not(Box<Expression>),
  // And(Box<Expression>, Box<Expression>),
  // Or(Box<Expression>, Box<Expression>),

  // Equal(Box<Expression>, Box<Expression>),
  // NotEqual(Box<Expression>, Box<Expression>),
  // LessThen(Box<Expression>, Box<Expression>),
  // GreaterThen(Box<Expression>, Box<Expression>),
  // LessOrEqual(Box<Expression>, Box<Expression>),
  // GreaterOrEqual(Box<Expression>, Box<Expression>),
  
  Let { name: String, type_: Option<Type>, value: Box<Expression> },
  Var { name: String, type_: Option<Type>, value: Option<Box<Expression>> },
  Assign { name: String, value: Box<Expression> },

  // Fn { name: String, args: ParamsList, return_type: Option<Type>, body: Block },
  Call { name: String, args: Vec<Expression> },
  // Return(Box<Expression>),

  // If { condition: Box<Expression>, then: Block, else_: Option<Block> },
  // While { condition: Box<Expression>, body: Block },
  // For { index: Option<String>, index_type: Option<Type>, iter: String, iter_type: Option<Type>, generator: Box<Expression>, body: Block },
  // Break,
  // Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ParamsList(pub Vec<(String, String)>);

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Expression>);

parser!{
  pub fn expression['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      combinator::attempt(let_()),
      combinator::attempt(var()),
      combinator::attempt(assign()),
      combinator::attempt(call()),
      combinator::attempt(operation()),
    ))
  }
}

parser!{
  pub fn let_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

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
  }
}

parser!{
  pub fn var['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

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
  }
}

parser!{
  pub fn assign['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(),
      punctuator("="),
      operation(),
    )
    .map(|(name, _, value)| Expression::Assign {
      name,
      value: Box::new(value),
    })
  }
}

parser!{
  pub fn call['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(),
      between(
        punctuator("("),punctuator(")"),
        repeat::sep_end_by(operation(), punctuator(","))
      ),
    )
    .map(|(name, args)| Expression::Call { name, args })
  }
}