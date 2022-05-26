use std::fs::File;
use std::io::Write;

use combine::{choice, attempt};
use combine::parser::{char, repeat};
use combine::parser::{Parser, EasyParser};
use combine::stream::RangeStream;
use combine::stream::position::Stream;

mod tokens;
mod identifier;
mod keyword;
mod operator;
mod punctuator;
mod spaces;
mod literals;
mod type_;

pub use tokens::Token;
pub use keyword::*;
pub use operator::*;
pub use punctuator::*;
pub use spaces::{ignore_spaces, separator};


pub fn parse<'src>(source: &'src str) -> Vec<Token> {

  let tokens = tokenize().easy_parse(Stream::new(source));

  match tokens {
    Ok((tokens, stream)) => {
      // Check if there are any tokens left.
      if !stream.input.is_empty() {
        eprintln!("[Tokenizer]");
        eprintln!("Unparsed: {}", stream.input);
        eprintln!("Position: [line: {}, column: {}]\n", stream.positioner.line, stream.positioner.column);
      }

      // Save tokens to a file
      let mut file = File::create("target/output.tokens").unwrap();
      for token in &tokens {
        file.write(format!("{:?}\n", token).as_bytes()).unwrap();
      }

      tokens
    },
    Err(errors) => {
      eprintln!("[Tokenizer error]");
      eprintln!("position: [{}:{}]", errors.position.line, errors.position.column);
      eprintln!("error: {:?}\n", errors.errors);
      panic!();
    },
  }
}

fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Token>> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let token = choice((
    literals::literals(),
    keyword::keyword(),
    operator::operator(),
    punctuator::punctuator(),
    identifier::identifier(),
    attempt(spaces::terminator()),
    spaces::newline(),
  ));

  return repeat::sep_end_by(token, spaces::spaces());
}