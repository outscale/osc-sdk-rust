use secrecy::SecretString;

#[derive(Clone)]
pub struct SigOksConfig {
    pub access_key: SecretString,
    pub secret_key: SecretString,
}
