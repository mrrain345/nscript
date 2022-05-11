use combine::{parser::{combinator, choice}, RangeStream};
use combine::parser;

use super::expressions::Expression;

mod let_;
mod var;
mod if_;
mod fn_;
mod return_;
mod class;

parser! {
  pub fn statement['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      combinator::attempt(if_::if_()),
      combinator::attempt(let_::let_()),
      combinator::attempt(var::var()),
      combinator::attempt(fn_::fn_()),
      combinator::attempt(class::class()),
      combinator::attempt(return_::return_()),
    ))
  }
}