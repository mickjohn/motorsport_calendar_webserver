use chrono::prelude::*;
use chrono::Duration;
use config;
use motorsport_calendar_common::event::*;
use reqwest;
use reqwest::Client;
use rocket;
use rocket::response::content;
use rocket::response::NamedFile;
use serde_json;
use std::io::Read;
use std::path::{Path, PathBuf};
use templates;
use tera::Context;

#[get("/")]
fn template() -> content::Html<String> {
    content::Html(render_template().unwrap())
}

#[get("/events/<event_id>")]
fn event_template(event_id: i32) -> content::Html<String> {
    content::Html(render_event_template(event_id).unwrap())
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    let config = config::global::CONFIG.read().unwrap();
    NamedFile::open(Path::new(&config.static_content_dir).join(file)).ok()
}

#[catch(500)]
fn internal_server_error() -> NamedFile {
    let config = config::global::CONFIG.read().unwrap();
    let fp = format!("{}/500error.html", config.static_content_dir);
    NamedFile::open(&fp).unwrap()
}

pub fn run_webserver() {
    rocket::ignite()
        .mount("/", routes![static_file, template, event_template,])
        .register(catchers![internal_server_error])
        .launch();
}

fn make_api_request() -> Result<Vec<Event>, String> {
    let config = config::global::CONFIG.read().unwrap();
    rlog_debug!("config = {:?}", *config);

    let client = Client::new();
    let mut resp = client
        .get(&config.api_url)
        .header(reqwest::header::CONTENT_TYPE, "json")
        .send()
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("url {} returned non 200 status", &config.api_url));
    }

    let mut s = String::new();
    try!(resp.read_to_string(&mut s).map_err(|e| e.to_string()));
    rlog_debug!("repsonse from {} = {}", &config.api_url, s);

    let mut events: Vec<Event> = serde_json::from_str(&s).unwrap();
    events.sort_by(|a, b| a.get_start_date().cmp(&b.get_start_date()));
    rlog_debug!("deserialized and sorted events = {:?}", events);
    Ok(events)
}

// fn render_template(offset: i32) -> Result<String, String>{
fn render_template() -> Result<String, String> {
    let mut context = Context::new();
    let events = try!(make_api_request());
    // Don't display event's that are over a day old.
    let events_older_than_yesterday = get_events_older_than_yesterday(events);
    context.insert("events", &events_older_than_yesterday);
    context.insert(
        "sport_types",
        &get_sport_types(&events_older_than_yesterday),
    );
    // context.add("offset", &offset);

    let template = templates::init_template();
    let rendered_template = try!(template
        .render("index.html.tera", &context)
        .map_err(|e| e.to_string()));
    Ok(rendered_template)
}

fn render_event_template(event_id: i32) -> Result<String, String> {
    let config = config::global::CONFIG.read().unwrap();
    let client = Client::new();
    let url = format!("{}/{}/", &config.api_url, event_id);

    let mut resp = client
        .get(&url)
        .header(reqwest::header::CONTENT_TYPE, "json")
        .send()
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("url {} returned non 200 status", &config.api_url));
    }

    let mut event_string = String::new();
    try!(resp
        .read_to_string(&mut event_string)
        .map_err(|e| e.to_string()));

    let event: Event = serde_json::from_str(&event_string).unwrap();

    let mut context = Context::new();
    context.insert("event", &event);
    let template = templates::init_template();
    let rendered_template = try!(template
        .render("event.html.tera", &context)
        .map_err(|e| e.to_string()));
    Ok(rendered_template)
}

fn get_events_older_than_yesterday(events: Vec<Event>) -> Vec<Event> {
    rlog_debug!("About to filter events older than one day");
    let now: DateTime<Utc> = Utc::now();
    let one_day = Duration::seconds(60 * 60 * 24);
    events
        .into_iter()
        .filter(|x| x.get_end_date().is_some())
        // .filter(|x| { now.signed_duration_since(x.get_end_date().unwrap().and_hms(0,0,0)) <= one_day })
        .collect::<Vec<Event>>()
}

fn get_sport_types(events: &[Event]) -> Vec<&str> {
    let mut sport_types = events
        .iter()
        .map(|e| e.sport.as_str())
        .collect::<Vec<&str>>();
    sport_types.sort();
    sport_types.dedup();
    sport_types
}
