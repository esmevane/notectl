use api::content_types;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result};
use serde_json;

#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
}

#[derive(Serialize)]
pub struct Health {
    status: HealthStatus,
}

impl Health {
    pub fn new() -> Health {
        Health {
            status: HealthStatus::Healthy,
        }
    }
}

impl Responder for Health {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _request: HttpRequest) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type(content_types::JSON)
            .body(body)?)
    }
}

pub fn handler(_request: HttpRequest) -> Health {
    Health::new()
}
