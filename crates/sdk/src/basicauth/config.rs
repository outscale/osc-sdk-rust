use secrecy::SecretString;

#[derive(Clone)]
pub struct BasicAuthConfig {
    pub username: SecretString,
    pub password: SecretString,
}
