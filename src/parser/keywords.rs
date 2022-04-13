use super::Token;
use combine::parser::{char, choice, range};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::error::ParseError;

#[derive(Debug)]
pub enum Keyword {
  Let,
  Var,
  Fn,
  If,
  Else,
  While,
  For,
  Return,
  Break,
  Continue,
  This,
  Typeof,
  In,
  Match,
  Default,
  Try,
  Catch,
  Finally,
  Throw,
}

pub fn keyword<'a, I>() -> impl Parser<I, Output=Token> + 'a
  where I: RangeStream<Token=char, Range=&'a str> + 'a,
        I::Error: ParseError<I::Token, I::Range, I::Position> {
  
  return choice::choice((
    range::range("let").map(|_| Token::Keyword(Keyword::Let)),
    range::range("var").map(|_| Token::Keyword(Keyword::Var)),
    range::range("fn").map(|_| Token::Keyword(Keyword::Fn)),
    range::range("if").map(|_| Token::Keyword(Keyword::If)),
    range::range("else").map(|_| Token::Keyword(Keyword::Else)),
    range::range("while").map(|_| Token::Keyword(Keyword::While)),
    range::range("for").map(|_| Token::Keyword(Keyword::For)),
    range::range("return").map(|_| Token::Keyword(Keyword::Return)),
    range::range("break").map(|_| Token::Keyword(Keyword::Break)),
    range::range("continue").map(|_| Token::Keyword(Keyword::Continue)),
    range::range("this").map(|_| Token::Keyword(Keyword::This)),
    range::range("typeof").map(|_| Token::Keyword(Keyword::Typeof)),
    range::range("in").map(|_| Token::Keyword(Keyword::In)),
    range::range("match").map(|_| Token::Keyword(Keyword::Match)),
    range::range("default").map(|_| Token::Keyword(Keyword::Default)),
    range::range("try").map(|_| Token::Keyword(Keyword::Try)),
    range::range("catch").map(|_| Token::Keyword(Keyword::Catch)),
    range::range("finally").map(|_| Token::Keyword(Keyword::Finally)),
    range::range("throw").map(|_| Token::Keyword(Keyword::Throw)),
  ));
}