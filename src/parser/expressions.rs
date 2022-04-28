use combine::parser::{choice, combinator, repeat};
use combine::stream::RangeStream;
use combine::{optional, between, parser};

use crate::tokenizer::*;

use super::operations::{operation, assignment_operation};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Integer(i32),
  Number(f64),
  String(String),
  Boolean(bool),
  Null,
  Identifier(String),
  
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),
  Mul(Box<Expression>, Box<Expression>),
  Div(Box<Expression>, Box<Expression>),
  Modulo(Box<Expression>, Box<Expression>),
  Power(Box<Expression>, Box<Expression>),
  Minus(Box<Expression>),
  Plus(Box<Expression>),

  BitwiseAnd(Box<Expression>, Box<Expression>),
  BitwiseOr(Box<Expression>, Box<Expression>),
  BitwiseXor(Box<Expression>, Box<Expression>),
  BitwiseNot(Box<Expression>),
  LeftShift(Box<Expression>, Box<Expression>),
  RightShift(Box<Expression>, Box<Expression>),

  And(Box<Expression>, Box<Expression>),
  Or(Box<Expression>, Box<Expression>),
  Not(Box<Expression>),

  LessThan(Box<Expression>, Box<Expression>),
  GreaterThan(Box<Expression>, Box<Expression>),
  LessOrEqual(Box<Expression>, Box<Expression>),
  GreaterOrEqual(Box<Expression>, Box<Expression>),

  Equal(Box<Expression>, Box<Expression>),
  NotEqual(Box<Expression>, Box<Expression>),
  
  Let { name: String, type_: Option<Type>, value: Box<Expression> },
  Var { name: String, type_: Option<Type>, value: Option<Box<Expression>> },

  Assign { name: String, value: Box<Expression> },
  AddAssign { name: String, value: Box<Expression> },
  SubAssign { name: String, value: Box<Expression> },
  MulAssign { name: String, value: Box<Expression> },
  DivAssign { name: String, value: Box<Expression> },
  ModuloAssign { name: String, value: Box<Expression> },
  PowerAssign { name: String, value: Box<Expression> },
  BitwiseAndAssign { name: String, value: Box<Expression> },
  BitwiseOrAssign { name: String, value: Box<Expression> },
  BitwiseXorAssign { name: String, value: Box<Expression> },
  LeftShiftAssign { name: String, value: Box<Expression> },
  RightShiftAssign { name: String, value: Box<Expression> },

  // Fn { name: String, args: ParamsList, return_type: Option<Type>, body: Vec<Expression> },
  Call { name: String, args: Vec<Expression> },
  // Return(Box<Expression>),

  If { condition: Box<Expression>, then: Vec<Expression>, else_: Vec<Expression> },
  // While { condition: Box<Expression>, body: Vec<Expression> },
  // For { index: Option<String>, index_type: Option<Type>, iter: String, iter_type: Option<Type>, generator: Box<Expression>, body: Vec<Expression> },
  // Break,
  // Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ParamsList(pub Vec<(String, String)>);


parser!{
  pub fn expression['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      combinator::attempt(if_()),
      combinator::attempt(let_()),
      combinator::attempt(var()),
      combinator::attempt(call()),
      combinator::attempt(assignment_operation()),
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

parser! {
  pub fn if_['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    keyword("if").with((
      operation(),
      punctuator("{")
        .with(repeat::sep_end_by(expression(), terminator()))
        .skip(punctuator("}")),
      optional(
        keyword("else")
        .with(punctuator("{")
        .with(repeat::sep_end_by(expression(), terminator()))
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