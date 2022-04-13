use super::Token;
use combine::parser::{char, choice, range};
use combine::parser::Parser;
use combine::stream::RangeStream;

#[derive(Debug)]
pub enum Operator {
  // Comparison operators
  Equal,              // ==
  NotEqual,           // !=
  LessOrEqual,        // <=
  GreaterOrEqual,     // >=
  LessThan,           // <
  GreaterThan,        // >
  
  // Assignment operators
  Assign,             // =
  PlusAssign,         // +=
  MinusAssign,        // -=
  PowerAssign,        // **=
  MultiplyAssign,     // *=
  DivideAssign,       // /=
  ModuloAssign,       // %=
  BitwiseAndAssign,   // &=
  BitwiseOrAssign,    // |=
  BitwiseXorAssign,   // ^=

  // Other operators
  Spread,             // ...
  Dot,                // .
  NullCoalescing,     // ??
  OptionalChain,      // ?.
  Optional,           // ?
  Arrow,              // ->
  FatArrow,           // =>

  // Arithmetic operators
  Plus,               // +
  Minus,              // -
  Power,              // **
  Multiply,           // *
  Divide,             // /
  Modulo,             // %

  // Logical operators
  And,                // &&
  Or,                 // ||
  Not,                // !

  // Bitwise operators
  BitwiseAnd,         // &
  BitwiseOr,          // |
  BitwiseXor,         // ^
  BitwiseNot,         // ~
}

pub fn operator<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {
    
  let comparison = choice::choice((
    range::range("==")  .map(|_| Token::Operator(Operator::Equal)),
    range::range("!=")  .map(|_| Token::Operator(Operator::NotEqual)),
    range::range("<=")  .map(|_| Token::Operator(Operator::LessOrEqual)),
    range::range(">=")  .map(|_| Token::Operator(Operator::GreaterOrEqual)),
    range::range("<")   .map(|_| Token::Operator(Operator::LessThan)),
    range::range(">")   .map(|_| Token::Operator(Operator::GreaterThan)),
  ));

  let other = choice::choice((
    range::range("...") .map(|_| Token::Operator(Operator::Spread)),
    range::range(".")   .map(|_| Token::Operator(Operator::Dot)),
    range::range("??")  .map(|_| Token::Operator(Operator::NullCoalescing)),
    range::range("?.")  .map(|_| Token::Operator(Operator::OptionalChain)),
    range::range("?")   .map(|_| Token::Operator(Operator::Optional)),
    range::range("->")  .map(|_| Token::Operator(Operator::Arrow)),
    range::range("=>")  .map(|_| Token::Operator(Operator::FatArrow)),
  ));
  
  let assignment = choice::choice((
    range::range("=")   .map(|_| Token::Operator(Operator::Assign)),
    range::range("+=")  .map(|_| Token::Operator(Operator::PlusAssign)),
    range::range("-=")  .map(|_| Token::Operator(Operator::MinusAssign)),
    range::range("**=") .map(|_| Token::Operator(Operator::PowerAssign)),
    range::range("*=")  .map(|_| Token::Operator(Operator::MultiplyAssign)),
    range::range("/=")  .map(|_| Token::Operator(Operator::DivideAssign)),
    range::range("%=")  .map(|_| Token::Operator(Operator::ModuloAssign)),
    range::range("&=")  .map(|_| Token::Operator(Operator::BitwiseAndAssign)),
    range::range("|=")  .map(|_| Token::Operator(Operator::BitwiseOrAssign)),
    range::range("^=")  .map(|_| Token::Operator(Operator::BitwiseXorAssign)),
  ));

  let arithmetic = choice::choice((
    range::range("+")   .map(|_| Token::Operator(Operator::Plus)),
    range::range("-")   .map(|_| Token::Operator(Operator::Minus)),
    range::range("**")  .map(|_| Token::Operator(Operator::Power)),
    range::range("*")   .map(|_| Token::Operator(Operator::Multiply)),
    range::range("/")   .map(|_| Token::Operator(Operator::Divide)),
    range::range("%")   .map(|_| Token::Operator(Operator::Modulo)),
  ));

  let logical = choice::choice((
    range::range("&&")  .map(|_| Token::Operator(Operator::And)),
    range::range("||")  .map(|_| Token::Operator(Operator::Or)),
    range::range("!")   .map(|_| Token::Operator(Operator::Not)),
  ));

  let bitwise = choice::choice((
    range::range("&")   .map(|_| Token::Operator(Operator::BitwiseAnd)),
    range::range("|")   .map(|_| Token::Operator(Operator::BitwiseOr)),
    range::range("^")   .map(|_| Token::Operator(Operator::BitwiseXor)),
    range::range("~")   .map(|_| Token::Operator(Operator::BitwiseNot)),
  ));

  return choice::choice((
    comparison,
    other,
    assignment,
    arithmetic,
    logical,
    bitwise,
  ));
}