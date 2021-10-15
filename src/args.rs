// handles args

#[derive(Debug)]
pub enum AppArgs {
  Global { help: bool },
  New { name: String, path: Option<String> },
  Build { output: Option<String> },
  Serve { port: Option<i32> },
  Help {},
}

pub fn submain() -> Result<AppArgs, Box<dyn std::error::Error>> {
  let mut args = pico_args::Arguments::from_env();
  match args.subcommand()?.as_deref() {
    Some("new") => Ok(AppArgs::New {
      name: args.value_from_str(["-n", "--name"])?,
      path: args.opt_value_from_str(["-p", "--path"])?,
    }),
    Some("build") => Ok(AppArgs::Build {
      output: args.opt_value_from_str(["-o", "--output"])?,
    }),
    Some("serve") => Ok(AppArgs::Serve {
      port: args.opt_value_from_str(["-p", "--port"])?,
    }),
    Some("help") => Ok(AppArgs::Help {}),
    Some(_) => Err("unknown subcommand".into()),
    None => Ok(AppArgs::Global {
      help: args.contains(["-h", "--help"]),
    }),
  }
}
