use combine::easy::Errors;
use combine::parser::{EasyParser, Parser, char, repeat};
use combine::stream::position::{Stream as PositionStream, SourcePosition};
use combine::stream::RangeStream;

use crate::tokenizer::terminator;
use super::tokenizer;

pub mod expressions;
mod operations;
mod statements;

use expressions::{expression, Expression};

pub fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Expression>> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // let spaces = range::take_while(|c: char| c.is_whitespace());
  repeat::sep_end_by(expression(), terminator())
}

pub fn parse<'src>(source: &'src str) -> Result<Vec<Expression>, Errors<char, &str, SourcePosition>> {

  let expressions = tokenize().easy_parse(PositionStream::new(source));

  match expressions {
    Ok((expressions, stream)) => {
      if !stream.input.is_empty() {
        println!("Unparsed: {:?}", stream.input);
        println!("{:?}", stream.positioner);
      }

      for expr in &expressions {
        println!("{:?}", expr);
      }

      println!("\nExecution:");
      Ok(expressions)
    },
    Err(errors) => {
      Err(errors)
    }
  }
}