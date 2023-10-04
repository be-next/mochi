use axum::http::uri::PathAndQuery;
use axum::http::{Method, StatusCode};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub enum LatencyCore {
    Constant(u32),
}

#[derive(Clone, Debug)]
pub struct ApiCore(pub Vec<RuleCore>);

#[derive(Clone, Debug)]
pub struct ApiSetCore {
    pub name: String,
    pub shape: Option<Vec<EndpointCore>>,
    pub apis: Vec<ApiCore>,
}

#[derive(Clone, Debug)]
pub struct RuleCore {
    pub endpoint: EndpointCore,
    pub headers: HashMap<String, String>,
    pub latency: Option<LatencyCore>,
    pub status: StatusCode,
    pub format: String,
    pub body: Option<String>,
}

/// Implements the `Display` trait for `RuleCore`.
///
/// This allows for a human-readable representation of the `RuleCore` struct.
/// The displayed format includes the endpoint's route, a list of headers, the status code,
/// the response format, and optionally the latency if it is defined.
impl Display for RuleCore {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Endpoint: {} ", self.endpoint.route)?;

        for (key, value) in &self.headers {
            write!(f, "{}: {} ", key, value)?;
        }

        write!(f, "Status: {}, ", self.status)?;
        write!(f, "Format: {}, ", self.format)?;

        if let Some(latency) = &self.latency {
            write!(f, "Latency: {:?} ", latency.clone())?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct EndpointCore {
    pub route: PathAndQuery,
    pub method: Method,
}

#[derive(Clone, Debug)]
pub struct SystemCore {
    pub name: String,
    pub api_sets: Vec<ApiSetCore>,
}
