use std::fs::File;
use std::io::Write;
use std::vec;

use combine::easy::Errors;
use combine::parser::{EasyParser, Parser, char, repeat};
use combine::stream::position::{Stream as PositionStream, SourcePosition};
use combine::stream::RangeStream;

use crate::tokenizer::terminator;
use super::tokenizer;

mod expressions;
mod operations;
mod statements;
mod call;
mod object;
mod prop_chain;
mod identifier;

pub use expressions::{expression, Expression, Property, PropertyValue};

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
        println!("{:?}\n", stream.positioner);
      }

      // Save AST to file.
      let mut ast_file = File::create("target/output.ast").unwrap();
      for expr in &expressions {
        ast_file.write(format_ast(expr).as_bytes()).unwrap();
      }

      // Return the expressions.
      Ok(expressions)
    },
    Err(errors) => {
      Err(errors)
    }
  }
}

fn format_intent(output: &mut Vec<char>, indent: usize) {
  output.push('\n');
  for _ in 0..indent {
    output.push(' ');
    output.push(' ');
  }
}

/// Format an AST into a string with indentation.
fn format_ast(expr: &Expression) -> String {
  let str = format!("{:?}", expr);
  let mut indent = 0;
  let mut output = vec![];
  
  for c in str.chars() {
    match c {
      '{' | '[' => {
        if c != '[' { output.push(' '); }
        output.push(c);
        indent += 1;
        format_intent(&mut output, indent);
      },
      '}' | ']' => {
        indent -= 1;
        format_intent(&mut output, indent);
        output.push(c);
      },
      ',' => {
        output.push(c);
        format_intent(&mut output, indent);
      },
      ' ' => {},
      ':' => {
        output.push(c);
        output.push(' ');
      },
      '\n' => {
        format_intent(&mut output, indent);
      },
      _ => {
        output.push(c);
      }
    }
  }

  output.push('\n');
  output.push('\n');
  output.into_iter().collect()
}