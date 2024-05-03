use super::Options;
use anyhow::{bail, ensure};
use base64::encode;
use hmac::{Hmac, Mac};
use reqwest::{Method, Url};
use sha2::Sha256;
use std::convert::TryFrom;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug)]
pub struct Credential {
    key: String,
    secret: String,
}

impl Credential {
    pub fn new(key: &str, secret: &str) -> Self {
        Self {
            key: key.into(),
            secret: secret.into(),
        }
    }

    pub(crate) fn signature(
        &self,
        method: Method,
        timestamp: &str,
        url: &Url,
        body: &str,
    ) -> (&str, String) {
        // sign=CryptoJS.enc.Base64.stringify(CryptoJS.HmacSHA256(timestamp + 'GET' + '/users/self/verify' + body, SecretKey))
        // let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        let sign_message = match url.query() {
            Some(query) => format!(
                "{}{}{}?{}{}",
                timestamp,
                method.as_str(),
                url.path(),
                query,
                body
            ),
            None => format!("{}{}{}{}", timestamp, method.as_str(), url.path(), body),
        };

        mac.update(sign_message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let signature = encode::<&[u8]>(code_bytes.as_ref());

        // let signature = encode(hmac::sign(&signed_key, sign_message.as_bytes()).as_ref());
        (self.key.as_str(), signature)
    }

    pub(crate) fn signature_ws(
        &self,
        method: Method,
        timestamp: &str,
        url: &str,
    ) -> (&str, String) {
        // sign=CryptoJS.enc.Base64.stringify(CryptoJS.HmacSHA256(timestamp + 'GET' + '/users/self/verify' + body, SecretKey))
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        // let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
        let sign_message = format!("{}{}{}", timestamp, method.as_str(), url);

        mac.update(sign_message.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        let signature = encode::<&[u8]>(code_bytes.as_ref());

        // let signature = encode(hmac::sign(&signed_key, sign_message.as_bytes()).as_ref());
        (self.key.as_str(), signature)
    }
}

impl TryFrom<&Options> for Credential {
    type Error = anyhow::Error;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        ensure!(options.key.is_some(), "key is not set");
        ensure!(options.secret.is_some(), "secret is not set");
        if let (Some(key), Some(secret)) = (&options.key, &options.secret) {
            Ok(Self {
                key: key.to_owned(),
                secret: secret.to_owned(),
            })
        } else {
            bail!("not enough credentials from Options")
        }
    }
}
