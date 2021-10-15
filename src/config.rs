//config is parsed here
// lots of work to do
extern crate tini;
use tini::Ini;

// main config struct
pub struct Config {
    pub port: i32,
    pub postspath: String,
    pub templatespath: String,
    pub outputbuildpath: String,
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
