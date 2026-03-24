use tower::Layer;

use super::{config::SigOksConfig, service::SigOksService, signer::SigOksSigner};

pub(crate) struct SigOksLayer {
    signer: SigOksSigner,
}

impl SigOksLayer {
    pub fn new(config: SigOksConfig) -> Self {
        Self {
            signer: SigOksSigner::new(config),
        }
    }
}

impl<S> Layer<S> for SigOksLayer {
    type Service = SigOksService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SigOksService {
            inner,
            signer: self.signer.clone(),
        }
    }
}
