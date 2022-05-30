use combine::{Stream, parser::repeat, parser, optional};
use super::{Expression, tokens::*, PropertyValue, expression};

parser! {
  /// Syntax:
  /// ```
  /// <class_name> { <property_value>* }
  /// <property_value> ::= <identifier> [: <type>] = <expression> ,
  pub fn object[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      identifier(), // class name
      punctuator(Punctuator::LeftBrace),
      repeat::sep_end_by( // properties
        ignore_newlines!(
          identifier(), // property name
          optional( // type
            ignore_newlines!(
              punctuator(Punctuator::Colon),
              type_(),
            ).map(|(_, type_)| type_)
          ),
          operator(Operator::Assign),
          expression(), // value
        ),
        punctuator(Punctuator::Comma).skip(newline())
      ),
      punctuator(Punctuator::RightBrace),
    )

    .map(|(name, _, props, _): (_, _, Vec<_>, _)| {
      let props = props
        .into_iter()
        .map(|(name, type_, _, value)| PropertyValue { name, type_, value, modifiers: None })
        .collect();

      Expression::Object { name, properties: props }
    })
  }
}