use super::Token;
use combine::parser::{char, choice, range, combinator};
use combine::parser::Parser;
use combine::stream::RangeStream;

pub fn number<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  // Parser for the exponent part of a number.
  let exp = (
    char::char('e').or(char::char('E')),
    choice::optional(
      char::char('+').or(char::char('-')),
    ),
    range::take_while1(|c: char| c.is_digit(10)),
  );

  // Parser for numbers with a decimal point with an optional exponent.
  let number1 = (
    choice::or(
      (
        range::take_while1(|c: char| c.is_digit(10)),
        char::char('.'),
        range::take_while(|c: char| c.is_digit(10)),
      ), (
        range::take_while(|c: char| c.is_digit(10)),
        char::char('.'),
        range::take_while1(|c: char| c.is_digit(10)),
      )
    ).and(
      choice::optional(exp),
    )
  ).map(|((num1, _, num2), exp): ((&str, char, &str), Option<(char, Option<char>, &str)>)| {
    let num = if let Some((_, sign, exp)) = exp {
      let sign = match sign {
        Some('+') => "+",
        Some('-') => "-",
        _ => "",
      };
      format!("{}.{}e{}{}", &num1, &num2, &sign, &exp)
    } else {
      format!("{}.{}", &num1, &num2)
    };

    Token::Number(num.parse::<f64>().unwrap())
  });

  // Parser for the exponent part of a number.
  let exp = (
    char::char('e').or(char::char('E')),
    choice::optional(
      char::char('+').or(char::char('-')),
    ),
    range::take_while1(|c: char| c.is_digit(10)),
  );

  // Parser for numbers without a decimal point with an exponent.
  let number2 = (
    range::take_while1(|c: char| c.is_digit(10)),
    exp,
  ).map(|(num, (_, sign, exp)): (&str, (char, Option<char>, &str))| {
    let sign = match sign {
      Some('+') => "+",
      Some('-') => "-",
      _ => "",
    };
    let num = format!("{}e{}{}", &num, &sign, &exp);
    Token::Number(num.parse::<f64>().unwrap())
  });
  
  return choice::choice((
    combinator::attempt(number1),
    combinator::attempt(number2),
  ));
}