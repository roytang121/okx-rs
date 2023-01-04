use std::fmt::Debug;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error<T>>;

#[derive(Debug, Error)]
pub enum Error<T: Debug> {
    #[error("Api error: {0}")]
    Api(ApiError<T>),

    #[error("placing limit order requires price")]
    PlacingLimitOrderRequiresPrice,

    #[error("endpoint requires auth but no secret configured")]
    NoSecretConfigured,

    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub struct ApiError<T: Debug> {
    pub code: u32,
    pub msg: String,
    pub data: Option<T>,
}
