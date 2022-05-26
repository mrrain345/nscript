use combine::{parser::token, parser, token, Stream};

pub use crate::tokenizer::{Token, Keyword, Punctuator, Operator};

// Keyword
parser! {
  /// Check if token is a keyword.
  pub fn keyword[I](keyword: Keyword)(I) -> ()
  where [ I: Stream<Token=Token> ] {
    
    token(Token::Keyword(*keyword)).map(|_| ())
  }
}

// Punctuator
parser! {
  /// Check if token is a punctuator.
  pub fn punctuator[I](punctuator: Punctuator)(I) -> ()
  where [ I: Stream<Token=Token> ] {
    
    token(Token::Punctuator(*punctuator)).map(|_| ())
  }
}

// Operator
parser! {
  /// Check if token is an operator.
  pub fn operator[I](operator: Operator)(I) -> Operator
  where [ I: Stream<Token=Token> ] {
    
    token(Token::Operator(*operator)).map(|_| *operator)
  }
}

// Identifier
parser! {
  /// Check if token is an identifier.
  pub fn identifier[I]()(I) -> String
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Identifier(_) = token {true} else {false}
    }).map(|token| {
      if let Token::Identifier(identifier) = token { identifier }
      else { unreachable!() }
    })
  }
}

// Type
parser! {
  /// Check if token is a type.
  pub fn type_[I]()(I) -> String
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Identifier(_) = token {true} else {false}
    }).map(|token| {
      if let Token::Identifier(type_) = token { type_ }
      else { unreachable!() }
    })
  }
}

// Integer
parser! {
  /// Check if token is an integer.
  pub fn integer[I]()(I) -> i32
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Integer(_) = token {true} else {false}
    }).map(|token| {
      if let Token::Integer(value) = token { value }
      else { unreachable!() }
    })
  }
}

// Number
parser! {
  /// Check if token is a number.
  pub fn number[I]()(I) -> f64
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Number(_) = token {true} else {false}
    }).map(|token| {
      if let Token::Number(value) = token { value }
      else { unreachable!() }
    })
  }
}

// Boolean
parser! {
  /// Check if token is a boolean.
  pub fn boolean[I]()(I) -> bool
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Boolean(_) = token {true} else {false}
    }).map(|token| {
      if let Token::Boolean(value) = token { value }
      else { unreachable!() }
    })
  }
}

// String
parser! {
  /// Check if token is a string.
  pub fn string[I]()(I) -> String
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::String(_) = token {true} else {false}
    }).map(|token| {
      if let Token::String(value) = token { value }
      else { unreachable!() }
    })
  }
}

// Null
parser! {
  /// Check if token is a null.
  pub fn null[I]()(I) -> ()
  where [ I: Stream<Token=Token> ] {
      
    token::satisfy(|token| {
      if let Token::Null = token {true} else {false}
    }).map(|token| {
      if let Token::Null = token { () }
      else { unreachable!() }
    })
  }
}

// Terminator
parser! {
  /// Check if token is a terminator.
  pub fn terminator[I]()(I) -> ()
  where [ I: Stream<Token=Token> ] {
    
    token(Token::Terminator).or(token(Token::NewLine)).map(|_| ())
  }
}