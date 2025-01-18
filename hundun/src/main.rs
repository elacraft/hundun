// Copyright 2025 Elacraft LLC.

use async_trait::async_trait;
use log::{debug, info};

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};

pub struct AIGateway {}

#[async_trait]
impl ProxyHttp for AIGateway {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .map(|v| v.as_bytes());
        match provider {
            Some(provider) => {
                debug!("provider: {:?}", provider);
            }
            _ => {
                info!("Header x-ai-provider is needed");
                let _ = session.respond_error(403).await;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .and_then(|v| v.to_str().ok());

        let addr = match provider {
            Some("openai") => ("api.openai.com", 443),
            Some("ollama") => ("localhost", 11434),
            _ => ("0.0.0.0", 6666),
        };

        debug!("connecting to {addr:?}");

        let sni = addr.0.to_string();
        let tls = if addr.1 == 443 { true } else { false };
        let peer = Box::new(HttpPeer::new(addr, tls, sni));
        info!("tls: {tls}, peer: {peer}, peer: {peer}");
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .and_then(|v| v.to_str().ok());
        if provider == Some("openai") {
            let _ = upstream_request.insert_header("Host", "api.openai.com");
        }
        upstream_request.remove_header("x-ai-provider");
        Ok(())
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        upstream_response.insert_header("Server", "Hundun").unwrap();

        Ok(())
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora_core::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );
    }
}

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

    let mut gateway = pingora_proxy::http_proxy_service(&server.configuration, AIGateway {});
    gateway.add_tcp("0.0.0.0:2025");
    server.add_service(gateway);
    info!("starting server");

    server.run_forever();
}
