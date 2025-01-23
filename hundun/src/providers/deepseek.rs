// Copyright 2025 Elacraft LLC.

use crate::providers::Provider;
use log::debug;
use pingora_http::ResponseHeader;

pub struct DeepseekProvider {
    pub name: String,
    pub base_url: String,
    pub tls: bool,
}

impl Default for DeepseekProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DeepseekProvider {
    pub fn new() -> Self {
        Self {
            name: "deepseek".to_string(),
            base_url: "https://api.deepseek.com".to_string(),
            tls: true,
        }
    }
}

impl Provider for DeepseekProvider {
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
        debug!("Deepseek process_request_header");
    }

    fn process_response_header(&self, _upstream_request: &mut ResponseHeader) {
        debug!("Deepseek process_response_header");
    }
}
