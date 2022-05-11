use combine::{parser::repeat, RangeStream};
use combine::{parser, between};

use crate::parser::expressions::Expression;
use crate::tokenizer::{identifier, punctuator};

use super::operations::operation;

parser! {
  pub fn call['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(), // name
      between(
        punctuator("("),
        punctuator(")"),
        repeat::sep_end_by(operation(), punctuator(",")) // args
      ),
    )
    .map(|(name, args)| Expression::Call { name, args })
  }
}