// Copyright 2025 Elacraft LLC.

use pingora_core::upstreams::peer::HttpPeer;
use pingora_http::{RequestHeader, ResponseHeader};

mod deepseek;
mod ollama;
mod openai;
pub use deepseek::DeepseekProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;

pub trait Provider: Send + Sync {
    fn name(&self) -> String;
    fn base_url(&self) -> String;
    fn tls(&self) -> bool;
    fn peer(&self) -> Box<pingora::prelude::HttpPeer> {
        let base_url = url::Url::parse(&self.base_url()).unwrap();
        let hostname = base_url.host_str().unwrap().to_owned();
        let sni = hostname.clone();
        let port = base_url.port_or_known_default().unwrap();
        Box::new(HttpPeer::new((hostname, port), self.tls(), sni))
    }
    fn process_request_header(&self, upstream_request: &mut RequestHeader);
    fn process_response_header(&self, upstream_request: &mut ResponseHeader);
}

pub fn create_provider(name: &str) -> Option<Box<dyn Provider>> {
    match name {
        "openai" => Some(Box::new(OpenAIProvider::new())),
        "ollama" => Some(Box::new(OllamaProvider::new())),
        "deepseek" => Some(Box::new(DeepseekProvider::new())),
        _ => None,
    }
}
