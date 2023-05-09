use super::*;

#[derive(Debug)]
pub(crate) struct Interpreter {
  pointer: usize,
  state: Vec<u8>,
}

impl Interpreter {
  pub(crate) fn new() -> Self {
    Self {
      pointer: 0,
      state: vec![0],
    }
  }

  pub(crate) fn reset(&mut self) {
    self.pointer = 0;
    self.state = vec![0];
  }

  pub(crate) fn interpret(&mut self, tokens: &[Token]) -> Result {
    let mut index = 0;

    let mut stack = Vec::new();

    while index < tokens.len() {
      match tokens[index] {
        Token::Dot => {
          if let Some(value) = self.state.get(self.pointer) {
            print!("{}", *value as char);
          }
        }
        Token::Greater => {
          self.pointer += 1;

          if self.pointer >= self.state.len() {
            self.state.push(0);
          }
        }
        Token::Less => {
          if self.pointer == 0 {
            continue;
          }

          self.pointer -= 1;
        }
        Token::Minus => {
          if let Some(value) = self.state.get_mut(self.pointer) {
            *value -= 1;
          }
        }
        Token::Plus => {
          if let Some(value) = self.state.get_mut(self.pointer) {
            *value += 1;
          }
        }
        Token::Comma => {
          if let Some(value) = self.state.get_mut(self.pointer) {
            let input = stdin();
            let mut buffer = String::new();
            input.read_line(&mut buffer)?;
            *value = buffer.as_bytes()[0];
          }
        }
        Token::BraceL => {
          if let Some(value) = self.state.get(self.pointer) {
            if *value == 0 {
              let mut count = 1;

              while count > 0 {
                index += 1;

                match tokens[index] {
                  Token::BraceL => count += 1,
                  Token::BraceR => count -= 1,
                  _ => {}
                }
              }
            } else {
              stack.push(index);
            }
          }
        }
        Token::BraceR => {
          if let Some(value) = self.state.get(self.pointer) {
            if *value != 0 {
              index = stack.last().unwrap().clone();
            } else {
              stack.pop();
            }
          }
        }
        Token::Unknown => {
          index += 1;
          continue;
        }
      }

      index += 1;
    }

    Ok(())
  }
}
