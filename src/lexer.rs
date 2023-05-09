use super::*;

#[derive(Debug)]
pub(crate) struct Lexer {
  input: String,
}

impl Lexer {
  pub(crate) fn new() -> Self {
    Self {
      input: String::new(),
    }
  }

  pub(crate) fn load(&self, input: String) -> Self {
    Self { input }
  }

  pub(crate) fn lex(&self) -> Vec<Token> {
    self.input.chars().map(|token| Token::from(token)).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::{Token::*, *};

  #[test]
  fn lex() {
    assert_eq!(
      Lexer::new()
        .load(">>>.<<[--],++ This is cool".to_string())
        .lex(),
      vec![
        Greater, Greater, Greater, Dot, Less, Less, BraceL, Minus, Minus,
        BraceR, Comma, Plus, Plus, Unknown, Unknown, Unknown, Unknown, Unknown,
        Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown
      ]
    );
  }
}
