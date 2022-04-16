use combine::parser::{Parser, char, choice, range, repeat, combinator};
use combine::stream::{RangeStream};

mod keywords;
mod operators;
mod punctuators;
mod number;
mod integer;
mod string;

pub use keywords::Keyword;
pub use operators::Operator;
pub use punctuators::Punctuator;

#[derive(Debug, Clone, PartialEq)]
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
  SoftTerminator,
}

pub fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Token>> + 'src
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
  repeat::sep_by(tokens, spaces)
}

fn literals<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let true_ = range::range("true")
    .map(|_| Token::Boolean(true));

  let false_ = range::range("false")
    .map(|_| Token::Boolean(false));

  let null_ = range::range("null")
    .map(|_| Token::Null);

  combinator::attempt(
    choice::choice((
      true_, false_, null_,
    )).skip(combinator::not_followed_by(char::alpha_num()))
  )
}


fn identifier<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
  
  range::take_while1(|c: char| c.is_alphanumeric() || c == '_').map(|s: &str| {
    Token::Identifier(String::from(s))
  })
}

fn terminator<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let term = char::char(';').with(range::take_while1(|c: char| c == ';' || c.is_whitespace())).map(|_| {
    Token::Terminator
  });

  let soft_term = range::take_while1(|c: char| c.is_whitespace()).map(|_| {
    Token::SoftTerminator
  });

  return term.or(soft_term);
}