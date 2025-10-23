use reqwest::blocking::Client;
use reqwest::blocking::{Request, Response};
use reqwest::{IntoUrl, Method};

use rand::Rng;
use reqwest::blocking::RequestBuilder;

#[derive(Debug, Clone)]
pub struct ClientWithBackoff {
    inner: Client,
    max_retries: i32,
    retry_backoff_factor: f32,
    retry_backoff_jitter: f32,
    retry_backoff_max: f32,
}

impl ClientWithBackoff {
    pub fn new(
        inner: Client,
        max_retries: i32,
        retry_backoff_factor: f32,
        retry_backoff_jitter: f32,
        retry_backoff_max: f32,
    ) -> Self {
        Self {
            inner,
            max_retries,
            retry_backoff_factor,
            retry_backoff_jitter,
            retry_backoff_max,
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

        loop {
            let duplicate_request = request.try_clone().unwrap();
            let result = self.inner.execute(duplicate_request);

            if let Ok(res) = &result {
                match res.status().as_u16() {
                    429 | 409 | 500..600 => {}
                    _ => return result,
                }
            }

            if i == self.max_retries {
                return result;
            }

            let backoff = (self.retry_backoff_factor * 2_f32.powi(i)
                + rng.random_range(0.0..=self.retry_backoff_jitter))
            .min(self.retry_backoff_max);
            std::thread::sleep(std::time::Duration::from_secs_f32(backoff));
            i += 1;
        }
    }
}
