use combine::{Stream, parser::repeat, parser};
use super::{Expression, tokens::*, expression};

parser! {
  /// Syntax:
  /// ```
  /// <identifier> ( <expression_comma>* )
  /// <expression_comma> ::= <expression> ,
  /// ```
  pub fn call[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      identifier(), // name
      punctuator(Punctuator::LeftParen),
      repeat::sep_end_by(expression(), punctuator(Punctuator::Comma)),
      punctuator(Punctuator::RightParen),
    )
    
    .map(|(name, _, args, _)| Expression::Call { name, args })
  }
}