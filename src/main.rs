extern crate include_dir;

mod args;
mod new;
pub use crate::args::{submain, AppArgs};
pub use crate::new::init;

const HELPTEXT: &str = r#" 
     _.._
   .' .-'`  ,--.          
  /  /      |  |,--.,--.,--,--,  ,--,--. 
  |  |      |  ||  ||  ||      \' ,-.  | 
  \  \      |  |'  ''  '|  ||  |\ '-'  | 
   '._'-._  `--' `----' `--''--' `--`--' 
      ```
       a simple static site generator

USAGE:
  luna [COMMAND] [OPTIONS]
COMMANDS:
  help                  Prints help information
  new                   Generates new luna project
  serve                 Serves up your site
  build                 Compiles your site
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  --output PATH         Sets an output path

"#;

fn main() {
    // main application logic

    match submain() {
        Ok(args) => match args {
            AppArgs::Global { help } => {
                // implement help function here
                print!("{}", HELPTEXT);
            }
            AppArgs::Help {} => {
                print!("{}", HELPTEXT);
            }
            AppArgs::Build { output } => {
                // call assembler function here
                println!("placeholder")
            }
            AppArgs::New { name, path } => {
                // copy fs from memory
                init(name, "./".to_string());
            }
        },
        Err(e) => eprintln!("Error: {:?}.", e),
    }
}
