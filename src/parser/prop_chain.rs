use combine::{Stream, parser::repeat, parser, optional};
use super::{Expression, tokens::*};

parser! {
  pub fn prop_chain[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    (
      identifier(), // object name
      optional(
        operator(Operator::Dot)
        .with(repeat::sep_by1(
          identifier(), // property name
          operator(Operator::Dot)
        ))
      )
    )
    .map(|(object, chain): (_, Option<Vec<_>>)| Expression::PropChain {
      object: Box::new(Expression::Identifier(object)),
      chain: chain.unwrap_or_default()
    })
  }
}