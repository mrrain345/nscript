use std::env;
use std::path::Path;
use std::path::PathBuf;

use inkwell::context::Context;
use nscript::Environment;
use nscript::compile;
// use nscript::compile;
// use nscript::Environment;

mod parser;
mod tokenizer;
mod nscript;
mod codegen;

fn main() {
  // Enable better panic.
  better_panic::install();

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

  // Tokenize
  let tokens = tokenizer::parse(&script);

  // Create a new environment.
  let context = Context::create();
  let mut env = Environment::new(&context);

  // Parse the file.
  match parser::parse(&tokens) {
    Ok(expressions) => {

      // Compile the file.
      let main = compile(&mut env, &expressions);

      // Save output LLVM IR to file.
      env.borrow_mut().module.print_to_file(Path::new("target/output.ll")).unwrap();

      // Run the main function.
      unsafe {
        main.call();
      }
    }
    Err(err) => eprintln!("[Parser error] {:?}", &err),
  }
}