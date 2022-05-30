use combine::{Stream, parser::repeat, parser, optional};
use super::{Expression, tokens::*};

parser! {
  /// Syntax:
  /// ```
  /// <identifier> [. <identifier>]*
  pub fn prop_chain[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    ignore_newlines!(
      identifier(), // object name
      optional(
        ignore_newlines!(
          operator(Operator::Dot),
          repeat::sep_by1(
            identifier(), // property name
            operator(Operator::Dot),
          ),
        ).map(|(_, props)| props)
      ),
    )

    .map(|(object, chain): (_, Option<Vec<_>>)| Expression::PropChain {
      object: Box::new(Expression::Identifier(object)),
      chain: chain.unwrap_or_default()
    })
  }
}