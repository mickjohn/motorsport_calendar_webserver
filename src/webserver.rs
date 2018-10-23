use std::io::Read;
use std::path::{Path, PathBuf};
use rocket;
use rocket::response::NamedFile;
use rocket::response::content;
use rocket::Error as RocketError;
use reqwest;
use reqwest::Client;
use motorsport_calendar_common::event::*;
use config;
use tera::Context;
use serde_json;
use templates;
use chrono::prelude::*;
use chrono::Duration;

#[derive(FromForm)]
struct UtcOffsetSeconds {
    offset: i32
}

#[get("/")]
fn template() -> Result<content::Html<String>, RocketError> {
    let now: DateTime<Local> = Local::now();
    let offset = now.offset().fix().utc_minus_local();
    match render_template() {
        Ok(rendered) => Ok(content::Html(rendered)),
        Err(e) => {
            rlog_error!("Error getting events from API: '{}'", e); 
            Err(RocketError::Internal)
        },
    }
}

#[get("/?<offset>")]
fn template_with_offset(offset: UtcOffsetSeconds) -> Result<content::Html<String>, RocketError> {
    match render_template() {
        Ok(rendered) => Ok(content::Html(rendered)),
        Err(e) => {
            rlog_error!("Error getting events from API: '{}'", e); 
            Err(RocketError::Internal)
        },
    }
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    let config = config::global::CONFIG.read().unwrap();
    NamedFile::open(Path::new(&config.static_content_dir).join(file)).ok()
}

#[error(500)]
fn internal_server_error() -> NamedFile {
    let config = config::global::CONFIG.read().unwrap();
    let fp = format!("{}/500error.html", config.static_content_dir);
    NamedFile::open(&fp).unwrap()
}

pub fn run_webserver() {
    rocket::ignite()
        .mount("/", routes![
               static_file,
               template,
               template_with_offset,
               ])
        .catch(errors![internal_server_error])
        .launch();
}

fn make_api_request() -> Result<Vec<Event>, String> {
    let config = config::global::CONFIG.read().unwrap();
    rlog_debug!("config = {:?}", *config);

    let client = Client::new().map_err(|e| e.to_string())?;
    let mut resp = client.get(&config.api_url).map_err(|e| e.to_string())?
        .header(reqwest::header::ContentType::json())
        .send().map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("url {} returned non 200 status", &config.api_url));
    }

    let mut s = String::new();
    try!(resp.read_to_string(&mut s).map_err(|e| e.to_string()));
    rlog_debug!("repsonse from {} = {}", &config.api_url, s);

    let mut events: Vec<Event> = serde_json::from_str(&s).unwrap();
    events.sort_by(|a,b| a.get_start_date().cmp(&b.get_start_date()));
    rlog_debug!("deserialized and sorted events = {:?}", events);
    Ok(events)
}

// fn render_template(offset: i32) -> Result<String, String>{
fn render_template() -> Result<String, String>{
    let mut context = Context::new();
    let events = try!(make_api_request());
    // Don't display event's that are over a day old.
    let events_older_than_yesterday = get_events_older_than_yesterday(events);
    context.add("events", &events_older_than_yesterday);
    context.add("sport_types", &get_sport_types(&events_older_than_yesterday));
    // context.add("offset", &offset);

    let template = templates::init_template();
    let rendered_template = try!(template.render("index.html.tera", &context).map_err(|e| e.to_string()));
    Ok(rendered_template)
}

fn get_events_older_than_yesterday(events: Vec<Event>) -> Vec<Event> {
    rlog_debug!("About to filter events older than one day");
    let now: DateTime<Utc> = Utc::now();
    let one_day = Duration::seconds(60*60*24);
    events.into_iter()
        .filter(|x| x.get_end_date().is_some())
        .filter(|x| { now.signed_duration_since(x.get_end_date().unwrap().and_hms(0,0,0)) <= one_day })
        .collect::<Vec<Event>>()
}

fn get_sport_types(events: &[Event]) -> Vec<&str> {
    let mut sport_types = events.iter().map(|e| e.sport.as_str()).collect::<Vec<&str>>();
    sport_types.sort();
    sport_types.dedup();
    sport_types
}
