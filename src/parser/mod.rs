use combine::parser::{EasyParser, Parser, char, repeat};
use combine::stream::position::{Stream as PositionStream};
use combine::stream::RangeStream;

use crate::tokenizer::terminator;
use super::tokenizer;

pub mod expressions;
mod operations;

use expressions::{expression, Expression};

pub fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Expression>> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // let spaces = range::take_while(|c: char| c.is_whitespace());
  repeat::sep_end_by(expression(), terminator())
}

pub fn parse<'src>(source: &'src str) {
  // -> Result<(Vec<Token>, Stream<&str, SourcePosition>), Errors<char, &str, SourcePosition>> {
  
  let tokens = tokenize().easy_parse(PositionStream::new(source));

  match tokens {
    Ok((tokens, stream)) => {
      // println!("{:?}", tokens);
      println!("{:?}\n", stream);

      tokens.iter().for_each(|token| {
        println!("{:?}", token);
      });
      // let result = parse_instruction().easy_parse(PositionStream::new(&tokens[..]));

      // match result {
      //   Ok((instruction, _)) => {
      //     println!("\n[INSTRUCTIONS]:\n{:?}\n\n", instruction);
      //     // println!("[STREAM]:\n{:?}\n\n", stream);
      //   },
      //   Err(errors) => {
      //     println!("\n{:?} {:?}", errors.position, errors.errors);
      //   },
      // }
    },
    Err(errors) => {
      println!("{:?}", errors);
    }
  }
}