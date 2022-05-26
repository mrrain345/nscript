use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, operations::operation, Type};

parser!{
  pub fn var[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::Var).with((
      identifier(), // name
      optional( punctuator(Punctuator::Colon).with(type_()) ), // type
      optional( operator(Operator::Assign).with(operation()) ), // value
    ))
    .map(|(name, type_, value)| Expression::Var {
      name,
      type_: match type_ {
        Some(t) => Some(Type(t)),
        None => None,
      },
      value: match value {
        Some(v) => Some(Box::new(v)),
        None => None,
      },
    })
  }
}