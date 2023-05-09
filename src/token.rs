use super::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
  BraceL,
  BraceR,
  Comma,
  Dot,
  Greater,
  Less,
  Minus,
  Plus,
  Unknown,
}

impl From<char> for Token {
  fn from(c: char) -> Self {
    match c {
      '+' => Self::Plus,
      ',' => Self::Comma,
      '-' => Self::Minus,
      '.' => Self::Dot,
      '<' => Self::Less,
      '>' => Self::Greater,
      '[' => Self::BraceL,
      ']' => Self::BraceR,
      _ => Self::Unknown,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::BraceL => write!(f, "["),
      Self::BraceR => write!(f, "]"),
      Self::Comma => write!(f, ","),
      Self::Dot => write!(f, "."),
      Self::Greater => write!(f, ">"),
      Self::Less => write!(f, "<"),
      Self::Minus => write!(f, "-"),
      Self::Plus => write!(f, "+"),
      Self::Unknown => write!(f, "unknown"),
    }
  }
}
