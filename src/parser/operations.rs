use combine::parser::{choice, repeat, sequence};
use combine::stream::RangeStream;
use combine::parser;

use super::expressions::Expression;
use super::tokenizer::*;


parser!{
  pub fn operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    logical_or_operation()
  }
}

parser!{
  fn parenthesis['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    sequence::between(
      punctuator("("),
      punctuator(")"),
      operation(),
    )
  }
}

parser!{
  fn literal['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    choice::choice((
      number().map(|v| Expression::Number(v)),
      integer().map(|v| Expression::Integer(v)),
      string().map(|v| Expression::String(v)),
      boolean().map(|v| Expression::Boolean(v)),
      null().map(|_| Expression::Null),
    ))
  }
}

parser!{
  fn highest_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    choice::choice((
      parenthesis(),
      literal(),
      identifier().map(|s| Expression::Identifier(s)),
    ))
  }
}

// Operator precedence
// Category         | Operator    | Associativity
// -----------------+-------------+-----------------
// Unitary          | - + ! ~     | Right
// Power            | **          | Right
// Multiplicative   | * / %       | Left
// Additive         | + -         | Left
// Shift            | << >>       | Left
// Relational       | < > <= =>   | Left
// Equality         | == !=       | Left
// Bitwise AND      | &           | Left
// Bitwise XOR      | ^           | Left
// Bitwise OR       | |           | Left
// Logical AND      | &&          | Left
// Logical OR       | ||          | Left
// Assignment       | = *= /= **= | Right
//                  | += -= &= %=
//                  | >>= <<= |= ^=

parser!{
  /// Unitary operation (- + ! ~)
  fn unary_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    choice::choice((
      highest_operation(), // allows to nest highest-order operations
      unitary_operator().and(operation()).map(|(op, expr)| {
        match op {
          '-' => Expression::Minus(Box::new(expr)),
          '+' => Expression::Plus(Box::new(expr)),
          '!' => Expression::Not(Box::new(expr)),
          '~' => Expression::BitwiseNot(Box::new(expr)),
          _ => unreachable!(),
        }
      }),
    ))
  }
}

parser!{
  /// Power operation (**)
  fn power_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    repeat::chainr1(
      unary_operation(), // allow to nest higher-order operations
      power_operator().map(|_| move |l, r| {
        Expression::Power(Box::new(l), Box::new(r))
      })
    )
  }
}

parser!{
  /// Multiplicative operations (*, /, %)
  fn multiplicative_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      power_operation(), // allows to nest higher-order operations
      multiplicative_operator().map(|op| move |l, r| {
        match op {
          '*' => Expression::Mul(Box::new(l), Box::new(r)),
          '/' => Expression::Div(Box::new(l), Box::new(r)),
          '%' => Expression::Modulo(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser!{
  /// Additive operations (+, -)
  pub fn additive_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {
    
    repeat::chainl1(
      multiplicative_operation(), // allows to nest higher-order operations
      additive_operator().map(|op| move |l, r| {
        match op {
          '+' => Expression::Add(Box::new(l), Box::new(r)),
          '-' => Expression::Sub(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser!{
  /// Shift operations (<<, >>)
  fn shift_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      additive_operation(), // allows to nest higher-order operations
      shift_operator().map(|op| move |l, r| {
        match op {
          "<<" => Expression::LeftShift(Box::new(l), Box::new(r)),
          ">>" => Expression::RightShift(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser! {
  /// Relational operations (<, >, <=, >=)
  fn relational_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      shift_operation(), // allows to nest higher-order operations
      relational_operator().map(|op| move |l, r| {
        match op {
          "<" => Expression::LessThan(Box::new(l), Box::new(r)),
          ">" => Expression::GreaterThan(Box::new(l), Box::new(r)),
          "<=" => Expression::LessOrEqual(Box::new(l), Box::new(r)),
          ">=" => Expression::GreaterOrEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser! {
  /// Equality operations (==, !=)
  fn equality_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      relational_operation(), // allows to nest higher-order operations
      equality_operator().map(|op| move |l, r| {
        match op {
          "==" => Expression::Equal(Box::new(l), Box::new(r)),
          "!=" => Expression::NotEqual(Box::new(l), Box::new(r)),
          _ => unreachable!(),
        }
      })
    )
  }
}

parser! {
  /// Bitwise AND operation (&)
  fn bitwise_and_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      equality_operation(), // allows to nest higher-order operations
      bitwise_and_operator().map(|_| move |l, r| {
        Expression::BitwiseAnd(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Bitwise XOR operation (^)
  fn bitwise_xor_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_and_operation(), // allows to nest higher-order operations
      bitwise_xor_operator().map(|_| move |l, r| {
        Expression::BitwiseXor(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Bitwise OR operation (|)
  fn bitwise_or_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_xor_operation(), // allows to nest higher-order operations
      bitwise_or_operator().map(|_| move |l, r| {
        Expression::BitwiseOr(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Logical AND operation (&&)
  fn logical_and_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      bitwise_or_operation(), // allows to nest higher-order operations
      logical_and_operator().map(|_| move |l, r| {
        Expression::And(Box::new(l), Box::new(r))
      })
    )
  }
}

parser! {
  /// Logical OR operation (||)
  fn logical_or_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    repeat::chainl1(
      logical_and_operation(), // allows to nest higher-order operations
      logical_or_operator().map(|_| move |l, r| {
        Expression::Or(Box::new(l), Box::new(r))
      })
    )
  }
}

// TODO: allows to chain assignment operations
parser! {
  /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
  pub fn assignment_operation['src, I]()(I) -> Expression
  where [ I: RangeStream<Token=char, Range=&'src str> + 'src ] {

    (
      identifier(),
      assignment_operator(),
      logical_and_operation(), // allows to nest higher-order operations
    ).map(|(name, op, value)| {
      match op {
        "=" => Expression::Assign { name, value: Box::new(value) },
        "+=" => Expression::AddAssign { name, value: Box::new(value) },
        "-=" => Expression::SubAssign { name, value: Box::new(value) },
        "*=" => Expression::MulAssign { name, value: Box::new(value) },
        "/=" => Expression::DivAssign { name, value: Box::new(value) },
        "%=" => Expression::ModuloAssign { name, value: Box::new(value) },
        "**=" => Expression::PowerAssign { name, value: Box::new(value) },
        "<<=" => Expression::LeftShiftAssign { name, value: Box::new(value) },
        ">>=" => Expression::RightShiftAssign { name, value: Box::new(value) },
        "&=" => Expression::BitwiseAndAssign { name, value: Box::new(value) },
        "^=" => Expression::BitwiseXorAssign { name, value: Box::new(value) },
        "|=" => Expression::BitwiseOrAssign { name, value: Box::new(value) },
        _ => unreachable!(),
      }
    })
  }
}