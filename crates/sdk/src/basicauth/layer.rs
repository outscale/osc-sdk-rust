use tower::Layer;

use super::{config::BasicAuthConfig, service::BasicAuthService, signer::BasicAuthSigner};

pub(crate) struct BasicAuthLayer {
    signer: BasicAuthSigner,
}

impl BasicAuthLayer {
    pub fn new(config: BasicAuthConfig) -> Self {
        Self {
            signer: BasicAuthSigner::new(config),
        }
    }
}

impl<S> Layer<S> for BasicAuthLayer {
    type Service = BasicAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BasicAuthService {
            inner,
            signer: self.signer.clone(),
        }
    }
}
