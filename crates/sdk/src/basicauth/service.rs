use std::task::{Context, Poll};

use futures::future::BoxFuture;
use reqwest::Request;
use tower::Service;

use super::signer::BasicAuthSigner;

#[derive(Clone)]
pub struct BasicAuthService<S> {
    pub(crate) inner: S,
    pub(crate) signer: BasicAuthSigner,
}

impl<S> Service<Request> for BasicAuthService<S>
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
