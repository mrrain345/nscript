use combine::parser::{Parser, choice, repeat, sequence};
use combine::parser::combinator::{FnOpaque, no_partial};
use combine::stream::RangeStream;
use combine::opaque;

use super::expressions::Expression;
use super::tokenizer::*;

pub fn operation<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    additive_operation().expected("operation")
  ))
}

pub fn parenthesis<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    sequence::between(
      punctuator("("),
      punctuator(")"),
      operation(),
    )
  ))
}

pub fn literal<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    choice::choice((
      number().map(|n| Expression::Number(n)),
      integer().map(|i| Expression::Integer(i)),
      string().map(|s| Expression::String(s)),
      boolean().map(|b| Expression::Bool(b)),
      null().map(|_| Expression::Null),
    ))
  ))
}

/// Power operation (**)
pub fn power_operation<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    repeat::chainr1(
      choice::choice(( // allows to nest highest-order operations
        parenthesis(),
        literal(),
        identifier().map(|s| Expression::Identifier(s)),
      )),
      operator("**").map(|_| move |l, r| Expression::Pow(Box::new(l), Box::new(r)) )
    )
  ))
}

// Multiplicative operations (*, /, %)
pub fn multiplicative_operation<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

    opaque!(no_partial(
      repeat::chainl1(
        power_operation(), // allows to nest higher-order operations
        multiplicative_operator().map(|op| move |l, r| {
          match op {
            '*' => Expression::Mul(Box::new(l), Box::new(r)),
            '/' => Expression::Div(Box::new(l), Box::new(r)),
            '%' => Expression::Mod(Box::new(l), Box::new(r)),
            _ => unreachable!(),
          }
        })
      )
    ))
}

/// Additive operations (+, -)
pub fn additive_operation<'src, I>() -> FnOpaque<I, Expression>
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  opaque!(no_partial(
    repeat::chainl1(
      multiplicative_operation(), // allows to nest higher-order operations
      additive_operator().map(|op| move |l, r| {
        match op {
          '+' => Expression::Add(Box::new(l), Box::new(r)),
          '-' => Expression::Sub(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  ))
}