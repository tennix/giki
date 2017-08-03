#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate git2;
extern crate comrak;
#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use git2::Repository;
use comrak::{markdown_to_html, ComrakOptions};

const REPO_PATH: &'static str = "wiki";

#[get("/")]
fn index() -> String {
    markdown_to_html("show index", &ComrakOptions::default())
}

#[get("/p/<page>")]
fn page(page: String) -> String {
    let p = Path::new(REPO_PATH).join(page).with_extension("md");
    let f = File::open(p).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);
    markdown_to_html(&buf, &ComrakOptions::default())
}

#[post("/p/<page>", data="<page>")]
fn new_page(page: String) -> String {
    markdown_to_html("create new page", &ComrakOptions::default())
}

#[delete("/p/<page>")]
fn delete_page(page: String) -> String {
    markdown_to_html("delete page", &ComrakOptions::default())
}

#[get("/c/<catagory>")]
fn catagory(catagory: String) -> String {
    markdown_to_html("show catagory", &ComrakOptions::default())
}

#[post("/c/<catagory>", data="<catagory>")]
fn new_catagory(catagory: String) -> String {
    markdown_to_html("create new catagory", &ComrakOptions::default())
}

#[delete("/c/<catagory>")]
fn delete_catagory(catagory: String) -> String {
    markdown_to_html("delete catagory", &ComrakOptions::default())
}

fn main() {
    let repo = match Repository::open(REPO_PATH) {
        Ok(repo) => repo,
        Err(err) => panic!(err),
    };
    rocket::ignite().mount(
        "/",
        routes![
            index,
            page,
            new_page,
            delete_page,
            catagory,
            new_catagory,
            delete_catagory,
        ]
    ).launch();
}
