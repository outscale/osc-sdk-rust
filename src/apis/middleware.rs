use reqwest::Client;
use reqwest::{IntoUrl, Method};
use reqwest::{Request, Response};

use reqwest::RequestBuilder;

#[derive(Debug, Clone)]
pub struct ClientWithMiddleware {
    inner: Client,
    max_retries: i32,
    retry_backoff_factor: f32,
}

impl ClientWithMiddleware {
    pub fn new(inner: Client, max_retries: i32, retry_backoff_factor: f32) -> ClientWithMiddleware {
        ClientWithMiddleware {
            inner,
            max_retries,
            retry_backoff_factor,
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

        loop {
            let duplicate_request = request.try_clone().unwrap();
            let result = self.inner.execute(duplicate_request);

            match &result {
                Ok(res) => match res.status().as_u16() {
                    429 | 409 | 500..600 => {}
                    _ => return result,
                },
                Err(_) => {}
            }

            if i == self.max_retries {
                return result;
            }

            let backoff = self.retry_backoff_factor * 2_f32.powi(i);
            std::thread::sleep(std::time::Duration::from_secs_f32(backoff));
            i += 1;
        }
    }
}
