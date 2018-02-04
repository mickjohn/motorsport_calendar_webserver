#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate rocket;
extern crate motorsport_calendar_common;
#[macro_use] extern crate lazy_static;
extern crate reqwest;
extern crate chrono;
#[macro_use] extern crate tera;
extern crate hyper;
extern crate clap;

mod config;
mod webserver;
mod templates;
mod utils;

use config::Config;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Motorsport calendar webserver")
        .version("1.0")
        .author("Michael A. <mickjohnashe@hotmail.com>")
        .about("A webserver for serving mickjohn.com")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file, defaults to 'config.yml'")
             .takes_value(true))
        .get_matches();
    env_logger::init().unwrap();
    let config_file = matches.value_of("config").unwrap_or("config.yml");
    match run(&config_file) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run(config_file: &str) -> Result<(), String> {
    let config = try!(Config::init_config_from_file(config_file));
    init_global_config(config);
    webserver::run_webserver();
    Ok(())
}

fn init_global_config(c: Config) {
    let mut global_config = config::global::CONFIG.write().unwrap();
    *global_config = c;
}
