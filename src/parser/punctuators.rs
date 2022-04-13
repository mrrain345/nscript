use super::Token;
use combine::parser::{char, choice, range};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::error::ParseError;

#[derive(Debug)]
pub enum Punctuator {
  LeftParen,          // (
  RightParen,         // )
  LeftBrace,          // {
  RightBrace,         // }
  LeftBracket,        // [
  RightBracket,       // ]
  LeftAngle,          // <
  RightAngle,         // >
  Comma,              // ,
  Dot,                // .
  Colon,              // :
  QuestionMark,       // ?
  Arrow,              // ->
}

pub fn punctuator<'a, I>() -> impl Parser<I, Output=Token> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {
  
  return choice::choice((
    range::range("(").map(|_| Token::Punctuator(Punctuator::LeftParen)),
    range::range(")").map(|_| Token::Punctuator(Punctuator::RightParen)),
    range::range("{").map(|_| Token::Punctuator(Punctuator::LeftBrace)),
    range::range("}").map(|_| Token::Punctuator(Punctuator::RightBrace)),
    range::range("[").map(|_| Token::Punctuator(Punctuator::LeftBracket)),
    range::range("]").map(|_| Token::Punctuator(Punctuator::RightBracket)),
    range::range("<").map(|_| Token::Punctuator(Punctuator::LeftAngle)),
    range::range(">").map(|_| Token::Punctuator(Punctuator::RightAngle)),
    range::range(",").map(|_| Token::Punctuator(Punctuator::Comma)),
    range::range(".").map(|_| Token::Punctuator(Punctuator::Dot)),
    range::range(":").map(|_| Token::Punctuator(Punctuator::Colon)),
    range::range("?").map(|_| Token::Punctuator(Punctuator::QuestionMark)),
    range::range("->").map(|_| Token::Punctuator(Punctuator::Arrow)),
  ));
}