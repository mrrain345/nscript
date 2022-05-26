use combine::{Stream, parser::repeat, parser, between};
use super::{Expression, tokens::*, operations::operation};

parser! {
  pub fn call[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    (
      identifier(), // name
      between(
        punctuator(Punctuator::LeftParen),
        punctuator(Punctuator::RightParen),
        repeat::sep_end_by(operation(), punctuator(Punctuator::Comma)) // args
      ),
    )
    .map(|(name, args)| Expression::Call { name, args })
  }
}