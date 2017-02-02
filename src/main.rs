#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate motorsport_calendar_common;
#[macro_use] extern crate lazy_static;
extern crate reqwest;
extern crate chrono;

mod config;
mod webserver;

use config::Config;

fn main() {
    env_logger::init().unwrap();
    match run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run() -> Result<(), String> {
    let config = try!(Config::init_config_from_file("config.yml"));
    init_global_config(config);
    webserver::run_webserver();
    Ok(())
}

fn init_global_config(c: Config) {
    let mut global_config = config::global::CONFIG.write().unwrap();
    *global_config = c;
}
