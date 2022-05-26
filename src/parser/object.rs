use combine::{Stream, parser::repeat, parser, between};
use super::{Expression, tokens::*, operations::operation, PropertyValue};

parser! {
  pub fn object[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    (
      identifier(), // class name
      between(
        punctuator(Punctuator::LeftBracket),
        punctuator(Punctuator::RightBracket),
        repeat::sep_end_by(
          identifier().skip(operator(Operator::Assign)).and(operation()), // properties
          punctuator(Punctuator::Comma),
        )
      ),
    )
    .map(|(name, props): (_, Vec<(_, _)>)| {
      let props = props
        .into_iter()
        .map(|(name, value)| PropertyValue { name, type_: None, value, modifiers: None })
        .collect();

      Expression::Object { name, properties: props }
    })
  }
}