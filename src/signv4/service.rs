use std::task::{Context, Poll};

use futures::future::BoxFuture;
use reqwest::{IntoUrl, Request, RequestBuilder};
use tower::Service;

use super::signer::SigV4Signer;

pub struct SigV4Service<S> {
    pub(crate) inner: S,
    pub(crate) signer: SigV4Signer,
}

impl<S> Service<Request> for SigV4Service<S>
where
    S: Service<Request, Response = reqwest::Response> + Send,
    S::Future: Send + 'static,
{
    type Response = reqwest::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        match self.signer.sign(&mut request) {
            Ok(_) => {
                let fut = self.inner.call(request);
                Box::pin(fut)
            }
            Err(_err) => unimplemented!(),
        }
    }
}
