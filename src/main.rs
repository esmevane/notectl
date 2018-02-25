#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate daemonize;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use daemonize::Daemonize;
use rocket_contrib::Json;

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

fn main() {
    Daemonize::new()
        .working_directory("/tmp")
        .pid_file("/tmp/notectl.pid")
        .start()
        .unwrap();

    rocket::ignite().mount("/", routes![index, health]).launch();
}
