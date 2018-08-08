mod content_types;
mod health;
mod index;

use actix_web::{server, App};

pub struct Api;

impl Api {
    pub fn new() -> Api {
        Api {}
    }

    pub fn run(&self) {
        let api = server::new(|| {
            App::new()
                .resource("/", |resource| resource.f(index::handler))
                .resource("/health", |resource| resource.f(health::handler))
        });

        api.bind("0.0.0.0:8000")
            .expect("Unable to start server")
            .run()
    }
}
