use combine::{Stream, parser::repeat, parser, optional};
use crate::parser::{Expression, tokens::*, expression};

parser! {
  /// Syntax:
  /// ```
  /// fn <identifier> ( <identifier> : <type> , )* [-> <type>] { <expression>* }
  /// ```
  pub fn fn_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      keyword(Keyword::Fn),
      identifier(), // name
      punctuator(Punctuator::LeftParen),
      repeat::sep_end_by( // args
        ignore_newlines!(
          identifier(), // arg name
          punctuator(Punctuator::Colon),
          type_(), // arg type
        ).map(|(name, _, type_)| (name, type_)),
        punctuator(Punctuator::Comma),
      ),
      punctuator(Punctuator::RightParen),
      optional(  // return_type
        ignore_newlines!(
          operator(Operator::Arrow),
          type_(),
        ).map(|(_, type_)| type_)
      ),
      punctuator(Punctuator::LeftBrace),
      repeat::sep_end_by(expression(), terminator()), // body
      punctuator(Punctuator::RightBrace),
    )
    
    .map(|(_, name, _, args, _, return_type, _, body, _)| Expression::Fn {
      name,
      args,
      return_type: return_type.unwrap_or("null".to_string()),
      body,
    })
  }
}