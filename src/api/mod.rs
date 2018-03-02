mod content_types;
mod health;
mod index;

use actix_web::{Application, HttpServer};

pub struct Api;

impl Api {
    pub fn new() -> Api {
        Api {}
    }

    pub fn run(&self) {
        let server = HttpServer::new(|| {
            Application::new()
                .resource("/", |resource| resource.f(index::handler))
                .resource("/health", |resource| resource.f(health::handler))
        });

        server
            .bind("0.0.0.0:8000")
            .expect("Unable to start server")
            .run()
    }
}
