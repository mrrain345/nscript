use super::Token;
use combine::parser::{Parser, choice, char};
use combine::stream::RangeStream;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuator {
  LeftParen,          // (
  RightParen,         // )
  LeftBrace,          // {
  RightBrace,         // }
  LeftBracket,        // [
  RightBracket,       // ]
  Comma,              // ,
  Colon,              // :
}

pub fn punctuator<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // Parser for punctuators
  let punctuator = |c, p| char::char(c).map(move |_| Token::Punctuator(p));
  
  choice::choice((
    punctuator('(', Punctuator::LeftParen),
    punctuator(')', Punctuator::RightParen),
    punctuator('{', Punctuator::LeftBrace),
    punctuator('}', Punctuator::RightBrace),
    punctuator('[', Punctuator::LeftBracket),
    punctuator(']', Punctuator::RightBracket),
    punctuator(',', Punctuator::Comma),
    punctuator(':', Punctuator::Colon),
  ))
}