use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, expression};

parser!{
  /// Syntax:
  /// ```
  /// var <identifier> [: <type>] [= <expression>]
  pub fn var[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::Var),
      identifier(), // name
      optional( // type
        ignore_newlines!(
          punctuator(Punctuator::Colon),
          type_(),
        ).map(|(_, type_)| type_)
      ),
      optional( // value
        ignore_newlines!(
          operator(Operator::Assign),
          expression(),
        ).map(|(_, value)| value)
      )
    )

    .map(|(_, name, type_, value)| Expression::Var {
      name,
      type_,
      value: value.map(|value| Box::new(value)),
    })
  }
}