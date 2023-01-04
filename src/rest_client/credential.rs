use super::options::Options;
use base64::encode;
use reqwest::{Method, Url};
use ring::hmac;
use std::convert::TryFrom;

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
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
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

        let signature = encode(hmac::sign(&signed_key, sign_message.as_bytes()).as_ref());
        (self.key.as_str(), signature)
    }
}

impl TryFrom<&Options> for Credential {
    type Error = &'static str;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        if let (Some(key), Some(secret)) = (&options.key, &options.secret) {
            Ok(Self {
                key: key.to_owned(),
                secret: secret.to_owned(),
            })
        } else {
            Err("not enough credentials from Options")
        }
    }
}
