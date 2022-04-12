use combine::parser::{char, choice, range, repeat};
use combine::parser::{Parser, EasyParser};
use combine::stream::{RangeStream};
use combine::stream::position::{Stream, SourcePosition};
use combine::error::ParseError;
use combine::easy::Errors;

mod keywords;
mod operators;
mod punctuators;
mod literals;

use keywords::Keyword;
use operators::Operator;
use punctuators::Punctuator;

#[derive(Debug)]
pub enum Token<'a> {
  Keyword(Keyword),
  Operator(Operator),
  Punctuator(Punctuator),
  Identifier(&'a str),
  Integer(i32),
  Number(f64),
  Boolean(bool),
  String(&'a str),
  Null,
  Terminator,
}

pub fn parse<'a>(source: &'a str) -> Result<
    (Vec<Token<'a>>, Stream<&str, SourcePosition>),
    Errors<char, &str, SourcePosition>> {
    
  return tokenize().easy_parse(Stream::new(source));
}

fn tokenize<'a, I>() -> impl Parser<I, Output = Vec<Token<'a>>>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let tokens = choice::choice((
    literals::literals(),
    keywords::keyword(),
    operators::operator(),
    punctuators::punctuator(),
    identifier(),
    terminator(),
  ));

  let spaces = range::take_while(|c: char| c != '\n' && c.is_whitespace());
  
  return repeat::sep_by(tokens, spaces);
}

fn identifier<'a, I>() -> impl Parser<I, Output=Token<'a>>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {
  
  return range::take_while1(|c: char| c.is_alphanumeric() || c == '_').map(|s: &str| Token::Identifier(s));
}

fn terminator<'a, I>() -> impl Parser<I, Output=Token<'a>>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  return range::take_while1(|c: char| c == ';' || c == '\n' || c.is_whitespace()).map(|_| Token::Terminator);
}