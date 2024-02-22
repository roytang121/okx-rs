#[derive(Debug, Clone, Default)]
pub struct Options {
    pub key: Option<String>,
    pub secret: Option<String>,
    pub passphrase: Option<String>,
}
