// webserver for development
// lots of work to do here
pub use crate::config::getconfig;
pub use crate::generator::build;
use ascii::AsciiString;
use std::fs;
use std::path::Path;

extern crate ascii;
extern crate tiny_http;

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

pub fn run(argport: Option<i32>) {
    let config = getconfig();

    // trigger build before serve
    build();

    let mut port = 0;

    if argport.is_none() {
        port = config.port
    } else {
        port = argport.unwrap();
    }

    let server = tiny_http::Server::http("0.0.0.0:".to_string() + &port.to_string()).unwrap();
    let port = server.server_addr().port();
    println!("Now listening on http://localhost:{}", port);

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break,
        };

        println!("{:?}", rq);
        let root = String::from("./build");
        let url = rq.url().to_string();
        let strpath = format!("{}{}", root, url);
        let path = Path::new(&strpath);
        let file = fs::File::open(&path);

        if file.is_ok() {
            let response = tiny_http::Response::from_file(file.unwrap());

            let response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(get_content_type(&path)).unwrap(),
            });

            let _ = rq.respond(response);
        } else {
            // code to look for index files
            let strpath = format!("{}{}{}", root, url, "/index.html");
            let path = Path::new(&strpath);
            let file = fs::File::open(&path);
            if file.is_ok() {
                let response = tiny_http::Response::from_file(file.unwrap());

                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii(get_content_type(&path)).unwrap(),
                });
                let _ = rq.respond(response);
            } else {
                let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
                let _ = rq.respond(rep);
            }
        }
    }
}
