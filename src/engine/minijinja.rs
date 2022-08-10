use crate::TemplateEngine;

use super::Engine;

use axum::{http::StatusCode, response::IntoResponse};
use minijinja::{Environment, Error};
use thiserror::Error;

impl TemplateEngine for Engine<Environment<'static>> {
    type Error = MinijinjaError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let template = self.engine.get_template(key)?;
        let rendered = template.render(&data)?;

        Ok(rendered)
    }
}

#[derive(Error, Debug)]
pub enum MinijinjaError {
    #[error(transparent)]
    RenderError(#[from] Error),
}

impl IntoResponse for MinijinjaError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_response()
    }
}
