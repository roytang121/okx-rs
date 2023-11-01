use std::ops::Deref;
use ::chrono::{DateTime, Utc};

pub mod chrono {
    pub use chrono::*;
}

pub type UTCDateTime = DateTime<Utc>;
pub trait TimeExt {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

impl TimeExt for UTCDateTime {

}