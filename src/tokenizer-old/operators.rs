use super::Token;
use combine::parser::{Parser, choice, range};
use combine::stream::RangeStream;

#[derive(Debug, Clone, Copy, PartialEq)]
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

  // Parser for operators
  let operator = |s, op| range::range(s).map(move |_| Token::Operator(op));
    
  let comparison = choice::choice((
    operator("==",  Operator::Equal),
    operator("!=",  Operator::NotEqual),
    operator("<=",  Operator::LessOrEqual),
    operator(">=",  Operator::GreaterOrEqual),
    operator("<",   Operator::LessThan),
    operator(">",   Operator::GreaterThan),
  ));

  let other = choice::choice((
    operator("...", Operator::Spread),
    operator(".",   Operator::Dot),
    operator("??",  Operator::NullCoalescing),
    operator("?.",  Operator::OptionalChain),
    operator("?",   Operator::Optional),
    operator("->",  Operator::Arrow),
    operator("=>",  Operator::FatArrow),
  ));
  
  let assignment = choice::choice((
    operator("=",   Operator::Assign),
    operator("+=",  Operator::PlusAssign),
    operator("-=",  Operator::MinusAssign),
    operator("**=", Operator::PowerAssign),
    operator("*=",  Operator::MultiplyAssign),
    operator("/=",  Operator::DivideAssign),
    operator("%=",  Operator::ModuloAssign),
    operator("&=",  Operator::BitwiseAndAssign),
    operator("|=",  Operator::BitwiseOrAssign),
    operator("^=",  Operator::BitwiseXorAssign),
  ));

  let arithmetic = choice::choice((
    operator("+",   Operator::Plus),
    operator("-",   Operator::Minus),
    operator("**",  Operator::Power),
    operator("*",   Operator::Multiply),
    operator("/",   Operator::Divide),
    operator("%",   Operator::Modulo),
  ));

  let logical = choice::choice((
    operator("&&",  Operator::And),
    operator("||",  Operator::Or),
    operator("!",   Operator::Not),
  ));

  let bitwise = choice::choice((
    operator("&",   Operator::BitwiseAnd),
    operator("|",   Operator::BitwiseOr),
    operator("^",   Operator::BitwiseXor),
    operator("~",   Operator::BitwiseNot),
  ));

  choice::choice((
    comparison,
    other,
    assignment,
    arithmetic,
    logical,
    bitwise,
  ))
}