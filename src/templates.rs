use utils;
use tera;
use tera::Tera;
use motorsport_calendar_common::event::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*.tera");
        tera.register_filter("event_date_range", event_date_range_helper);
        tera.register_filter("session_date", session_date_helper);
        tera
    };
}

pub fn init_template() -> Tera {
    let mut tera = compile_templates!("templates/**/*.tera");
    tera.register_filter("event_date_range", event_date_range_helper);
    tera.register_filter("session_date", session_date_helper);
    tera
}

pub fn event_date_range_helper(value: tera::Value, _: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let event = try_get_value!("event_date_range", "value", Event, value);
    let s = utils::pretty_print_date_range(&event.start_date, &event.end_date);
    Ok(tera::to_value(&s).unwrap())
}

pub fn session_date_helper(value: tera::Value, _: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let session = try_get_value!("session_date", "value", Session, value);
    let s = utils::pretty_print_session_date_and_time(&session.date, &session.time);
    Ok(tera::to_value(&s).unwrap())
}
