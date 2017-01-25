#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate motorsport_calendar_common;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use motorsport_calendar_common::event::*;
use std::collections::HashMap;

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("static/index.html").unwrap()
}

#[get("/template")]
fn template() -> Template {
    let mut context = HashMap::new();
    context.insert("events", make_api_request());
    Template::render("index", &context)
}

#[get("/static/<filename>")]
fn static_file(filename: &str) -> NamedFile {
    let fp = format!("static/{}", filename);
    NamedFile::open(&fp).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               static_file,
               template,
               ]).launch();
}

fn make_api_request() -> Vec<Event> {
    let s = include_str!("res.json");
    let mut events: Vec<Event> = serde_json::from_str(s).unwrap();
    events.sort_by(|a,b| a.start_date.cmp(&b.start_date));
    events
}
