use std::sync::Arc;

use chrono::Utc;
use log::debug;
use reqwest::{Request, header::HeaderValue};

use super::config::SigV4Config;
use secrecy::{ExposeSecret, SecretSlice, SecretString};

use hmac::Mac as _;
use sha2::Digest as _;
type HmacSha256 = hmac::Hmac<sha2::Sha256>;

static EMPTY_BODY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

#[derive(Clone)]
pub(crate) struct SigV4Signer {
    config: Arc<SigV4Config>,
}

impl SigV4Signer {
    pub fn new(config: SigV4Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    fn get_signature_key(&self, date_stamp: &str) -> Result<SecretSlice<u8>, ()> {
        fn sign(key: &[u8], msg: &[u8]) -> Result<SecretSlice<u8>, ()> {
            let mut mac = HmacSha256::new_from_slice(key).map_err(|_| ())?;
            mac.update(msg);
            Ok(SecretSlice::from(mac.finalize().into_bytes().to_vec()))
        }

        let k_date = sign(
            format!("OSC4{}", self.config.secret_key.expose_secret()).as_bytes(),
            date_stamp.as_bytes(),
        )?;
        let k_region = sign(k_date.expose_secret(), self.config.region.as_bytes())?;
        let k_service = sign(k_region.expose_secret(), self.config.service.as_bytes())?;
        sign(k_service.expose_secret(), b"osc4_request")
    }

    fn get_canonical_request(&self, request: &Request, timestamp: &str) -> String {
        let canonical_uri = request.url().path();
        let canonical_query = request.url().query().unwrap_or("");
        let canonical_headers = format!(
            "host:{}\nx-osc-date:{}\n",
            request.url().host_str().unwrap_or(""),
            timestamp
        );

        let payload_hash = match request.body().and_then(|body| body.as_bytes()) {
            Some(b) => {
                let hash = sha2::Sha256::digest(b);
                format!("{:x}", hash)
            }
            None => EMPTY_BODY_SHA256.to_string(),
        };

        format!(
            "{method}\n{canonical_uri}\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{payload_hash}",
            method = request.method().as_str(),
            canonical_uri = canonical_uri,
            canonical_query = canonical_query,
            canonical_headers = canonical_headers,
            signed_headers = "host;x-osc-date",
            payload_hash = payload_hash,
        )
    }

    fn get_authorization_header(&self, credential_scope: &str, signature: &SecretString) -> String {
        format!(
            "OSC4-HMAC-SHA256 Credential={}/{}, SignedHeaders=host;x-osc-date, Signature={}",
            self.config.access_key.expose_secret(),
            credential_scope,
            signature.expose_secret(),
        )
    }

    pub(crate) fn sign(&self, request: &mut Request) -> Result<(), ()> {
        let now = Utc::now();
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let datestamp = now.format("%Y%m%d").to_string();

        // Create a canonical request
        let canonical_request = self.get_canonical_request(request, &timestamp);
        debug!("canonical_request: {}", canonical_request);

        // Create the string to sign
        let credential_scope = format!(
            "{datestamp}/{region}/{service}/osc4_request",
            datestamp = datestamp,
            region = self.config.region,
            service = self.config.service
        );
        debug!("credential_scope: {}", credential_scope);
        let string_to_sign = format!(
            "OSC4-HMAC-SHA256\n{}\n{}\n{:x}",
            timestamp,
            credential_scope,
            sha2::Sha256::digest(canonical_request.as_bytes())
        );
        debug!("string_to_sign: {}", string_to_sign);

        let signing_key = self.get_signature_key(&datestamp)?;
        let signature: SecretString = {
            let mut mac =
                HmacSha256::new_from_slice(signing_key.expose_secret()).map_err(|_| ())?;
            mac.update(string_to_sign.as_bytes());
            SecretString::from(format!("{:x}", mac.finalize().into_bytes()))
        };

        // Add Signing information to the request
        let authorization_header = self.get_authorization_header(&credential_scope, &signature);
        let mut authorization_header =
            HeaderValue::from_str(&authorization_header).map_err(|_| ())?;
        authorization_header.set_sensitive(true);

        let timestamp = HeaderValue::from_str(&timestamp).map_err(|_| ())?;
        request.headers_mut().insert("x-osc-date", timestamp);
        request
            .headers_mut()
            .insert("Authorization", authorization_header);

        if let Some(ref token) = self.config.session_token {
            let mut token = HeaderValue::from_str(token.expose_secret()).map_err(|_| ())?;
            token.set_sensitive(true);

            request.headers_mut().insert("X-Osc-Security-Token", token);
        }
        debug!("req: {:?}", request);

        Ok(())
    }
}
