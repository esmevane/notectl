extern crate actix_web;
extern crate clap;
extern crate daemonize;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{App, SubCommand};
use api::Api;
use service::Service;

mod api;
mod service;

fn start() {
    let service = Service::empty();

    service.start(|| Api::new().run());
}

fn stop() {
    let service = Service::empty();

    service.stop();
}

fn print_pid() {
    let service = Service::empty();
    match service.id() {
        Some(pid) => println!("{}", pid),
        None => println!("Unable to locate running notectl process"),
    }
}

fn main() {
    let matches = App::new("notectl")
        .version("0.0.1")
        .about("A note handling core service")
        .author("Joseph McCormick <esmevane@gmail.com>")
        .subcommand(SubCommand::with_name("start").about("Starts the service"))
        .subcommand(SubCommand::with_name("stop").about("Stops the service"))
        .subcommand(
            SubCommand::with_name("pid").about("Returns the process id"),
        )
        .get_matches();

    match matches.subcommand() {
        ("start", Some(_)) => start(),
        ("stop", Some(_)) => stop(),
        ("pid", Some(_)) => print_pid(),
        _ => (),
    };
}
