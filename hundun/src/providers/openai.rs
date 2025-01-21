// Copyright 2025 Elacraft LLC.

use crate::providers::Provider;
use log::debug;
use pingora_http::ResponseHeader;

pub struct OpenAIProvider {
    pub name: String,
    pub base_url: String,
    pub tls: bool,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            name: "openai".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            tls: true,
        }
    }
}

impl Provider for OpenAIProvider {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn tls(&self) -> bool {
        self.tls
    }

    fn process_request_header(&self, upstream_request: &mut pingora_http::RequestHeader) {
        debug!("OpenAI process_request_header");
        upstream_request
            .insert_header("Host", "api.openai.com")
            .unwrap();
    }

    fn process_respsonse_header(&self, _upstream_request: &mut ResponseHeader) {
        debug!("ollama process_respsonse_header");
    }
}
