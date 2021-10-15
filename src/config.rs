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
    let data = Ini::from_file(path).unwrap();
    return data;
}

// main function
// reads file and then translates it to struct
pub fn getconfig() -> Config {
    // parse and print args
    // todo: parse config file
    let confile = readfile("./luna.ini");
    let config = Config {
        port: 8000,
        postspath: String::from("./posts"),
        templatespath: String::from("./templates"),
        outputbuildpath: String::from("./build"),
    };
    return config;
}
