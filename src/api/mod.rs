mod content_types;
mod health;
mod index;

use actix_web::{server, App};
use service::configuration::Configuration;

pub struct Api {
    configuration: Configuration,
}

impl Api {
    pub fn new() -> Api {
        Api {
            configuration: Configuration::new(),
        }
    }

    pub fn run(&self) {
        let api = server::new(|| {
            App::new()
                .resource("/", |resource| resource.f(index::handler))
                .resource("/health", |resource| resource.f(health::handler))
        });

        api.bind(format!(
            "{}:{}",
            self.configuration.host, self.configuration.port
        )).expect("Unable to start server")
            .run()
    }
}
