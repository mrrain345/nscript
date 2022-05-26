use combine::{Stream, parser::repeat, parser, optional};
use crate::parser::{Expression, tokens::*, operations::operation, expression};

parser! {
  pub fn if_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::If).with((
      operation(), // condition
      punctuator(Punctuator::LeftBrace)
        .with(repeat::sep_end_by(expression(), terminator())) // then
        .skip(punctuator(Punctuator::RightBrace)),
      optional(
        keyword(Keyword::Else)
        .with(punctuator(Punctuator::LeftBrace)
        .with(repeat::sep_end_by(expression(), terminator())) // else
        .skip(punctuator(Punctuator::RightBrace)))
      ),
    ))
    .map(|(condition, then, else_)| Expression::If {
      condition: Box::new(condition),
      then,
      else_: else_.unwrap_or(vec![]),
    })
  }
}