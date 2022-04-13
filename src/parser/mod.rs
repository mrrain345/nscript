use combine::parser::{char, choice, range, repeat, combinator};
use combine::parser::{Parser, EasyParser};
use combine::stream::RangeStream;
use combine::stream::position::{SourcePosition, Stream};
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

pub fn parse<'src>(source: &'src str) ->
  Result<(Vec<Token>, Stream<&str, SourcePosition>), Errors<char, &str, SourcePosition>> {

  tokenize().easy_parse(Stream::new(source))
}

fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Token>> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

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

fn literals<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let true_ = range::range("true").map(|_| Token::Boolean(true));
  let false_ = range::range("false").map(|_| Token::Boolean(false));
  let null_ = range::range("null").map(|_| Token::Null);

  return choice::choice((
    combinator::attempt(true_),
    combinator::attempt(false_),
    combinator::attempt(null_),
  ));
}

fn identifier<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  range::take_while1(|c: char| c.is_alphanumeric() || c == '_').map(|s: &str| {
    Token::Identifier(String::from(s))
  })
}

fn terminator<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  range::take_while1(|c: char| c == ';' || c == '\n' || c.is_whitespace()).map(|_| {
    Token::Terminator
  })
}