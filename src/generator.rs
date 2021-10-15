// generates from assets
// 1. create new folder
// 2. copy static files
// 3. create folder structure
// 4. generate posts
// 5. fill remaining templates
pub use crate::config::getconfig;
use dircpy::*;
use extract_frontmatter::Extractor;
use microtemplate::{render, Substitutions};
use slug::slugify;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
extern crate markdown;

#[derive(Substitutions)]
struct Post<'a> {
    title: &'a str, // automatically replaces "{title}" in a template
    body: &'a str,
}

pub fn build() {
    // get data from luna.ini
    let config = getconfig();
    // create base dirs
    fs::create_dir("./build");
    fs::create_dir("./build/posts");

    // copy over static assets
    copy_dir("./static", "./build/static");

    // loop over posts
    // read content
    // read header
    // parse markdowm
    // parse template
    // create new folder with slug name
    // create new file (index.html name)
    for entry in WalkDir::new("./posts").into_iter().filter_map(|e| e.ok()) {
        if Path::new(entry.path()).is_file() {
            let rawdata = fs::read_to_string(entry.path()).unwrap();
            let data = format!("{}", rawdata);

            let mut extractor = Extractor::new(&data);
            extractor.select_by_terminator("---");

            let (front_matter, document): (Vec<&str>, &str) = extractor.split();

            let front_matter = front_matter.join("\n");
            let document = document.trim();
            let info: Vec<&str> = front_matter.split(":").collect();
            let title = info[2].trim();
            let html: String = markdown::to_html(document);

            let postdata = Post {
                title: title,
                body: &html,
            };

            //read in templates
            let posttemplate = fs::read_to_string("./templates/post.html").unwrap();
            let rendered = render(&posttemplate, postdata);
            let slug = slugify(title);
            let postpath = "./build/posts/".to_string() + &slug;
            fs::create_dir(postpath);
            let filepath = format!("{}/{}", "./build/posts/".to_string() + &slug, "index.html");
            fs::write(filepath, rendered);
        }
    }
}
