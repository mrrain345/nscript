use super::Token;
use combine::parser::{Parser, char, choice, range, combinator};
use combine::stream::RangeStream;

pub fn number<'src, I>() -> impl Parser<I, Output=Token> + 'src
  where I: RangeStream<Token=char, Range=&'src str> + 'src {

  let take_num1 = || range::take_while1(|c: char| c.is_digit(10));
  let take_num  = ||  range::take_while(|c: char| c.is_digit(10));

  // Parser for the exponent part of a number.
  let exponent = || (
    char::char('e').or(char::char('E')),
    choice::choice((  
      range::range("+"),
      range::range("-"),
      range::range(""),
    )),
    take_num1(),
  );

  // Parser for numbers with a decimal point and optional exponent.
  let number1 = choice::or(
    ( take_num1(), char::char('.'), take_num()  ),
    ( take_num(),  char::char('.'), take_num1() ),
  )
  .and( choice::optional(exponent()) )
  .map(|((num1, _, num2), exp)| {
    if let Some((_, sign, exp)) = exp {
      // Parse a number with an exponent.
      let num = format!("{}.{}e{}{}", &num1, &num2, &sign, &exp);
      Token::Number(num.parse::<f64>().unwrap())
    } else {
      // Parse a number without an exponent.
      let num = format!("{}.{}", &num1, &num2);
      Token::Number(num.parse::<f64>().unwrap())
    }
  });

  // Parser for numbers without a decimal point, but with an exponent.
  let number2 = ( take_num1(), exponent() )
    .map(|(num, (_, sign, exp))| {
      let num = format!("{}e{}{}", &num, &sign, &exp);
      Token::Number(num.parse::<f64>().unwrap())
    });
  
  choice::choice((
    combinator::attempt(number1),
    combinator::attempt(number2),
  ))
}