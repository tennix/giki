#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate git2;
extern crate comrak;
#[macro_use]
extern crate lazy_static;

use git2::Repository;
use comrak::{markdown_to_html, ComrakOptions};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> String {
    markdown_to_html("Hello", &ComrakOptions::default())
}

fn main() {
    let path = "/path/to/repo";
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(err) => panic!(err),
    };
    rocket::ignite().mount("/", routes![hello]).launch();
}
