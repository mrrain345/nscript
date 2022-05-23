use combine::parser::{choice, combinator};
use combine::stream::RangeStream;
use combine::parser;

use crate::nscript::Type;

use super::operations::{operation, assignment_operation};
use super::statements::statement;

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

  Assign { ptr: Box<Expression>, value: Box<Expression> },
  AddAssign { ptr: Box<Expression>, value: Box<Expression> },
  SubAssign { ptr: Box<Expression>, value: Box<Expression> },
  MulAssign { ptr: Box<Expression>, value: Box<Expression> },
  DivAssign { ptr: Box<Expression>, value: Box<Expression> },
  ModuloAssign { ptr: Box<Expression>, value: Box<Expression> },
  PowerAssign { ptr: Box<Expression>, value: Box<Expression> },
  BitwiseAndAssign { ptr: Box<Expression>, value: Box<Expression> },
  BitwiseOrAssign { ptr: Box<Expression>, value: Box<Expression> },
  BitwiseXorAssign { ptr: Box<Expression>, value: Box<Expression> },
  LeftShiftAssign { ptr: Box<Expression>, value: Box<Expression> },
  RightShiftAssign { ptr: Box<Expression>, value: Box<Expression> },

  Fn { name: String, args: Vec<(String, Type)>, return_type: Type, body: Vec<Expression> },
  Call { name: String, args: Vec<Expression> },
  Return(Box<Expression>),

  If { condition: Box<Expression>, then: Vec<Expression>, else_: Vec<Expression> },
  // While { condition: Box<Expression>, body: Vec<Expression> },
  // For { index: Option<String>, index_type: Option<Type>, iter: String, iter_type: Option<Type>, generator: Box<Expression>, body: Vec<Expression> },
  // Break,
  // Continue,

  Class { name: String, properties: Vec<Property> },
  Object { name: String, properties: Vec<PropertyValue> },
  PropChain { object: Box<Expression>, chain: Vec<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  pub name: String,
  pub type_: Type,
  pub modifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyValue {
  pub name: String,
  pub type_: Option<Type>,
  pub modifiers: Option<Vec<String>>,
  pub value: Expression,
}


parser!{
  pub fn expression['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      combinator::attempt(statement()),
      combinator::attempt(assignment_operation()),
      combinator::attempt(operation()),
    ))
  }
}