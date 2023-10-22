use crate::api::options::Options;
use crate::api::Rest;
use dotenv::dotenv;
use std::future::Future;

#[allow(clippy::manual_async_fn)]
pub fn with_public_client<C, Fut>(ctx: C) -> impl Future<Output = ()>
where
    C: FnOnce(Rest) -> Fut,
    Fut: Future<Output = ()>,
{
    async move {
        ctx(Rest::new(Options::default())).await;
    }
}

#[allow(clippy::manual_async_fn)]
pub fn with_env_private_client<C, Fut>(ctx: C) -> impl Future<Output = ()>
where
    C: FnOnce(Rest) -> Fut,
    Fut: Future<Output = ()>,
{
    dotenv().expect("Failed to read .env file");

    async move {
        let api_key = std::env::var("OKX_API_KEY").expect("OKX_API_KEY not set");
        let api_secret = std::env::var("OKX_API_SECRET").expect("OKX_API_SECRET not set");
        let api_passphrase =
            std::env::var("OKX_API_PASSPHRASE").expect("OKX_API_PASSPHRASE not set");
        ctx(Rest::new(Options {
            key: Some(api_key),
            secret: Some(api_secret),
            passphrase: Some(api_passphrase),
        }))
        .await;
    }
}
