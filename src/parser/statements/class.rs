use combine::{Stream, parser::repeat, parser};
use crate::parser::{Expression, tokens::*, Type, Property};

parser! {
  pub fn class[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::Class).with(identifier()) // name
    .skip(punctuator(Punctuator::LeftBrace))
    .and(repeat::sep_end_by( // properties
      (
        identifier(), // name
        punctuator(Punctuator::Colon).with(type_()), // type
      ),
      terminator(),
    ))
    .skip(punctuator(Punctuator::RightBrace))
    .map(|(name, properties): (_, Vec<(_, _)>)| {
      let properties = properties
        .into_iter()
        .map(|(name, type_)| Property { name, type_: Type(type_), modifiers: None })
        .collect();

      Expression::Class { name, properties }
    })
  }
}