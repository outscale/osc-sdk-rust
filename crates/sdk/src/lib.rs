#![deny(clippy::unwrap_used, clippy::print_stderr, clippy::print_stdout)]
pub mod basicauth;
pub mod errors;
pub mod policy;
pub mod profile;
pub mod signv4;
pub(crate) mod transport;

#[cfg(feature = "oks")]
pub mod oks;

#[cfg(feature = "osc")]
pub mod osc;

pub use errors::Error;
pub use profile::Profile;
