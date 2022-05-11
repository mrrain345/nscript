use combine::parser::{choice, repeat, sequence};
use combine::stream::RangeStream;
use combine::{parser, attempt, between};

use self::op9_logical::logical_or_operation;

use super::call::call;
use super::expressions::Expression;
use super::tokenizer::*;

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


parser! {
  pub fn operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    choice::choice((
      attempt(call()),
      logical_or_operation(),
    ))
  }
}

parser! {
  fn parenthesis['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    sequence::between(
      punctuator("("),
      punctuator(")"),
      operation(),
    )
  }
}

parser! {
  fn literal['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    choice::choice((
      number().map(|v| Expression::Number(v)),
      integer().map(|v| Expression::Integer(v)),
      string().map(|v| Expression::String(v)),
      boolean().map(|v| Expression::Boolean(v)),
      null().map(|_| Expression::Null),
    ))
  }
}

parser! {
  fn highest_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      parenthesis(),
      literal(),
      identifier().map(|s| Expression::Identifier(s)),
    ))
  }
}