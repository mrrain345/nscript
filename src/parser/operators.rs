use super::Token;
use combine::parser::{char, choice, range};
use combine::parser::Parser;
use combine::stream::RangeStream;
use combine::error::ParseError;

#[derive(Debug)]
pub enum Operator {
  // Assignment operators
  Assign,             // =
  PlusAssign,         // +=
  MinusAssign,        // -=
  MultiplyAssign,     // *=
  DivideAssign,       // /=
  ModuloAssign,       // %=
  PowerAssign,        // **=
  BitwiseAndAssign,   // &=
  BitwiseOrAssign,    // |=
  BitwiseXorAssign,   // ^=

  // Arithmetic operators
  Plus,               // +
  Minus,              // -
  Multiply,           // *
  Divide,             // /
  Modulo,             // %
  Power,              // **

  // Logical operators
  And,                // &&
  Or,                 // ||
  Not,                // !

  // Comparison operators
  Equal,              // ==
  NotEqual,           // !=
  LessThan,           // <
  GreaterThan,        // >
  LessOrEqual,        // <=
  GreaterOrEqual,     // >=

  // Bitwise operators
  BitwiseAnd,         // &
  BitwiseOr,          // |
  BitwiseXor,         // ^
  BitwiseNot,         // ~
}

pub fn operator<'a, I>() -> impl Parser<I, Output=Token<'a>>
  where I: RangeStream<Token=char, Range=&'a str>,
        I::Error: ParseError<I::Token, I::Range, I::Position> {

  let assignment = choice::choice((
    range::range("=").map(|_| Token::Operator(Operator::Assign)),
    range::range("+=").map(|_| Token::Operator(Operator::PlusAssign)),
    range::range("-=").map(|_| Token::Operator(Operator::MinusAssign)),
    range::range("*=").map(|_| Token::Operator(Operator::MultiplyAssign)),
    range::range("/=").map(|_| Token::Operator(Operator::DivideAssign)),
    range::range("%=").map(|_| Token::Operator(Operator::ModuloAssign)),
    range::range("**=").map(|_| Token::Operator(Operator::PowerAssign)),
    range::range("&=").map(|_| Token::Operator(Operator::BitwiseAndAssign)),
    range::range("|=").map(|_| Token::Operator(Operator::BitwiseOrAssign)),
    range::range("^=").map(|_| Token::Operator(Operator::BitwiseXorAssign)),
  ));

  let arithmetic = choice::choice((
    range::range("+").map(|_| Token::Operator(Operator::Plus)),
    range::range("-").map(|_| Token::Operator(Operator::Minus)),
    range::range("*").map(|_| Token::Operator(Operator::Multiply)),
    range::range("/").map(|_| Token::Operator(Operator::Divide)),
    range::range("%").map(|_| Token::Operator(Operator::Modulo)),
    range::range("**").map(|_| Token::Operator(Operator::Power)),
  ));

  let logical = choice::choice((
    range::range("&&").map(|_| Token::Operator(Operator::And)),
    range::range("||").map(|_| Token::Operator(Operator::Or)),
    range::range("!").map(|_| Token::Operator(Operator::Not)),
  ));

  let comparison = choice::choice((
    range::range("==").map(|_| Token::Operator(Operator::Equal)),
    range::range("!=").map(|_| Token::Operator(Operator::NotEqual)),
    range::range("<").map(|_| Token::Operator(Operator::LessThan)),
    range::range(">").map(|_| Token::Operator(Operator::GreaterThan)),
    range::range("<=").map(|_| Token::Operator(Operator::LessOrEqual)),
    range::range(">=").map(|_| Token::Operator(Operator::GreaterOrEqual)),
  ));

  let bitwise = choice::choice((
    range::range("&").map(|_| Token::Operator(Operator::BitwiseAnd)),
    range::range("|").map(|_| Token::Operator(Operator::BitwiseOr)),
    range::range("^").map(|_| Token::Operator(Operator::BitwiseXor)),
    range::range("~").map(|_| Token::Operator(Operator::BitwiseNot)),
  ));

  return choice::choice((
    assignment,
    arithmetic,
    logical,
    comparison,
    bitwise,
  ));
}