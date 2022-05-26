use combine::{Stream, parser, choice, attempt, between};
use crate::parser::{Expression, tokens::*};

use self::op9_logical::logical_or_operation;

// Operator precedence
// Category         | Operator    | Associativity
// -----------------+-------------+-----------------
// Unitary          | - + ! ~     | Right
// Power            | **          | Right
// Multiplicative   | * / %       | Left
// Additive         | + -         | Left
// Shift            | << >>       | Left
// Relational       | < > <= =>   | Left
// Equality         | == !=       | Left
// Bitwise AND      | &           | Left
// Bitwise XOR      | ^           | Left
// Bitwise OR       | |           | Left
// Logical AND      | &&          | Left
// Logical OR       | ||          | Left
// Assignment       | = *= /= **= | Right
//                  | += -= &= %=
//                  | >>= <<= |= ^=

mod op1_unitary;
mod op2_power;
mod op3_multiplicative;
mod op4_additive;
mod op5_shift;
mod op6_relational;
mod op7_equality;
mod op8_bitwise;
mod op9_logical;
mod op10_assignment;

pub use op10_assignment::assignment_operation;

use super::{call::call, object::object, prop_chain::prop_chain};


parser! {
  pub fn operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {
    
    logical_or_operation()
  }
}

parser! {
  fn parenthesis[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    between(
      punctuator(Punctuator::LeftParen),
      punctuator(Punctuator::RightParen),
      operation(),
    )
  }
}

parser! {
  fn literal[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {
    
    choice((
      number().map(|v| Expression::Number(v)),
      integer().map(|v| Expression::Integer(v)),
      string().map(|v| Expression::String(v)),
      boolean().map(|v| Expression::Boolean(v)),
      null().map(|_| Expression::Null),
    ))
  }
}

parser! {
  fn highest_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    choice((
      attempt(call()),
      attempt(object()),
      parenthesis(),
      literal(),
      attempt(prop_chain()),
      // identifier(),
    ))
  }
}