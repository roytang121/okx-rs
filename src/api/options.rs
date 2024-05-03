use std::sync::Arc;

#[derive(Clone)]
pub struct Production;

impl OKXEnv for Production {
    fn rest(&self) -> &str {
        "https://www.okx.com/api/v5"
    }

    fn public_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/public"
    }

    fn private_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/private"
    }

    fn business_websocket(&self) -> &str {
        "wss://ws.okx.com:8443/ws/v5/business"
    }
}

#[derive(Clone)]
pub struct DemoTrading;

impl OKXEnv for DemoTrading {
    fn rest(&self) -> &str {
        "https://www.okx.com/api/v5"
    }

    fn public_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999"
    }

    fn private_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999"
    }

    fn business_websocket(&self) -> &str {
        "wss://wspap.okx.com:8443/ws/v5/business?brokerId=9999"
    }

    fn headers(&self) -> Option<&[(&str, &str)]> {
        Some(&[("x-simulated-trading", "1")])
    }
}

pub trait OKXEnv {
    fn rest(&self) -> &str;
    fn public_websocket(&self) -> &str;
    fn private_websocket(&self) -> &str;
    fn business_websocket(&self) -> &str;
    fn headers(&self) -> Option<&[(&str, &str)]> {
        None
    }
}

#[derive(Clone)]
pub struct Options {
    pub env: Arc<dyn OKXEnv>,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub passphrase: Option<String>,
}

impl Options {
    pub fn new(env: impl OKXEnv + 'static) -> Options {
        Self {
            env: Arc::new(env),
            key: None,
            secret: None,
            passphrase: None,
        }
    }

    pub fn new_with(
        env: impl OKXEnv + 'static,
        key: impl AsRef<str>,
        secret: impl AsRef<str>,
        passphrase: impl AsRef<str>,
    ) -> Self {
        Self {
            env: Arc::new(env),
            key: Some(key.as_ref().to_string()),
            secret: Some(secret.as_ref().to_string()),
            passphrase: Some(passphrase.as_ref().to_string()),
        }
    }
}

impl Options {
    pub fn rest(&self) -> &str {
        self.env.rest()
    }
    pub fn public_websocket(&self) -> &str {
        self.env.public_websocket()
    }
    pub fn private_websocket(&self) -> &str {
        self.env.private_websocket()
    }
    pub fn business_websocket(&self) -> &str {
        self.env.business_websocket()
    }
}
