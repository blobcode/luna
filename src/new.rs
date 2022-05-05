// creates the fresh file structure
// copy from packaged binary (aka ../default)
use include_dir::include_dir;
use include_dir::Dir;

static DEFAULT: Dir = include_dir!("./default");

pub fn init(name: String, path: String) {
    DEFAULT.extract(path + &name).unwrap();
}
