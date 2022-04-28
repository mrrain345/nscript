use std::env;
use std::path::PathBuf;

use inkwell::context::Context;
use nscript::compile;
use nscript::environment::Environment;

mod parser;
mod tokenizer;
mod nscript;
mod codegen;

fn main() {
  // Get the path to the file.
  let path = match env::args().nth(1) {
    Some(path) => PathBuf::from(path),
    None => {
      eprintln!("Usage: nscript <path>");
      return;
    }
  };

  // Get the contents of the file.
  let script = match std::fs::read_to_string(&path) {
    Ok(s) => s,
    Err(err) => {
      let path = 
      if path.is_absolute() { path }
      else { env::current_dir().unwrap().join(&path) };
    
      let path = path.to_str().unwrap();

      eprintln!("Failed to open a file \"{}\n{}\"", path, &err);
      return;
    }
  };

  // Create a new environment.
  let context = Context::create();
  let mut env = Environment::new(&context);

  // Parse the file.
  match parser::parse(&script) {
    Ok(expressions) => {

      // Compile the file.
      let main = compile(&mut env, &expressions);

      // Run the main function.
      unsafe {
        main.call();
      }
    }
    Err(err) => eprintln!("{}", &err),
  }
}