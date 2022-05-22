use combine::RangeStream;
use combine::parser;

use crate::parser::Expression;
use crate::parser::prop_chain::prop_chain;
use crate::tokenizer::{assignment_operator};

use super::op9_logical::logical_and_operation;

// TODO: allows to chain assignment operations
parser! {
  /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
  pub fn assignment_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      prop_chain(),
      assignment_operator(),
      logical_and_operation(), // allows to nest higher-order operations
    ).map(|(ptr, op, value)| {
      match op {
        "=" => Expression::Assign { ptr: Box::new(ptr), value: Box::new(value) },
        "+=" => Expression::AddAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "-=" => Expression::SubAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "*=" => Expression::MulAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "/=" => Expression::DivAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "%=" => Expression::ModuloAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "**=" => Expression::PowerAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "<<=" => Expression::LeftShiftAssign { ptr: Box::new(ptr), value: Box::new(value) },
        ">>=" => Expression::RightShiftAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "&=" => Expression::BitwiseAndAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "^=" => Expression::BitwiseXorAssign { ptr: Box::new(ptr), value: Box::new(value) },
        "|=" => Expression::BitwiseOrAssign { ptr: Box::new(ptr), value: Box::new(value) },
        _ => unreachable!(),
      }
    })
  }
}