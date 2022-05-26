use combine::{Stream, parser::repeat, parser, optional, between};
use crate::parser::{Expression, tokens::*, Type, expression};

parser! {
  pub fn fn_[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    keyword(Keyword::Fn).with((
      identifier().skip(punctuator(Punctuator::LeftParen)), // name
      repeat::sep_end_by( // args
        identifier()
          .skip(punctuator(Punctuator::Colon))
          .and(type_())
          .map(|(name, type_)| (name, Type(type_))),
        punctuator(Punctuator::Comma),
      )
      .skip(punctuator(Punctuator::RightBrace)),
      optional(operator(Operator::Arrow).with(type_())) // return type
        .map(|type_| type_.map(Type)),
      between (
        punctuator(Punctuator::LeftBrace),
        punctuator(Punctuator::RightBrace),
        repeat::sep_end_by(expression(), terminator())
      ) // body
    ))
    .map(|(name, args, type_, body)| Expression::Fn {
      name,
      args,
      return_type: type_.unwrap_or(Type("null".to_string())),
      body,
    })
  }
}