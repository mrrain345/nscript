use combine::parser::{char, choice, range, repeat, combinator};
use combine::parser::{Parser, EasyParser};
use combine::stream::{RangeStream};
use combine::stream::position::{Stream, SourcePosition};
use combine::error::ParseError;
use combine::easy::Errors;

mod keywords;
mod operators;
mod punctuators;
mod number;
mod integer;
mod string;

use keywords::Keyword;
use operators::Operator;
use punctuators::Punctuator;

#[derive(Debug)]
pub enum Token {
  Keyword(Keyword),
  Operator(Operator),
  Punctuator(Punctuator),
  Identifier(String),
  Integer(i32),
  Number(f64),
  Boolean(bool),
  String(String),
  Null,
  Terminator,
}

pub fn parse<'a>(source: &'a str) -> Result<
    (Vec<Token>, Stream<&str, SourcePosition>),
    Errors<char, &str, SourcePosition>> {
    
  return tokenize().easy_parse(Stream::new(source));
}

fn tokenize<'a, I>() -> impl Parser<I, Output = Vec<Token>> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let tokens = choice::choice((
    number::number(),
    integer::integer(),
    string::string(),
    literals(),
    keywords::keyword(),
    operators::operator(),
    punctuators::punctuator(),
    identifier(),
    terminator(),
  ));

  let spaces = range::take_while(|c: char| c != '\n' && c.is_whitespace());
  
  return repeat::sep_by(tokens, spaces);
}

fn literals<'a, I>() -> impl Parser<I, Output=Token> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let true_ = range::range("true").map(|_| Token::Boolean(true));
  let false_ = range::range("false").map(|_| Token::Boolean(false));
  let null_ = range::range("null").map(|_| Token::Null);

  return choice::choice((
    combinator::attempt(true_),
    combinator::attempt(false_),
    combinator::attempt(null_),
  ));
}

fn identifier<'a, I>() -> impl Parser<I, Output=Token> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {
  
  range::take_while1(|c: char| c.is_alphanumeric() || c == '_').map(|s: &str| {
    Token::Identifier(String::from(s))
  })
}

fn terminator<'a, I>() -> impl Parser<I, Output=Token> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  range::take_while1(|c: char| c == ';' || c == '\n' || c.is_whitespace()).map(|_| {
    Token::Terminator
  })
}