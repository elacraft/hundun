// Copyright 2025 Elacraft LLC.

use pingora_core::upstreams::peer::HttpPeer;

mod ollama;
mod openai;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;

use crate::error::GatewayError;

pub trait Provider {
    fn name(&self) -> String;
    fn base_url(&self) -> String;
    fn peer(&self) -> Box<HttpPeer>;
}

pub fn create_provider(name: &str) -> Result<Box<dyn Provider>, GatewayError> {
    match name {
        "openai" => Ok(Box::new(OpenAIProvider::new())),
        "ollama" => Ok(Box::new(OllamaProvider::new())),
        _ => Err(GatewayError::InvalidProvider),
    }
}
