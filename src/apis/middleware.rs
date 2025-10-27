use reqwest::blocking::Client;
use reqwest::blocking::{Request, Response};
use reqwest::{IntoUrl, Method};

use rand::Rng;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct BackoffParams {
    max_retries: i32,
    retry_backoff_factor: f32,
    retry_backoff_jitter: f32,
    retry_backoff_max: f32,
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

#[derive(Debug, Clone, Default)]
pub struct ClientWithBackoff {
    inner: Client,
    params: BackoffParams,
}

impl ClientWithBackoff {
    pub fn new(inner: Client, params: BackoffParams) -> Self {
        Self { inner, params }
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
