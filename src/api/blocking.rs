use std::{str::FromStr, time::Duration};

use chrono::Utc;
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use url::Url;

use crate::api::{
    credential::Credential,
    error::{ApiError, Error},
    v5::ApiResponse,
};

use super::{v5::Request, Options};

// FIXME: mostly a copy paste from async variant
#[derive(Clone)]
pub struct Rest {
    options: Options,
    client: Client,
}

impl Rest {
    pub fn new(options: Options) -> Self {
        let mut headers = HeaderMap::new();

        let client = ClientBuilder::new()
            .default_headers(headers)
            .tcp_nodelay(true)
            .tcp_keepalive(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client, options }
    }

    #[inline]
    pub fn options(&self) -> &Options {
        &self.options
    }

    #[inline]
    pub fn request<R>(&self, req: R) -> crate::api::error::Result<R::Response>
    where
        R: Request,
    {
        let mut callback = || {};
        self.request_with(req, &mut callback)
    }

    pub fn request_with<R>(
        &self,
        req: R,
        on_send: &mut (dyn FnMut() + Sync + Send),
    ) -> crate::api::error::Result<R::Response>
    where
        R: Request,
    {
        let (params, body) = match R::METHOD {
            Method::GET => (Some(serde_qs::to_string(&req)?), String::new()),
            _ => (None, serde_json::to_string(&req)?),
        };
        let mut path = req.path().into_owned();
        if let Some(params) = params {
            if !params.is_empty() {
                path.push('?');
                path.push_str(&params);
            }
        }
        let url = format!("{}{}", self.options().rest(), path);
        log::debug!("{} {}", url, body);
        // example: 2020-12-08T09:08:57.715Z
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // TODO: reuse headers if possible
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        if R::AUTH {
            let passphrase = self
                .options()
                .passphrase
                .to_owned()
                .ok_or(Error::NoSecretConfigured)?;
            let credential: Credential = match self.options().try_into() {
                Ok(credential) => credential,
                Err(_) => return Err(Error::NoSecretConfigured),
            };

            let (key, signature) =
                credential.signature(R::METHOD, &timestamp, &Url::from_str(&url).unwrap(), &body);

            headers.insert(
                HeaderName::from_str("OK-ACCESS-KEY").unwrap(),
                HeaderValue::from_str(key).unwrap(),
            );
            headers.insert(
                HeaderName::from_str("OK-ACCESS-SIGN").unwrap(),
                HeaderValue::from_str(&signature).unwrap(),
            );
            headers.insert(
                HeaderName::from_str("OK-ACCESS-TIMESTAMP").unwrap(),
                HeaderValue::from_str(&timestamp).unwrap(),
            );
            headers.insert(
                HeaderName::from_str("OK-ACCESS-PASSPHRASE").unwrap(),
                HeaderValue::from_str(&passphrase).unwrap(),
            );
        }

        if let Some(extras) = self.options.env.headers() {
            for (key, val) in extras {
                headers.insert(
                    HeaderName::from_str(key).unwrap(),
                    HeaderValue::from_str(val).unwrap(),
                );
            }
        }

        let sent = match self
            .client
            .request(R::METHOD, &url)
            .headers(headers)
            .body(body)
            .send()
        {
            Ok(sent) => sent,
            Err(err) => {
                log::error!("{err}");
                return Err(Error::Reqwest(err));
            }
        };

        if let Err(err) = sent.error_for_status_ref() {
            return Err(Error::Reqwest(err));
        }
        on_send();

        let body = sent.bytes()?;

        // println!("{}", std::str::from_utf8(body.as_ref()).unwrap()); // DEBUG

        match serde_json::from_slice::<ApiResponse<R::Response>>(&body) {
            Ok(ApiResponse { code, msg, data }) => match code {
                Some(0) => {
                    if let Some(data) = data {
                        Ok(data)
                    } else {
                        Err(Error::Api(ApiError {
                            code,
                            msg: Some("Success but empty response".to_owned()),
                            data: None,
                            conn_id: None,
                        }))
                    }
                }
                code => Err(Error::Api(ApiError {
                    code,
                    msg,
                    data,
                    conn_id: None,
                })),
            },
            Err(e) => {
                log::error!("{}", String::from_utf8_lossy(&body));
                Err(Error::Json(e))
            }
        }
    }
}
