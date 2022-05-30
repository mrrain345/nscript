use combine::{Stream, parser, optional};
use crate::parser::{Expression, tokens::*, expression};

parser!{
  /// Syntax:
  /// ```
  /// let <identifier> [: <type>] = <expression>
  pub fn let_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::Let),
      identifier(), // name
      optional( // type
        ignore_newlines!(
          punctuator(Punctuator::Colon),
          type_(),
        ).map(|(_, type_)| type_)
      ),
      operator(Operator::Assign),
      expression(), // value
    )
    .map(|(_, name, type_, _, value)| Expression::Let {
      name,
      type_,
      value: Box::new(value),
    })
  }
}