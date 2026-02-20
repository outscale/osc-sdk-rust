#[derive(Clone)]
pub struct SigV4Config {
    pub access_key: String,
    pub secret_key: String,
    pub service: String,
    pub region: String,
    pub session_token: Option<String>,
}
