// This needs a lot of work

pub use crate::config::getconfig;
use dircpy::*;
use extract_frontmatter::Extractor;
use microtemplate::{render, Substitutions};
use slug::slugify;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
extern crate markdown;

#[derive(Substitutions)]
struct Post<'a> {
    title: &'a str, // automatically replaces "{title}" in a template
    body: &'a str,
}
#[derive(Substitutions)]
struct Home<'a> {
    posts: &'a str, // automatically replaces "{title}" in a template
}
#[derive(Substitutions)]
struct Listing<'a> {
    title: &'a str, // automatically replaces "{title}" in a template
    slug: &'a str,
}
#[derive(Substitutions)]
struct Postsdata<'a> {
    posts: &'a str,
}

struct Postdata {
    title: String,
    slug: String,
}

// todo: use config for paths
// also maybe dynamic structure?
pub fn build() {
    let mut postlist: Vec<Postdata> = Vec::new();
    // get data from luna.ini
    let config = getconfig();
    // create base dirs
    fs::create_dir("./build").unwrap();
    fs::create_dir("./build/posts").unwrap();

    // copy over static assets
    copy_dir("./static", "./build/static").unwrap();

    // walk over posts
    for entry in WalkDir::new("./posts").into_iter().filter_map(|e| e.ok()) {
        if Path::new(entry.path()).is_file() {
            let rawdata = fs::read_to_string(entry.path()).unwrap();
            let data = rawdata.to_string();

            let mut extractor = Extractor::new(&data);
            extractor.select_by_terminator("---");

            let (front_matter, document): (Vec<&str>, &str) = extractor.split();

            let front_matter = front_matter.join("\n");
            let document = document.trim();
            let info: Vec<&str> = front_matter.split(':').collect();
            let title = info[2].trim();
            let html: String = markdown::to_html(document);
            let postdata = Post { title, body: &html };

            //read in templates
            let posttemplate = fs::read_to_string("./templates/post.html").unwrap();
            let rendered = render(&posttemplate, postdata);
            let slug = slugify(title);
            let postpath = "./build/posts/".to_string() + &slug;
            fs::create_dir(postpath).unwrap();
            let filepath = format!("{}/{}", "./build/posts/".to_string() + &slug, "index.html");
            fs::write(filepath, rendered).unwrap();

            let postinfo = Postdata {
                title: String::from("test"),
                slug,
            };
            postlist.push(postinfo);
        }
    }

    //read in templates
    let hometemplate = fs::read_to_string("./templates/home.html").unwrap();
    let poststemplate = fs::read_to_string("./templates/posts.html").unwrap();
    let listingtemplate = fs::read_to_string("./templates/postlisting.html").unwrap();

    let mut html = String::from("");
    for post in postlist {
        let listing = Listing {
            title: &post.title,
            slug: &post.slug,
        };
        let rendered = render(&listingtemplate, listing);
        html = format!("{}{}", html, &rendered);
    }
    let homedata = Home { posts: &html };
    let postlistdata = Postsdata { posts: &html };

    let rendered = render(&hometemplate, homedata);
    let renderedpostlist = render(&poststemplate, postlistdata);
    fs::write("./build/index.html", rendered).unwrap();
    fs::write("./build/posts/index.html", renderedpostlist).unwrap();
}
