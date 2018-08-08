use actix_web::{Error, HttpRequest, HttpResponse};
use api::content_types;
use serde_json;

#[derive(Serialize)]
pub struct Index {}

impl Index {
    pub fn new() -> Index {
        Index {}
    }
}

pub fn handler(_request: &HttpRequest) -> Result<HttpResponse, Error> {
    let index = Index::new();
    let body = serde_json::to_string(&index)?;

    Ok(HttpResponse::Ok()
        .content_type(content_types::JSON)
        .body(body))
}
