#[cfg(not(feature = "library"))]
pub mod contract;
pub mod error;
pub mod execute;
pub mod indexed_set;
pub mod msg;
pub mod query;
pub mod state;
pub mod types;

pub const CONTRACT_NAME:    &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
