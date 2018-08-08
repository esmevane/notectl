extern crate actix_web;
extern crate clap;
extern crate daemonize;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use clap::{App, SubCommand};
use api::Api;
use operation::Operation;
use service::Service;

mod api;
mod operation;
mod service;

fn main() {
    let service = Service::empty();

    let stop = Operation::new("stop", "Stops the service", || service.stop());
    let start = Operation::new("start", "Starts the service", || {
        service.start(|| Api::new().run())
    });

    let pid =
        Operation::new("pid", "Displays the service pid", || {
            match service.id() {
                Some(pid) => println!("{}", pid),
                None => println!("Unable to locate running notectl process"),
            }
        });

    let matches = App::new("notectl")
        .version("0.0.1")
        .about("A note handling core service")
        .author("Joseph McCormick <esmevane@gmail.com>")
        .subcommand(SubCommand::with_name(start.name).about(start.about))
        .subcommand(SubCommand::with_name(stop.name).about(stop.about))
        .subcommand(SubCommand::with_name(pid.name).about(pid.about))
        .get_matches();

    match matches.subcommand_name() {
        Some(command) if command == start.name => start.perform(),
        Some(command) if command == stop.name => stop.perform(),
        Some(command) if command == pid.name => pid.perform(),
        _ => (),
    };
}
