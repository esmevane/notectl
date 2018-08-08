use actix_web::{Error, HttpRequest, HttpResponse};
use api::content_types;
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

pub fn handler(_request: &HttpRequest) -> Result<HttpResponse, Error> {
    let health = Health::new();
    let body = serde_json::to_string(&health)?;

    Ok(HttpResponse::Ok()
        .content_type(content_types::JSON)
        .body(body))
}
