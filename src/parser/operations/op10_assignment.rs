use combine::{Stream, parser::repeat, parser, choice};
use crate::parser::{Expression, tokens::*, prop_chain::prop_chain};

use super::op9_logical::logical_and_operation;

parser! {
  /// Assignment operator (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
  fn assignment_operator[I]()(I) -> Operator
  where [ I: Stream<Token=Token> ] {

    choice((
      operator(Operator::Assign),
      operator(Operator::PlusAssign),
      operator(Operator::MinusAssign),
      operator(Operator::PowerAssign),
      operator(Operator::MultiplyAssign),
      operator(Operator::DivideAssign),
      operator(Operator::ModuloAssign),
      operator(Operator::LeftShiftAssign),
      operator(Operator::RightShiftAssign),
      operator(Operator::BitwiseAndAssign),
      operator(Operator::BitwiseXorAssign),
      operator(Operator::BitwiseOrAssign),
    ))
  }
}

// TODO: allows to chain assignment operations
parser! {
  /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
  pub fn assignment_operation[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    (
      prop_chain(),
      assignment_operator(),
      logical_and_operation(), // allows to nest higher-order operations
    ).map(|(ptr, op, value)| {
      match op {
        Operator::Assign => Expression::Assign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::PlusAssign => Expression::AddAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::MinusAssign => Expression::SubAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::PowerAssign => Expression::MulAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::MultiplyAssign => Expression::DivAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::DivideAssign => Expression::ModuloAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::ModuloAssign => Expression::PowerAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::LeftShiftAssign => Expression::LeftShiftAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::RightShiftAssign => Expression::RightShiftAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::BitwiseAndAssign => Expression::BitwiseAndAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::BitwiseXorAssign => Expression::BitwiseXorAssign { ptr: Box::new(ptr), value: Box::new(value) },
        Operator::BitwiseOrAssign => Expression::BitwiseOrAssign { ptr: Box::new(ptr), value: Box::new(value) },
        _ => unreachable!(),
      }
    })
  }
}