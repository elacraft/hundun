// Copyright 2025 Elacraft LLC.

pub mod error;
pub mod gateway;
pub mod providers;

use log::info;

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_proxy::http_proxy_service;

use crate::gateway::Gateway;

/*
RUST_LOG=DEBUG cargo run
curl 127.0.0.1:2025/api/tags -H "x-ai-provider: ollama"
curl 127.0.0.1:2025/v1/models -H "x-ai-provider: openai" -H "Authorization: Bearer OPENAI_API_KEY"
*/
fn main() {
    env_logger::init();

    let opt = Opt::parse_args();
    let mut server = Server::new(Some(opt)).unwrap();
    server.bootstrap();

    let mut gw = http_proxy_service(&server.configuration, Gateway {});
    gw.add_tcp("0.0.0.0:2025");
    server.add_service(gw);
    info!("starting server");

    server.run_forever();
}
