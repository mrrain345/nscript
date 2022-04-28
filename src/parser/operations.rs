use combine::parser::{choice, repeat, sequence};
use combine::stream::RangeStream;
use combine::parser;

use super::expressions::Expression;
use super::tokenizer::*;


parser!{
  pub fn operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    additive_operation()
  }
}

parser!{
  pub fn parenthesis['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    sequence::between(
      punctuator("("),
      punctuator(")"),
      operation(),
    )
  }
}

parser!{
  pub fn literal['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    choice::choice((
      number().map(|n| Expression::Number(n)),
      integer().map(|i| Expression::Integer(i)),
      string().map(|s| Expression::String(s)),
      boolean().map(|b| Expression::Bool(b)),
      null().map(|_| Expression::Null),
    ))
  }
}

parser!{
  /// Power operation (**)
  pub fn power_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    repeat::chainr1(
      choice::choice(( // allows to nest highest-order operations
        parenthesis(),
        literal(),
        identifier().map(|s| Expression::Identifier(s)),
      )),
      operator("**").map(|_| move |l, r| Expression::Power(Box::new(l), Box::new(r)) )
    )
  }
}

parser!{
  /// Multiplicative operations (*, /, %)
  pub fn multiplicative_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      power_operation(), // allows to nest higher-order operations
      multiplicative_operator().map(|op| move |l, r| {
        match op {
          '*' => Expression::Mul(Box::new(l), Box::new(r)),
          '/' => Expression::Div(Box::new(l), Box::new(r)),
          '%' => Expression::Modulo(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser!{
  /// Additive operations (+, -)
  pub fn additive_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
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
  }
}