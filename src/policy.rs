use std::time::Duration;

use tower::retry::Policy;
use tower::retry::backoff::{Backoff, ExponentialBackoff, ExponentialBackoffMaker, MakeBackoff};
use tower::util::rng::HasherRng;

#[derive(Clone)]
pub struct BasePolicy {
    attemps: usize,
    backoff: ExponentialBackoff,
}

impl BasePolicy {
    pub fn new(attemps: usize, min: Duration, max: Duration, jitter: f64) -> Self {
        let backoff = ExponentialBackoffMaker::new(min, max, jitter, HasherRng::new())
            .unwrap()
            .make_backoff();

        BasePolicy { attemps, backoff }
    }
}

impl Policy<reqwest::Request, reqwest::Response, reqwest::Error> for BasePolicy {
    type Future = tokio::time::Sleep;

    fn retry(
        &mut self,
        _req: &mut reqwest::Request,
        result: &mut Result<reqwest::Response, reqwest::Error>,
    ) -> Option<Self::Future> {
        match result {
            Ok(r) => match r.status().as_u16() {
                429 | 500..600 => {}
                _ => return None,
            },
            Err(e) if e.is_builder() => return None,
            _ => {}
        }

        if self.attemps > 0 {
            self.attemps -= 1;
            Some(self.backoff.next_backoff())
        } else {
            None
        }
    }

    fn clone_request(&mut self, req: &reqwest::Request) -> Option<reqwest::Request> {
        req.try_clone()
    }
}

pub(crate) fn default_pooled_transport() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .tcp_keepalive(Duration::from_secs(30))
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .http2_adaptive_window(true)
}
