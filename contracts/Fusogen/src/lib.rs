pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;

pub mod ibc;

pub use error::FusogenError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::FusogenInterface;

pub use msg::{FusogenExecuteMsgFns, FusogenQueryMsgFns};

pub const FUSOGEN_NAMESPACE: &str = "fusogen";
pub const FUSOGEN_NAME: &str = "proto";
pub const FUSOGEN_ID: &str = const_format::concatcp!(FUSOGEN_NAMESPACE, ":", FUSOGEN_NAME);
