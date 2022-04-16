use combine::parser::{Parser, char, repeat, combinator, token};
use combine::stream::RangeStream;

pub fn terminator<'src, I>() -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

    repeat::skip_many(token::satisfy(|c: char| c != '\n' && c.is_whitespace()))
      .skip(char::char(';').or(char::char('\n')))
      .with(repeat::skip_many(char::space().or(char::char(';'))))
}

pub fn spaces<'src, I>() -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  char::spaces()
}

pub fn separator<'src, I>() -> impl Parser<I, Output=()> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  combinator::not_followed_by(char::alpha_num())
}