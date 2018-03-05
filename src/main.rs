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

trait Command {
    fn about(&self) -> &'static str;
    fn execute(&self);
    fn init() -> Self;
    fn name(&self) -> &'static str;
}

struct Pid {}

impl Command for Pid {
    fn init() -> Self
    where
        Self: Command,
    {
        Pid {}
    }

    fn about(&self) -> &'static str {
        "Displays the PID"
    }

    fn execute(&self) {
        let service = Service::empty();
        match service.id() {
            Some(pid) => println!("{}", pid),
            None => println!("Unable to locate running notectl process"),
        }
    }

    fn name(&self) -> &'static str {
        "pid"
    }
}

struct Start {}

impl Command for Start {
    fn init() -> Self
    where
        Self: Command,
    {
        Start {}
    }

    fn about(&self) -> &'static str {
        "Starts the service"
    }

    fn execute(&self) {
        let service = Service::empty();

        service.start(|| Api::new().run());
    }

    fn name(&self) -> &'static str {
        "start"
    }
}

struct Stop {}

impl Command for Stop {
    fn init() -> Self
    where
        Self: Command,
    {
        Stop {}
    }

    fn about(&self) -> &'static str {
        "Stops the service"
    }

    fn execute(&self) {
        let service = Service::empty();
        service.stop();
    }

    fn name(&self) -> &'static str {
        "stop"
    }
}

fn main() {
    let start = Start::init();
    let stop = Stop::init();
    let pid = Pid::init();

    let matches = App::new("notectl")
        .version("0.0.1")
        .about("A note handling core service")
        .author("Joseph McCormick <esmevane@gmail.com>")
        .subcommand(SubCommand::with_name(start.name()).about(start.about()))
        .subcommand(SubCommand::with_name(stop.name()).about(stop.about()))
        .subcommand(SubCommand::with_name(pid.name()).about(pid.about()))
        .get_matches();

    match matches.subcommand_name() {
        Some(command) if command == start.name() => start.execute(),
        Some(command) if command == stop.name() => stop.execute(),
        Some(command) if command == pid.name() => pid.execute(),
        _ => (),
    };
}
