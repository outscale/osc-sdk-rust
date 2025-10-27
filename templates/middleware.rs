use reqwest::blocking::Client;
use reqwest::blocking::{Request, Response};
use reqwest::{IntoUrl, Method};

use rand::Rng;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Deserializer};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BackoffParams {
    pub max_retries: i32,
    pub retry_backoff_factor: f32,
    pub retry_backoff_jitter: f32,
    pub retry_backoff_max: f32,
}

impl<'de> Deserialize<'de> for BackoffParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            max_retries: Option<i32>,
            retry_backoff_factor: Option<f32>,
            retry_backoff_jitter: Option<f32>,
            retry_backoff_max: Option<f32>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let default = BackoffParams::default();

        Ok(BackoffParams {
            max_retries: helper.max_retries.unwrap_or(default.max_retries),
            retry_backoff_factor: helper
                .retry_backoff_factor
                .unwrap_or(default.retry_backoff_factor),
            retry_backoff_jitter: helper
                .retry_backoff_jitter
                .unwrap_or(default.retry_backoff_jitter),
            retry_backoff_max: helper
                .retry_backoff_max
                .unwrap_or(default.retry_backoff_max),
        })
    }
}

impl Default for BackoffParams {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_backoff_factor: 1.0,
            retry_backoff_jitter: 3.0,
            retry_backoff_max: 60.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LimiterParams {
    pub max_requests: usize,
    pub window: Duration,
}

impl<'de> Deserialize<'de> for LimiterParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            max_requests: Option<usize>,
            window: Option<Duration>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let default = LimiterParams::default();

        Ok(LimiterParams {
            max_requests: helper.max_requests.unwrap_or(default.max_requests),
            window: helper.window.unwrap_or(default.window),
        })
    }
}

impl Default for LimiterParams {
    fn default() -> Self {
        Self {
            max_requests: 5,
            window: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Limiter {
    params: LimiterParams,
    requests: VecDeque<Instant>,
}

impl Limiter {
    pub fn new(params: LimiterParams) -> Self {
        Self {
            params,
            requests: VecDeque::new(),
        }
    }

    pub fn acquire(&mut self) {
        let now = Instant::now();
        self.cleanup_old_requests(now);

        // If we're at the limit, wait until we can make another request
        if self.requests.len() >= self.params.max_requests {
            if let Some(oldest) = self.requests.front() {
                let wait_time = self
                    .params
                    .window
                    .saturating_sub(now.duration_since(*oldest));
                if !wait_time.is_zero() {
                    std::thread::sleep(wait_time);
                }

                // Clean up again after sleeping
                let now = Instant::now();
                self.cleanup_old_requests(now);
            }
        }

        // Record this request
        self.requests.push_back(now);
    }

    /// Removes requests that are outside the current time window
    fn cleanup_old_requests(&mut self, now: Instant) {
        if let Some(cutoff) = now.checked_sub(self.params.window) {
            while let Some(&front_time) = self.requests.front() {
                if front_time <= cutoff {
                    self.requests.pop_front();
                } else {
                    break;
                }
            }
        } else {
            self.requests.clear();
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientWithBackoff {
    inner: Client,
    params: BackoffParams,
    limiter: Arc<Mutex<Limiter>>,
}

impl ClientWithBackoff {
    pub fn new(inner: Client, params: BackoffParams, limiter_params: LimiterParams) -> Self {
        Self {
            inner,
            params,
            limiter: Arc::new(Mutex::new(Limiter::new(limiter_params))),
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.inner.request(method, url)
    }

    pub fn execute(&self, request: Request) -> reqwest::Result<Response> {
        let mut i: i32 = 0;
        let mut rng = rand::rng();

        {
            let mut limiter = self.limiter.lock().unwrap();
            limiter.acquire();
        }

        loop {
            let duplicate_request = request.try_clone().unwrap();
            let result = self.inner.execute(duplicate_request);

            if let Ok(res) = &result {
                match res.status().as_u16() {
                    429 | 409 | 500..600 => {}
                    _ => return result,
                }
            }

            if i == self.params.max_retries {
                return result;
            }

            let backoff = (self.params.retry_backoff_factor * 2_f32.powi(i)
                + rng.random_range(0.0..=self.params.retry_backoff_jitter))
            .min(self.params.retry_backoff_max);
            std::thread::sleep(std::time::Duration::from_secs_f32(backoff));
            i += 1;
        }
    }
}
