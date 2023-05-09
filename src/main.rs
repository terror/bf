use {
  crate::{interpreter::Interpreter, lexer::Lexer, repl::Repl, token::Token},
  rustyline::DefaultEditor,
  std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    io::stdin,
    process,
  },
};

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

mod interpreter;
mod lexer;
mod repl;
mod token;

fn run(args: Vec<String>) -> Result {
  match args.len() {
    1 => Repl::new()?.run(),
    _ => Interpreter::new().interpret(
      &Lexer::new()
        .load(fs::read_to_string(args[1].to_owned())?)
        .lex(),
    ),
  }
}

fn main() {
  if let Err(error) = run(env::args().collect()) {
    eprintln!("error: {}", error);
    process::exit(1);
  }
}
