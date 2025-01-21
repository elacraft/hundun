// Copyright 2025 Elacraft LLC.

use crate::providers::Provider;
use pingora_core::upstreams::peer::HttpPeer;

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

    fn peer(&self) -> Box<pingora::prelude::HttpPeer> {
        let hostname = self.base_url.split('/').nth(2).unwrap();

        let port = if self.tls { 443 } else { 80 };
        Box::new(HttpPeer::new(
            (hostname, port),
            self.tls,
            hostname.to_string(),
        ))
    }
}
