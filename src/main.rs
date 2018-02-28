#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate clap;
extern crate daemonize;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use std::fs::{remove_file, File};
use std::process::Command;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use clap::{App, SubCommand};
use daemonize::Daemonize;
use rocket_contrib::Json;

mod service;

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

fn start() -> Result<(), Error> {
    if is_running() {
        println!("Notectl process already running.");
        Ok(())
    } else {
        Daemonize::new()
            .working_directory("/tmp")
            .pid_file("/tmp/notectl.pid")
            .start()
            .unwrap();

        rocket::ignite().mount("/", routes![index, health]).launch();

        Ok(())
    }
}

fn stop() -> Result<(), Error> {
    if is_running() {
        let mut file = File::open("/tmp/notectl.pid")?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        Command::new("kill")
            .arg(contents)
            .output()
            .expect("Couldn't kill notectl program");
        remove_file("/tmp/notectl.pid")?;

        Ok(())
    } else {
        println!("Unable to find running notectl process.");
        Ok(())
    }
}

fn is_running() -> bool {
    std::path::Path::new("/tmp/notectl.pid").exists()
}

fn read_pidfile() -> Result<String, Error> {
    if is_running() {
        let mut file = File::open("/tmp/notectl.pid")?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Unable to locate notectl pidfile",
        ))
    }
}

fn pid() -> Option<usize> {
    match read_pidfile() {
        Ok(pidfile_contents) => match pidfile_contents.parse::<usize>() {
            Ok(parsed) => Some(parsed),
            Err(_) => None,
        },

        Err(_) => None,
    }
}

fn print_pid() -> Result<(), Error> {
    match pid() {
        Some(pid) => println!("{}", pid),
        None => println!("Unable to locate running notectl process"),
    }

    Ok(())
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

    println!("{:?}", matches);

    let _outcome = match matches.subcommand() {
        ("start", Some(_)) => start(),
        ("stop", Some(_)) => stop(),
        ("pid", Some(_)) => print_pid(),
        _ => Ok(()),
    };

    println!("{:?}", std::env::current_dir());

    ()
}
