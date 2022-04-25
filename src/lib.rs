#![warn(rust_2018_idioms)]
#![allow(unused_imports)]
#![allow(clippy::blacklisted_name)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Kube Api Error: {0}")]
    KubeError(#[source] kube::Error),

    #[error("SerializationError: {0}")]
    SerializationError(#[source] serde_json::Error),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// crd
/// Generated type, for crdgen
pub mod crd;
pub use crd::RSecret;

pub mod aws;
pub mod manager;
pub use manager::Manager;
