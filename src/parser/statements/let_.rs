use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, operations::operation, Type};

parser!{
  pub fn let_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::Let).with((
      identifier(), // name
      optional(punctuator(Punctuator::Colon).with(type_())) // type
      .skip(operator(Operator::Assign)),
      operation(), // value
    ))
    .map(|(name, type_, value)| Expression::Let {
      name,
      type_: match type_ {
        Some(t) => Some(Type(t)),
        None => None,
      },
      value: Box::new(value),
    })
  }
}