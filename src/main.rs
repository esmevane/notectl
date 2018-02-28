#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate clap;
extern crate daemonize;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use clap::{App, SubCommand};
use rocket_contrib::Json;

mod service;

use service::Service;

#[derive(Serialize, Deserialize)]
enum HealthStatus {
    Healthy,
    Unhealthy,
}

#[derive(Serialize, Deserialize)]
struct Health {
    status: HealthStatus,
}

#[derive(Serialize, Deserialize)]
struct Index {}

#[get("/")]
fn index() -> Option<Json<Index>> {
    Some(Json(Index {}))
}

#[get("/health")]
fn health() -> Option<Json<Health>> {
    Some(Json(Health {
        status: HealthStatus::Healthy,
    }))
}

fn start() {
    let routes = routes![index, health];
    let service = Service::empty();

    service.start(routes);
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
