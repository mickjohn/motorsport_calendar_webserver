use std::collections::HashMap;
use std::io::Read;
use rocket;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use serde_json;
use reqwest;
use motorsport_calendar_common::event::*;
use config;
use chrono::{DateTime, UTC, Datelike};

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

fn pretty_print_date_range(from: &DateTime<UTC>, to: &DateTime<UTC>) -> String {
    if from.month() == to.month() {
        format!("{day_from}-{day_to}, {month}",
                day_from = from.format("%d"),
                day_to = to.format("%d"),
                month = from.format("%B"),
        )
    } else if from.month() <= to.month() {
        format!("{day_from} {month_from} - {day_to} {month_to}",
                day_from = from.format("%d"),
                month_from = from.format("%B"),
                day_to = to.format("%d"),
                month_to = to.format("%B"),
        )
    } else {
        String::from(":(")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{UTC, TimeZone};

    #[test]
    fn test_pretty_print_date_range() {
        {
            let from = UTC.ymd(2017, 3, 23).and_hms(0, 0, 0);
            let to = UTC.ymd(2017, 3, 25).and_hms(0, 0, 0);
            assert_eq!(pretty_print_date_range(&from, &to), String::from("23-25, March"));
        }
        {
            let from = UTC.ymd(2017, 4, 30).and_hms(0, 0, 0);
            let to = UTC.ymd(2017, 5, 2).and_hms(0, 0, 0);
            assert_eq!(pretty_print_date_range(&from, &to), String::from("30 April - 02 May"));
        }
    }
}
