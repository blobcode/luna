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

// read in config file helper function
fn readfile(path: &str) -> Ini {
    Ini::from_file(path).unwrap()
}

// main function
// reads file and then translates it to struct
pub fn getconfig() -> Config {
    // parse and print args
    // todo: parse config file
    let ini = readfile("./luna.ini");
    let config = Config {
        port: ini.get("dev", "port").unwrap(),
        postspath: ini.get("build", "posts").unwrap(),
        templatespath: ini.get("build", "templates").unwrap(),
        outputbuildpath: ini.get("build", "output").unwrap(),
    };
    config
}
