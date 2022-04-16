use std::env;
use std::path::PathBuf;

mod parser;
mod tokenizer;

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

  // Parse the file.
  parser::parse(&script);
  // match parser::parse(script.as_str()) {
  //   Ok((output, remaining)) => {
  //     println!("Tokens:");

  //     output.iter().for_each(|token| {
  //       println!("{:?}", token);
  //     });

  //     println!("\nLength: {}", output.len());

  //     if remaining.input.len() > 0 {
  //       println!("\nRemaining (len: {}):\n{}", remaining.input.len(), remaining.input)
  //     }
  //   }
  //   Err(err) => eprintln!("{}", &err),
  // }
}