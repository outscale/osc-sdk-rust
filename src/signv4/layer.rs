use tower::Layer;

use super::{config::SigV4Config, service::SigV4Service, signer::SigV4Signer};

pub(crate) struct SigV4Layer {
    signer: SigV4Signer,
}

impl SigV4Layer {
    pub fn new(config: SigV4Config) -> Self {
        Self {
            signer: SigV4Signer::new(config),
        }
    }
}

impl<S> Layer<S> for SigV4Layer {
    type Service = SigV4Service<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SigV4Service {
            inner,
            signer: self.signer.clone(),
        }
    }
}
