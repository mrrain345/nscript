use combine::{Stream, parser::repeat, parser};
use crate::parser::{Expression, tokens::*, Property};

parser! {
  /// Syntax:
  /// ```
  /// class <identifier> { <property>* }
  /// <property> ::= <identifier> : <type> ;
  /// ```
  pub fn class[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::Class),
      identifier(), // name
      punctuator(Punctuator::LeftBrace),
      repeat::sep_end_by( // properties
        ignore_newlines!(
          identifier(), // property name
          punctuator(Punctuator::Colon),
          type_(), // property type
        ),
        terminator(),
      ),
      punctuator(Punctuator::RightBrace),
    )
      
    .map(|(_, name, _, properties, _): (_, _, _, Vec<_>, _)| {
      let properties = properties
        .into_iter()
        .map(|(name, _, type_)| Property { name, type_, modifiers: None })
        .collect();

      Expression::Class { name, properties }
    })
  }
}