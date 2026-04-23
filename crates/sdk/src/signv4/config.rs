use secrecy::SecretString;

#[derive(Clone)]
pub struct SigV4Config {
    pub access_key: SecretString,
    pub secret_key: SecretString,
    pub service: String,
    pub region: String,
    pub session_token: Option<SecretString>,
}
