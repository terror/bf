use super::*;

#[derive(Debug)]
pub(crate) struct Repl {
  editor: DefaultEditor,
  interpreter: Interpreter,
  lexer: Lexer,
}

impl Repl {
  pub(crate) fn new() -> Result<Self> {
    Ok(Self {
      editor: DefaultEditor::new()?,
      interpreter: Interpreter::new(),
      lexer: Lexer::new(),
    })
  }

  pub(crate) fn run(mut self) -> Result {
    loop {
      let line = self.editor.readline("> ")?;
      self.interpreter.interpret(&self.lexer.load(line).lex())?;
      self.interpreter.reset();
    }
  }
}
