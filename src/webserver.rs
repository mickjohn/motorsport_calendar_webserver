use std::io::Read;
use rocket;
use rocket::response::NamedFile;
use rocket::response::content;
use rocket::Error as RocketError;
use reqwest;
use motorsport_calendar_common::event::*;
use config;
use tera::Context;
use serde_json;
use templates;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};

#[get("/")]
fn template() -> Result<content::HTML<String>, RocketError> {
    //If there's an error map it to RocketError::Internal to trigger 500 catcher
    match render_template() {
        Ok(rendered) => Ok(content::HTML(rendered)),
        Err(e) => {
            envlog_error!("Error getting events from API: '{}'", e); 
            Err(RocketError::Internal)
        },
    }
}

#[get("/static/<filename>")]
fn static_file(filename: &str) -> NamedFile {
    let fp = format!("static/{}", filename);
    NamedFile::open(&fp).unwrap()
}

// TODO shouldn't need an extra route for every dir.
#[get("/static/favicon/<filename>")]
fn favicons(filename: &str) -> NamedFile {
    let fp = format!("static/favicon/{}", filename);
    NamedFile::open(&fp).unwrap()
}

#[error(500)]
fn internal_server_error() -> NamedFile {
    let fp = "static/500error.html";
    NamedFile::open(&fp).unwrap()
}

pub fn run_webserver() {
    rocket::ignite()
        .mount("/", routes![
               static_file,
               favicons,
               template,
               ])
        .catch(errors![internal_server_error])
        .launch();
}

fn make_api_request() -> Result<Vec<Event>, String> {
    let config = config::global::CONFIG.read().unwrap();
    envlog_debug!("config = {:?}", *config);

    let content_type = ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]));
    let client = try!(reqwest::Client::new().map_err(|e| e.to_string()));
    let mut resp = try!(client.get(&config.api_url)
                        .header(content_type)
                        .send()
                        .map_err(|e| e.to_string()));

    if !resp.status().is_success() {
        return Err(format!("url {} returned non 200 status", &config.api_url));
    }

    let mut s = String::new();
    try!(resp.read_to_string(&mut s).map_err(|e| e.to_string()));
    envlog_debug!("repsonse from {} = {}", &config.api_url, s);

    let mut events: Vec<Event> = serde_json::from_str(&s).unwrap();
    events.sort_by(|a,b| a.start_date.cmp(&b.start_date));
    envlog_debug!("deserialized and sorted events = {:?}", events);
    Ok(events)
}

fn render_template() -> Result<String, String>{
    let mut context = Context::new();
    let events = try!(make_api_request());
    context.add("events", &events);
    context.add("sport_types", &get_sport_types(&events));
    let template = templates::init_template();
    let rendered_template = try!(template.render("index.html.tera", context).map_err(|e| e.to_string()));
    // let rendered_template = try!(templates::TERA.render("index.html.tera", context).map_err(|e| e.to_string()));
    Ok(rendered_template)
}

fn get_sport_types(events: &[Event]) -> Vec<&str> {
    let mut sport_types = events.iter().map(|e| e.sport.as_str()).collect::<Vec<&str>>();
    sport_types.sort();
    sport_types.dedup();
    sport_types
}

