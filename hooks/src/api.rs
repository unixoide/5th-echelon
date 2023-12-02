use std::future::Future;
use std::sync::Mutex;
use std::sync::OnceLock;

use server_api::friends::friends_client::FriendsClient;
use server_api::friends::InviteRequest;
use server_api::friends::ListRequest;
use server_api::misc::misc_client::MiscClient;
use server_api::misc::EventRequest;
use server_api::misc::EventResponse;
use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use tonic::metadata::Ascii;
use tonic::metadata::MetadataValue;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::instrument;
use url::Url;

static TOKEN: Mutex<Option<MetadataValue<Ascii>>> = Mutex::new(None);
pub static URL: OnceLock<Url> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Missing URL for the api server")]
    MissingUrl,
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("gRPC error: {0}")]
    GRPCStatus(#[from] tonic::Status),
    #[error("Login failure")]
    LoginFailure,
    #[error("Login failure")]
    InvalidToken(#[from] tonic::metadata::errors::InvalidMetadataValue),
}

macro_rules! connect {
    ($client:ident) => {{
        let Some(ref url) = URL.get() else {
            return Err(Error::MissingUrl);
        };
        tracing::debug!("Connecting to {url}");
        let channel = tonic::transport::Channel::from_shared(url.as_str())
            .unwrap() // this should not fail (ideally) as url is a url::Url which gets converted to an Uri
            .connect_timeout(std::time::Duration::from_secs(1))
            .timeout(std::time::Duration::from_secs(10))
            .connect()
            .await?;
        tracing::debug!("Connected to {url}");
        $client::with_interceptor(channel, move |mut req: tonic::Request<_>| {
            let guard = TOKEN.lock().unwrap();
            if let Some(token) = (*guard).as_ref() {
                tracing::debug!("Adding auth token");
                req.metadata_mut().insert("authorization", token.clone());
            }
            Ok(req)
        })
    }};
}

pub struct Friend {
    pub id: String,
    pub username: String,
}

fn run<T>(func: impl Future<Output = Result<T, Error>>) -> Result<T, Error> {
    tokio::runtime::Runtime::new()?.block_on(func)
}

pub fn invite_friend(id: &str) -> Result<(), Error> {
    run(async {
        let mut client = connect!(FriendsClient);

        let request = tonic::Request::new(InviteRequest { id: id.into() });

        client.invite(request).await?;
        Ok(())
    })
}

pub fn list_friends() -> Result<Vec<Friend>, Error> {
    run(async {
        let mut client = connect!(FriendsClient);

        let request = tonic::Request::new(ListRequest {});

        let response = client.list(request).await?.into_inner();
        Ok(response
            .friends
            .into_iter()
            .map(|f| Friend {
                id: f.id,
                username: f.username,
            })
            .collect())
    })
}

#[instrument(skip(password))]
pub fn login(username: &str, password: &str) -> Result<(), Error> {
    run(async {
        let mut client = connect!(UsersClient);

        let request = tonic::Request::new(LoginRequest {
            username: String::from(username),
            password: String::from(password),
        });

        debug!("logging in");
        let response = client.login(request).await?.into_inner();
        if !response.error.is_empty() {
            error!("Login error: {}", response.error);
            return Err(Error::LoginFailure);
        } else if !response.token.is_empty() {
            info!("Login successful");
            let mut guard = TOKEN.lock().unwrap();
            *guard = Some(response.token.parse()?);
        }
        Ok(())
    })
}

#[instrument]
pub async fn event() -> Result<EventResponse, Error> {
    let mut client = connect!(MiscClient);

    let request = tonic::Request::new(EventRequest {});

    Ok(client.event(request).await?.into_inner())
}
