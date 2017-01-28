use std::collections::HashMap;
use std::io::Read;
use rocket;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use serde_json;
use reqwest;
use motorsport_calendar_common::event::*;
use config;

#[get("/")]
fn template() -> Template {
    let mut context = HashMap::new();
    context.insert("events", make_api_request().unwrap());
    Template::render("index", &context)
}

#[get("/static/<filename>")]
fn static_file(filename: &str) -> NamedFile {
    let fp = format!("static/{}", filename);
    NamedFile::open(&fp).unwrap()
}

pub fn run_webserver() {
    rocket::ignite()
        .mount("/", routes![
               static_file,
               template,
               ])
        .launch();
}

fn make_api_request() -> Result<Vec<Event>, String> {
    let config = config::global::CONFIG.read().unwrap();
    debug!("config = {:?}", *config);
    let mut resp = try!(reqwest::get(&config.api_url)
                    .map_err(|e| e.to_string()));


    if !resp.status().is_success() {
        return Err(format!("url {} reuturned non 200 status", &config.api_url));
    }

    let mut s = String::new();
    try!(resp.read_to_string(&mut s).map_err(|e| e.to_string()));
    debug!("repsonse from {} = {}", &config.api_url, s);

    let mut events: Vec<Event> = serde_json::from_str(&s).unwrap();
    events.sort_by(|a,b| a.start_date.cmp(&b.start_date));
    debug!("deserialized and sorted events = {:?}", events);
    Ok(events)
}
