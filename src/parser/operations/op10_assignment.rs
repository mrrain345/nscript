use combine::RangeStream;
use combine::parser;

use crate::parser::expressions::Expression;
use crate::tokenizer::{assignment_operator, identifier};

use super::op9_logical::logical_and_operation;

// TODO: allows to chain assignment operations
parser! {
  /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
  pub fn assignment_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(),
      assignment_operator(),
      logical_and_operation(), // allows to nest higher-order operations
    ).map(|(name, op, value)| {
      match op {
        "=" => Expression::Assign { name, value: Box::new(value) },
        "+=" => Expression::AddAssign { name, value: Box::new(value) },
        "-=" => Expression::SubAssign { name, value: Box::new(value) },
        "*=" => Expression::MulAssign { name, value: Box::new(value) },
        "/=" => Expression::DivAssign { name, value: Box::new(value) },
        "%=" => Expression::ModuloAssign { name, value: Box::new(value) },
        "**=" => Expression::PowerAssign { name, value: Box::new(value) },
        "<<=" => Expression::LeftShiftAssign { name, value: Box::new(value) },
        ">>=" => Expression::RightShiftAssign { name, value: Box::new(value) },
        "&=" => Expression::BitwiseAndAssign { name, value: Box::new(value) },
        "^=" => Expression::BitwiseXorAssign { name, value: Box::new(value) },
        "|=" => Expression::BitwiseOrAssign { name, value: Box::new(value) },
        _ => unreachable!(),
      }
    })
  }
}