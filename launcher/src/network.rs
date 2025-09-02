//! The central module for network-related functionality.
//!
//! This file aggregates functions from its submodules (`discover`, `quazal`, `rpc`)
//! and defines a common `Error` enum for all network operations.

mod discover;
mod quazal;
mod rpc;

// Re-export public functions from submodules.
pub use discover::try_locate_server;
pub use quazal::test_p2p;
pub use quazal::test_quazal_login;
pub use rpc::register;
pub use rpc::test_login;

/// Tests the connection to the configuration server.
///
/// This function sends a GET request to the `GetOnlineConfig` endpoint of the
/// specified server to verify that it is reachable and responding correctly.
pub async fn test_cfg_server(hostname: &str) -> Result<(), Error> {
    let url = format!("http://{hostname}/OnlineConfigService.svc/GetOnlineConfig");
    let resp = reqwest::Client::new().get(url).send().await.map_err(Error::ConfigServer)?;
    resp.error_for_status().map_err(Error::ConfigServer)?;
    Ok(())
}

/// A unified error type for all network operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Username not found")]
    UserNotFound,
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Error when sending request")]
    SendingRequestFailed,
    #[error("{0}")]
    ServerFailure(String),
    #[error("Username already taken or Ubisoft ID already registered")]
    UsernameAlreadyTaken,
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("RMC error: {0}")]
    Rmc(#[from] ::quazal::rmc::Error),
    #[error("Quazal error: {0}")]
    Quazal(#[from] ::quazal::Error),
    #[error("Quazal error: {0}")]
    Prudp(#[from] ::quazal::prudp::packet::Error),
    #[error("Connection attempt timed out")]
    TimedOut,
    #[error("RPC error: {0}")]
    Rpc(#[from] tonic::Status),
    #[error("Challenge mismatch")]
    ChallengeMismatch,
    #[error("P2P error: {0}")]
    P2P(#[from] Box<Error>),
    #[error("Config server: {0}")]
    ConfigServer(#[from] reqwest::Error),
}
