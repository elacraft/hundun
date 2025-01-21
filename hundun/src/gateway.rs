use async_trait::async_trait;
use log::{debug, error, info};
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};

use crate::error::GatewayError;
use crate::providers::create_provider;
use crate::providers::Provider;

pub struct Context {
    provider: Box<dyn Provider>,
}

pub struct Gateway {}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Context;
    fn new_ctx(&self) -> Self::CTX {
        Context {
            provider: create_provider("openai").expect("Failed to create default provider"),
        }
    }

    async fn early_request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<()> {
        debug!("early_request_filter, {:?}", session.req_header().headers);
        let provider = session
            .req_header()
            .headers
            .get("x-ai-provider")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("empty");

        ctx.provider = match create_provider(provider) {
            Some(provider) => provider,
            _ => {
                error!("Invalid x-ai-provider Header");
                let _ = session.respond_error(403).await;
                return Err(GatewayError::InvalidProvider.to_error());
            }
        };

        Ok(())
    }

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        Ok(ctx.provider.peer())
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        ctx.provider.process_request_header(upstream_request);

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
            "{} response code: {response_code}, provider: {}",
            self.request_summary(session, ctx),
            ctx.provider.name()
        );
    }
}
