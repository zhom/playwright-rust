use crate::imp::{cdp_session::CdpSession as Impl, core::*, prelude::*};

/// `CDPSession` instances are used to talk raw Chrome DevTools Protocol.
///
/// Obtain via [`BrowserContext::new_cdp_session`](crate::api::BrowserContext::new_cdp_session).
/// Calls `session.send(method, params)` for protocol methods. Available only
/// on Chromium-based browsers.
pub struct CdpSession {
    inner: Weak<Impl>,
}

impl CdpSession {
    pub(crate) fn new(inner: Weak<Impl>) -> Self {
        Self { inner }
    }

    /// Send a raw CDP method with optional JSON params. Returns the `result`
    /// payload from the CDP response (i.e. without the Playwright wrapper).
    pub async fn send(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
    ) -> ArcResult<serde_json::Value> {
        upgrade(&self.inner)?.send(method, params).await
    }

    /// Detach the session from its target. After this, no further calls or
    /// events are dispatched.
    pub async fn detach(&self) -> ArcResult<()> {
        upgrade(&self.inner)?.detach().await
    }
}
