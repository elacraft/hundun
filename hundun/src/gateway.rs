use async_trait::async_trait;
use log::{debug, error, info};
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};

use crate::error::GatewayError;
use crate::providers::create_provider;
use crate::providers::{OpenAIProvider, Provider};

pub struct Context {
    provider: Box<dyn Provider>,
}

pub struct Gateway {}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Context;
    fn new_ctx(&self) -> Self::CTX {
        Context {
            provider: "default".to_string(),
        }
    }

    async fn early_request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<()> {
        debug!("early_request_filter, {:?}", session.req_header().headers);
        ctx.provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("openai")
            .to_string();
        Ok(())
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .map(|v| v.to_str());
        match provider {
            Some(provider) => {
                debug!("provider: {:?}, ctx.provider: {}", provider, ctx.provider);
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
            _ => ("0.0.0.0", 6666), // fixme: use a default provider
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
