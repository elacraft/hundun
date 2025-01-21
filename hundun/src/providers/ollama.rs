// Copyright 2025 Elacraft LLC.

use crate::providers::Provider;
use pingora_core::upstreams::peer::HttpPeer;

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

    fn peer(&self) -> Box<pingora::prelude::HttpPeer> {
        let hostname = self.base_url.split('/').nth(2).unwrap();
        let port = self
            .base_url
            .split(':')
            .nth(2)
            .unwrap()
            .parse::<u16>()
            .unwrap();
        Box::new(HttpPeer::new(
            (hostname, port),
            self.tls,
            hostname.to_string(),
        ))
    }
}
