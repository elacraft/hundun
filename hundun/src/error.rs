// Copyright 2025 Elacraft LLC.

use pingora_core::{Error, ErrorSource, ErrorType};
use std::fmt;

#[derive(Debug)]
pub enum GatewayError {
    InvalidMethod,
    InvalidAPIKey,
    InvalidProvider,
}

impl fmt::Display for GatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GatewayError::InvalidMethod => write!(f, "Invalid HTTP method"),
            GatewayError::InvalidAPIKey => write!(f, "Invalid API key"),
            GatewayError::InvalidProvider => write!(f, "Invalid provider"),
        }
    }
}

impl std::error::Error for GatewayError {}

impl GatewayError {
    pub fn to_error(&self) -> Box<Error> {
        match self {
            GatewayError::InvalidMethod => {
                Error::create(ErrorType::ConnectError, ErrorSource::Upstream, None, None)
            }
            GatewayError::InvalidAPIKey => Error::create(
                ErrorType::InvalidHTTPHeader,
                ErrorSource::Upstream,
                None,
                None,
            ),
            GatewayError::InvalidProvider => {
                Error::create(ErrorType::UnknownError, ErrorSource::Unset, None, None)
            }
        }
    }
}
