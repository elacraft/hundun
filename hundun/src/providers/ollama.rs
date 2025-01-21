// Copyright 2025 Elacraft LLC.

use crate::providers::Provider;
use log::debug;
use pingora_http::ResponseHeader;

pub struct OllamaProvider {
    pub name: String,
    pub base_url: String,
    pub tls: bool,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            name: "ollama".to_string(),
            base_url: "http://127.0.0.1:11434".to_string(),
            tls: false,
        }
    }
}

impl Provider for OllamaProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn tls(&self) -> bool {
        self.tls
    }

    fn process_request_header(&self, _upstream_request: &mut pingora_http::RequestHeader) {
        debug!("ollama process_request_header");
    }
    fn process_respsonse_header(&self, _upstream_request: &mut ResponseHeader) {
        debug!("ollama process_respsonse_header");
    }
}
