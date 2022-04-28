use combine::parser::{Parser, char, range, choice};
use combine::stream::RangeStream;

use super::ignore_spaces;

// - + ! ~
pub fn unitary_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      char::char('-'),
      char::char('+'),
      char::char('!'),
      char::char('~'),
    ))
  )
}

// **
pub fn power_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range("**")
  )
}

// * / %
pub fn multiplicative_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      char::char('*'),
      char::char('/'),
      char::char('%'),
    ))
  )
}

// + -
pub fn additive_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      char::char('+'),
      char::char('-'),
    ))
  )
}

// << >>
pub fn shift_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      range::range("<<"),
      range::range(">>"),
    ))
  )
}

// < > <= >=
pub fn relational_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      range::range("<"),
      range::range(">"),
      range::range("<="),
      range::range(">="),
    ))
  )
}

// == !=
pub fn equality_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      range::range("=="),
      range::range("!="),
    ))
  )
}

// &
pub fn bitwise_and_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    char::char('&')
  )
}

// ^
pub fn bitwise_xor_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    char::char('^')
  )
}

// |
pub fn bitwise_or_operator<'src, I>() -> impl Parser<I, Output=char> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    char::char('|')
  )
}

// &&
pub fn logical_and_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range("&&")
  )
}

// ||
pub fn logical_or_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    range::range("||")
  )
}

// = *= /= %= **= += -= <<= >>= &= ^= |=
pub fn assignment_operator<'src, I>() -> impl Parser<I, Output=&'src str> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  ignore_spaces(
    choice::choice((
      range::range("="),
      range::range("*="),
      range::range("/="),
      range::range("%="),
      range::range("**="),
      range::range("+="),
      range::range("-="),
      range::range("<<="),
      range::range(">>="),
      range::range("&="),
      range::range("^="),
      range::range("|="),
    ))
  )
}