mod discover;
mod quazal;
mod rpc;

pub use discover::try_locate_server;
pub use quazal::test_p2p;
pub use quazal::test_quazal_login;
pub use rpc::register;
pub use rpc::test_login;

pub async fn test_cfg_server(hostname: &str) -> Result<(), Error> {
    let url = format!("http://{hostname}/OnlineConfigService.svc/GetOnlineConfig");
    let resp = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map_err(Error::ConfigServer)?;
    resp.error_for_status().map_err(Error::ConfigServer)?;
    Ok(())
}

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
