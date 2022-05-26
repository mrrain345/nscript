use combine::{Stream, parser, choice};
use crate::parser::{Expression, tokens::*};

mod let_;
mod var;
mod if_;
mod fn_;
mod return_;
mod class;

parser! {
  pub fn statement[I]()(I) -> Expression
  where [ I: Stream<Token=Token> ] {

    choice((
      fn_::fn_(),
      if_::if_(),
      let_::let_(),
      var::var(),
      class::class(),
      return_::return_(),
    ))
  }
}