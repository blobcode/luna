extern crate include_dir;

mod args;
mod config;
mod generator;
mod new;
mod serve;

pub use crate::args::{submain, AppArgs};
pub use crate::config::getconfig;
pub use crate::generator::build;
pub use crate::new::init;
pub use crate::serve::run;

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
  -c, --config          Path to project's luna.ini

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
        // call generator function here
        build();
      }
      AppArgs::New { name, path } => {
        // copy fs from memory
        // implement path
        init(name, "./".to_string());
      }
      AppArgs::Serve { port } => {
        run(port);
        print!("server running")
      }
    },
    Err(e) => eprintln!("Error: {:?}.", e),
  }
}
