use utils;
use tera;
use tera::Tera;
use motorsport_calendar_common::event::*;
use std::collections::HashMap;
use config;
use chrono::prelude::*;

lazy_static! {
    pub static ref TERA: Tera = {
        let config = config::global::CONFIG.read().unwrap();
        let templates = format!("{}/**/*.tera", config.template_directory);
        let mut tera = compile_templates!(&templates);
        tera.register_filter("event_date_range", event_date_range_helper);
        tera.register_filter("session_date", session_date_helper);
        tera
    };
}

pub fn init_template() -> Tera {
    let config = config::global::CONFIG.read().unwrap();
    let templates = format!("{}/**/*.tera", config.template_directory);
    let mut tera = compile_templates!(&templates);
    tera.register_filter("event_date_range", event_date_range_helper);
    tera.register_filter("session_date", session_date_helper);
    tera
}

pub fn event_date_range_helper(value: tera::Value, _: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let event = try_get_value!("event_date_range", "value", Event, value);
    let s = utils::pretty_print_date_range(&event.get_start_date(), &event.get_end_date());
    Ok(tera::to_value(&s).unwrap())
}

pub fn session_date_helper(value: tera::Value, params: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let session = try_get_value!("session_date", "value", Session, value);
    let session_time = &session.time.map(|dt| {
         Local.from_utc_datetime(&dt.naive_local())
    });
    let s = if let Some(offset) = params.get("utc_offset") {
        let o = match offset {
            &tera::Value::Number(ref x) => Some(x.as_i64().unwrap() as i32),
            _ => None,
        };
        if let Some(i32_offset) = o {
            utils::pretty_print_session_date_and_time_with_offset(&session.time.unwrap(), &session.time, &i32_offset)
        } else {
            utils::pretty_print_session_date_and_time(&session.time.unwrap(), &session_time)
        }
    } else {
        utils::pretty_print_session_date_and_time(&session.time.unwrap(), &session_time)
    };
    Ok(tera::to_value(&s).unwrap())
}
