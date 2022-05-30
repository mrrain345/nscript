use std::fs::File;
use std::io::Write;
use std::vec;

use combine::Positioned;
use combine::easy::Errors;
use combine::stream::position::{Stream as PositionStream};
use combine::parser::{EasyParser, char, repeat};


use self::tokens::terminator;
use super::tokenizer::Token;

mod expressions;
mod operations;
mod statements;
mod call;
mod object;
mod prop_chain;
pub mod tokens;

pub use expressions::{expression, Expression, Property, PropertyValue};
pub use operations::operation;


pub fn parse<'src>(tokens: &[Token]) -> Result<Vec<Expression>, Errors<Token, &[Token], usize>> {

  let parser = || repeat::sep_end_by(expression(), terminator());
  let expressions = parser().easy_parse(PositionStream::new(tokens));

  match expressions {
    Ok((expressions, stream)) => {
      // Check if there are any tokens left.
      if !stream.input.is_empty() {
        eprintln!("[Parser]");
        eprintln!("Unparsed:");
        for token in stream.input {
          eprintln!("  {token}");
        }
        eprintln!("\nPosition: {}\n", stream.position());
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