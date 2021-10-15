//config is parsed here
extern crate tini;
use tini::Ini;

// main config struct
pub struct Config {
    pub inputpath: Option<String>,
}

fn readfile(path: &str) -> Ini {
    let conf = Ini::from_file(path).unwrap();
    return conf;
}

// main function
pub fn getconfig() -> Config {
    // parse and print args
    // parse config file
    let confile = readfile("./luna.ini");
    let config = Config { inputpath: None };
    return config;
}
