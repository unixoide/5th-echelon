use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use server_api::friends::friends_server::Friends;
use server_api::friends::friends_server::FriendsServer;
use server_api::friends::Friend;
use server_api::friends::InviteRequest;
use server_api::friends::InviteResponse;
use server_api::friends::ListRequest;
use server_api::friends::ListResponse;
use server_api::misc::misc_server::Misc;
use server_api::misc::misc_server::MiscServer;
use server_api::misc::EventRequest;
use server_api::misc::EventResponse;
use server_api::misc::InviteEvent;
use server_api::users::users_server::Users;
use server_api::users::users_server::UsersServer;
use server_api::users::LoginRequest;
use server_api::users::LoginResponse;
use server_api::users::User;
use slog::Logger;
use sodiumoxide::base64;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::Key;
use sodiumoxide::crypto::secretbox::Nonce;
use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use crate::storage::Storage;

pub struct MyFriends {
    logger: Logger,
    storage: Arc<Storage>,
}

#[tonic::async_trait]
impl Friends for MyFriends {
    async fn invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, Status> {
        let sender: u32 = request
            .metadata()
            .get("user_id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        debug!(self.logger, "Invite request: {:?} from {}", request, sender);

        let receiver = request.into_inner().id;

        let Some(receiver_id) = self
            .storage
            .find_user_id_by_ubi_id_async(&receiver)
            .await
            .map_err(|e| Status::internal(format!("Couldn't add invite: {e:?}")))?
        else {
            return Err(Status::not_found("User not found"));
        };

        self.storage
            .add_invite_async(sender, receiver_id)
            .await
            .map_err(|e| Status::internal(format!("Couldn't add invite: {e:?}")))?;

        let reply = InviteResponse {};

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        debug!(
            self.logger,
            "Friendlist request: {:?} from {}",
            request,
            request.metadata().get("user_id").unwrap().to_str().unwrap()
        );
        let users = self
            .storage
            .list_users_async()
            .await
            .map_err(|e| Status::internal(format!("{e}")))?;
        let friends = users
            .into_iter()
            .map(|u| Friend {
                id: u.ubi_id,
                username: u.username,
            })
            .collect();
        let resp = ListResponse { friends };
        Ok(Response::new(resp))
    }
}

async fn check_token<T>(
    logger: &Logger,
    key: &Key,
    storage: &Arc<Storage>,
    mut req: Request<T>,
) -> Result<Request<T>, Status> {
    let token = req
        .metadata()
        .get("authorization")
        .ok_or(Status::unauthenticated("Missing authorization"))?;

    let mut parts = token
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid token"))?
        .split('.');
    let c = parts
        .next()
        .ok_or(Status::unauthenticated("Invalid token"))?;
    let n = parts
        .next()
        .ok_or(Status::unauthenticated("Invalid token"))?;
    if parts.next().is_some() {
        return Err(Status::unauthenticated("Invalid token"));
    }

    let c = base64::decode(c, base64::Variant::UrlSafeNoPadding)
        .map_err(|()| Status::unauthenticated("Invalid token"))?;
    let n = base64::decode(n, base64::Variant::UrlSafeNoPadding)
        .map_err(|()| Status::unauthenticated("Invalid token"))?;

    let user_id = secretbox::open(
        &c,
        &Nonce::from_slice(&n).ok_or(Status::unauthenticated("Invalid token"))?,
        key,
    )
    .map_err(|()| Status::unauthenticated("Invalid token"))?;

    let user_id =
        std::str::from_utf8(&user_id).map_err(|_| Status::unauthenticated("Invalid user"))?;

    debug!(logger, "Looking for user {user_id}");

    let user = storage
        .find_username_by_user_id_async(
            user_id
                .parse()
                .map_err(|_| Status::unauthenticated("Invalid user"))?,
        )
        .await
        .map_err(|_| Status::unauthenticated("Invalid token"))?;

    let Some(user) = user else {
        return Err(Status::unauthenticated("Invalid user"));
    };

    debug!(logger, "Valid token for user {user_id}: {user}");

    req.metadata_mut().insert(
        "user_id",
        user_id
            .parse()
            .map_err(|_| Status::unauthenticated("Invalid user"))?,
    );

    Ok(req)
}

pub struct MyUsers {
    logger: Logger,
    key: Key,
    storage: Arc<Storage>,
}

#[tonic::async_trait]
impl Users for MyUsers {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;
        let password = request.password;

        let maybe_user = self
            .storage
            .login_user_async(&username, &password)
            .await
            .map_err(|e| Status::internal(format!("Login error: {e:?}")))?;
        let Some(user_id) = maybe_user else {
            return Err(Status::unauthenticated("Invalid login"));
        };

        let user_id = format!("{}", user_id);
        let n = secretbox::gen_nonce();
        let c = secretbox::seal(user_id.as_bytes(), &n, &self.key);

        let c = base64::encode(c, base64::Variant::UrlSafeNoPadding);
        let n = base64::encode(n, base64::Variant::UrlSafeNoPadding);

        info!(self.logger, "Login successful for {username}");
        Ok(Response::new(LoginResponse {
            error: String::new(),
            token: format!("{c}.{n}"),
            user: None,
        }))
    }
}

pub struct MyMisc {
    logger: Logger,
    storage: Arc<Storage>,
}

#[tonic::async_trait]
impl Misc for MyMisc {
    async fn event(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventResponse>, Status> {
        let user_id: u32 = request
            .metadata()
            .get("user_id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();

        let Some(invite) = self.storage.take_invite_async(user_id).await.map_err(|e| {
            error!(self.logger, "Error getting latest invite for user: {e}");
            Status::internal(format!("{e:?}"))
        })?
        else {
            return Ok(Response::new(EventResponse { invite: None }));
        };

        let Some(sender) = self
            .storage
            .find_user_by_id_async(invite.sender)
            .await
            .map_err(|e| {
                error!(self.logger, "Error getting ubi id for user: {e}");
                Status::internal(format!("{e:?}"))
            })?
        else {
            return Err(Status::not_found(""));
        };

        return Ok(Response::new(EventResponse {
            invite: Some(InviteEvent {
                id: invite.id,
                sender: Some(User {
                    id: sender.ubi_id,
                    username: sender.username,
                }),
            }),
        }));
    }
}

fn authenticated<S>(
    service: S,
    logger: Logger,
    key: Key,
    storage: Arc<Storage>,
) -> tonic_async_interceptor::AsyncInterceptedService<
    S,
    impl tonic_async_interceptor::AsyncInterceptor<
            Future = impl Future<Output = Result<Request<()>, Status>> + Send,
        > + Clone,
> {
    tonic_async_interceptor::AsyncInterceptedService::new(service, move |req: Request<()>| {
        let storage = Arc::clone(&storage);
        let logger = logger.clone();
        let key = key.clone();
        async move {
            let this = check_token(&logger, &key, &storage, req).await;
            if let Err(ref e) = this {
                error!(logger, "Auth failure: {e}");
            }
            this
        }
    })
}

pub async fn start_server(
    logger: Logger,
    storage: Arc<Storage>,
    server_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = secretbox::gen_key();
    info!(logger, "Listening on {server_addr}");
    Server::builder()
        .add_service(authenticated(
            FriendsServer::new(MyFriends {
                logger: logger.clone(),
                storage: Arc::clone(&storage),
            }),
            logger.clone(),
            key.clone(),
            Arc::clone(&storage),
        ))
        .add_service(authenticated(
            MiscServer::new(MyMisc {
                logger: logger.clone(),
                storage: Arc::clone(&storage),
            }),
            logger.clone(),
            key.clone(),
            Arc::clone(&storage),
        ))
        .add_service(UsersServer::new(MyUsers {
            logger,
            key,
            storage,
        }))
        .serve(server_addr)
        .await?;
    Ok(())
}
