use crate::api::DemoTrading;
use crate::api::Options;
use crate::api::Rest;
use std::future::Future;

#[allow(clippy::manual_async_fn)]
pub fn with_env_private_client<C, Fut>(ctx: C) -> impl Future<Output = ()>
where
    C: FnOnce(Rest) -> Fut,
    Fut: Future<Output = ()>,
{
    dotenv::dotenv().expect("Failed to read .env file");

    async move {
        let key = std::env::var("OKX_API_KEY").expect("OKX_API_KEY not set");
        let secret = std::env::var("OKX_API_SECRET").expect("OKX_API_SECRET not set");
        let passphrase = std::env::var("OKX_API_PASSPHRASE").expect("OKX_API_PASSPHRASE not set");
        ctx(Rest::new(Options::new_with(
            DemoTrading,
            key,
            secret,
            passphrase,
        )))
        .await
    }
}
