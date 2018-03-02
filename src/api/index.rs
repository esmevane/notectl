use api::content_types;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result};
use serde_json;

#[derive(Serialize)]
pub struct Index {}

impl Index {
    pub fn new() -> Index {
        Index {}
    }
}

impl Responder for Index {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _request: HttpRequest) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type(content_types::JSON)
            .body(body)?)
    }
}

pub fn handler(_request: HttpRequest) -> Index {
    Index::new()
}
