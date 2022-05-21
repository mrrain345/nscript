use combine::{parser::choice, RangeStream};
use combine::parser;

use crate::parser::Expression;
use crate::tokenizer::unitary_operator;

use super::{highest_operation, operation};

parser! {
  /// Unitary operation (- + ! ~)
  pub fn unitary_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    choice::choice((
      highest_operation(), // allows to nest highest-order operations
      unitary_operator().and(operation()).map(|(op, expr)| {
        match op {
          '-' => Expression::Minus(Box::new(expr)),
          '+' => Expression::Plus(Box::new(expr)),
          '!' => Expression::Not(Box::new(expr)),
          '~' => Expression::BitwiseNot(Box::new(expr)),
          _ => unreachable!(),
        }
      }),
    ))
  }
}