use std::collections::HashMap;
use std::collections::HashSet;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use quazal::rmc::types::Property;
use quazal::rmc::types::QList;
use quazal::rmc::types::StationURL;
use server_api::friends;
use server_api::friends::friends_server::Friends;
use server_api::friends::friends_server::FriendsServer;
use server_api::friends::Friend;
use server_api::games;
use server_api::games::games_admin_server::GamesAdmin;
use server_api::games::games_admin_server::GamesAdminServer;
use server_api::misc;
use server_api::misc::misc_server::Misc;
use server_api::misc::misc_server::MiscServer;
use server_api::users;
use server_api::users::users_admin_server::UsersAdmin;
use server_api::users::users_admin_server::UsersAdminServer;
use server_api::users::users_server::Users;
use server_api::users::users_server::UsersServer;
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

use crate::config::DebugConfig;
use crate::storage::LoginError;
use crate::storage::Storage;

pub struct MyFriends {
    logger: Logger,
    storage: Arc<Storage>,
    debug_config: Arc<DebugConfig>,
}

#[tonic::async_trait]
impl Friends for MyFriends {
    async fn invite(
        &self,
        request: Request<friends::InviteRequest>,
    ) -> Result<Response<friends::InviteResponse>, Status> {
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

        let reply = friends::InviteResponse {};

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn list(&self, request: Request<friends::ListRequest>) -> Result<Response<friends::ListResponse>, Status> {
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
                is_online: u.is_online || self.debug_config.mark_all_as_online,
            })
            .collect();
        let resp = friends::ListResponse { friends };
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
    let c = parts.next().ok_or(Status::unauthenticated("Invalid token"))?;
    let n = parts.next().ok_or(Status::unauthenticated("Invalid token"))?;
    if parts.next().is_some() {
        return Err(Status::unauthenticated("Invalid token"));
    }

    let c =
        base64::decode(c, base64::Variant::UrlSafeNoPadding).map_err(|()| Status::unauthenticated("Invalid token"))?;
    let n =
        base64::decode(n, base64::Variant::UrlSafeNoPadding).map_err(|()| Status::unauthenticated("Invalid token"))?;

    let user_id = secretbox::open(
        &c,
        &Nonce::from_slice(&n).ok_or(Status::unauthenticated("Invalid token"))?,
        key,
    )
    .map_err(|()| Status::unauthenticated("Invalid token"))?;

    let user_id = std::str::from_utf8(&user_id).map_err(|_| Status::unauthenticated("Invalid user"))?;

    debug!(logger, "Looking for user {user_id}");

    let user = storage
        .find_username_by_user_id_async(user_id.parse().map_err(|_| Status::unauthenticated("Invalid user"))?)
        .await
        .map_err(|_| Status::unauthenticated("Invalid token"))?;

    let Some(user) = user else {
        return Err(Status::unauthenticated("Invalid user"));
    };

    debug!(logger, "Valid token for user {user_id}: {user}");

    req.metadata_mut().insert(
        "user_id",
        user_id.parse().map_err(|_| Status::unauthenticated("Invalid user"))?,
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
    async fn login(&self, request: Request<users::LoginRequest>) -> Result<Response<users::LoginResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;
        let password = request.password;

        let maybe_user = self
            .storage
            .login_user_async(&username, &password)
            .await
            .map_err(|e| Status::internal(format!("Login error: {e:?}")))?;

        let user_id = maybe_user.map_err(|err| match err {
            LoginError::InvalidPassword => Status::unauthenticated("Invalid login"),
            LoginError::NotFound => Status::not_found("Unknown user"),
        })?;

        let user_id = format!("{user_id}");
        let n = secretbox::gen_nonce();
        let c = secretbox::seal(user_id.as_bytes(), &n, &self.key);

        let c = base64::encode(c, base64::Variant::UrlSafeNoPadding);
        let n = base64::encode(n, base64::Variant::UrlSafeNoPadding);

        info!(self.logger, "Login successful for {username}");
        Ok(Response::new(users::LoginResponse {
            error: String::new(),
            token: format!("{c}.{n}"),
            user: None,
        }))
    }

    async fn register(
        &self,
        request: Request<users::RegisterRequest>,
    ) -> Result<Response<users::RegisterResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;
        let password = request.password;
        let ubi_id = request.ubi_id;

        let error = if let Err(err) = self
            .storage
            .register_user_async(&username, &password, Some(&ubi_id))
            .await
        {
            match err.downcast::<sqlx::Error>() {
                Ok(sqlx::Error::Database(db_err)) => {
                    if db_err.is_unique_violation() {
                        return Err(Status::already_exists(String::from(
                            "Username already taken or Ubisoft ID already registered",
                        )));
                    }
                    return Err(Status::internal(db_err.to_string()));
                }
                Ok(err) => return Err(Status::internal(err.to_string())),
                Err(err) => err.to_string(),
            }
        } else {
            String::new()
        };
        info!(self.logger, "New user {username} ({ubi_id}) registered");
        Ok(Response::new(users::RegisterResponse { error, user: None }))
    }
}

pub struct MyMisc {
    logger: Logger,
    storage: Arc<Storage>,
    debug_config: Arc<DebugConfig>,
}

#[tonic::async_trait]
impl Misc for MyMisc {
    async fn event(&self, request: Request<misc::EventRequest>) -> Result<Response<misc::EventResponse>, Status> {
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
            return Ok(Response::new(misc::EventResponse { invite: None }));
        };

        let Some(sender) = self.storage.find_user_by_id_async(invite.sender).await.map_err(|e| {
            error!(self.logger, "Error getting ubi id for user: {e}");
            Status::internal(format!("{e:?}"))
        })?
        else {
            return Err(Status::not_found(""));
        };

        Ok(Response::new(misc::EventResponse {
            invite: Some(misc::InviteEvent {
                id: invite.id,
                sender: Some(User {
                    id: sender.ubi_id,
                    username: sender.username,
                    ips: vec![],
                }),
                force_join: self.debug_config.force_joins,
            }),
        }))
    }

    async fn test_p2p(
        &self,
        request: Request<misc::TestP2pRequest>,
    ) -> Result<Response<misc::TestP2pResponse>, Status> {
        let mut client_addr = request
            .remote_addr()
            .ok_or(Status::failed_precondition("no client address"))?;
        client_addr.set_port(13_000);
        let request = request.into_inner();
        let mut resp_data = b"P2P Test - ".to_vec();
        resp_data.extend(request.challenge);

        let buf = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
            socket.connect(client_addr).await?;
            socket.send(&resp_data).await?;
            let mut buf = [0; 1024];
            let len = socket.recv(&mut buf).await?;
            Ok(buf[..len].to_vec())
        });
        let Ok(challenge) = buf.await else {
            return Err(Status::deadline_exceeded("client didn't response in time"));
        };
        let Ok(challenge): std::io::Result<Vec<u8>> = challenge else {
            return Err(Status::unknown(format!(
                "P2P communication failed: {}",
                challenge.unwrap_err()
            )));
        };

        Ok(Response::new(misc::TestP2pResponse { challenge }))
    }
}

pub struct MyUsersAdmin {
    logger: Logger,
    storage: Arc<Storage>,
}

#[tonic::async_trait]
impl UsersAdmin for MyUsersAdmin {
    async fn list(&self, request: Request<users::ListRequest>) -> Result<Response<users::ListResponse>, Status> {
        let _request = request.into_inner();
        let Ok(db_users) = self.storage.list_users_async().await else {
            return Err(Status::internal("Error listing users"));
        };

        let mut users = vec![];
        for user in db_users {
            let urls = self
                .storage
                .list_urls(user.id)
                .await
                .map_err(|e| Status::internal(format!("{e:?}")))?;
            let ips = urls
                .into_iter()
                .map(|u| u.parse::<StationURL>())
                .filter_map(Result::ok)
                .map(|u| u.address)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            users.push(User {
                id: user.ubi_id,
                username: user.username,
                ips,
            });
        }

        let resp = users::ListResponse {
            #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
            total: users.len() as i32,
            users,
        };
        Ok(Response::new(resp))
    }

    async fn get(&self, request: Request<users::GetRequest>) -> Result<Response<users::GetResponse>, Status> {
        let request = request.into_inner();
        let user_id = request.id;
        let user_id: u32 = user_id.parse().map_err(|_| Status::invalid_argument("Invalid ID"))?;
        let Ok(user) = self.storage.find_user_by_id_async(user_id).await else {
            return Err(Status::internal("Error retrieving user"));
        };

        let Some(user) = user else {
            return Err(Status::not_found("User not found"));
        };

        let urls = self
            .storage
            .list_urls(user.id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;
        let ips = urls
            .into_iter()
            .map(|u| u.parse::<StationURL>())
            .filter_map(Result::ok)
            .map(|u| u.address)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let resp = users::GetResponse {
            user: Some(User {
                id: user.ubi_id,
                username: user.username,
                ips,
            }),
        };
        Ok(Response::new(resp))
    }

    async fn delete(&self, request: Request<users::DeleteRequest>) -> Result<Response<users::DeleteResponse>, Status> {
        let request = request.into_inner();
        let Some(user) = self
            .storage
            .find_user_by_ubi_id_async(&request.id)
            .await
            .map_err(|_| Status::invalid_argument("Invalid ID"))?
        else {
            return Err(Status::not_found("User not found"));
        };
        match self.storage.delete_user_async(user.id).await {
            Ok(()) => {
                warn!(self.logger, "Deleted user {user:?}");
                Ok(Response::new(users::DeleteResponse {}))
            }
            Err(e) => Err(Status::internal(format!("{e:?}"))),
        }
    }
}

struct MyGamesAdmin {
    logger: Logger,
    storage: Arc<Storage>,
}

#[tonic::async_trait]
impl GamesAdmin for MyGamesAdmin {
    async fn list(&self, request: Request<games::ListRequest>) -> Result<Response<games::ListResponse>, Status> {
        let _request = request.into_inner();
        let sessions = self.storage.list_game_sessions_async().await.map_err(|e| {
            error!(self.logger, "Error listing games: {e}");
            Status::internal(format!("{e:?}"))
        })?;

        Ok(Response::new(games::ListResponse {
            games: sessions
                .into_iter()
                .map(|s| {
                    let attributes: QList<Property> = s
                        .attributes
                        .parse()
                        .inspect_err(|e| {
                            error!(self.logger, "Error parsing game type: {e}");
                        })
                        .unwrap_or_default();
                    let attributes: HashMap<u32, u32> = attributes.0.into_iter().map(|p| (p.id, p.value)).collect();

                    // just guessing that 105 gives me what I want... ¯\_(ツ)_/¯
                    let game_type = match attributes.get(&105) {
                        None => String::from("Lobby"),
                        Some(&1) => String::from("SvM"),
                        Some(&2) => String::from("Coop"),
                        Some(v) => format!("Unknown({v})"),
                    };

                    games::Game {
                        id: s.session_id,
                        creator: s
                            .participants
                            .iter()
                            .find(|p| p.user_id == s.creator_id)
                            .unwrap()
                            .name
                            .clone(),
                        participants: s
                            .participants
                            .into_iter()
                            .filter(|p| p.user_id != s.creator_id)
                            .map(|p| p.name)
                            .collect(),
                        game_type,
                    }
                })
                .collect(),
        }))
    }

    async fn delete(&self, request: Request<games::DeleteRequest>) -> Result<Response<games::DeleteResponse>, Status> {
        let request = request.into_inner();
        let session_id = request.id;
        match self.storage.delete_game_session_by_id_async(session_id).await {
            Ok(()) => {
                warn!(self.logger, "Deleted game session {session_id:?}");
                Ok(Response::new(games::DeleteResponse {}))
            }
            Err(e) => Err(Status::internal(format!("{e:?}"))),
        }
    }
}

fn authenticated<S>(
    service: S,
    logger: Logger,
    key: Key,
    storage: Arc<Storage>,
) -> tonic_async_interceptor::AsyncInterceptedService<
    S,
    impl tonic_async_interceptor::AsyncInterceptor<Future = impl Future<Output = Result<Request<()>, Status>> + Send>
        + Clone,
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

fn preshared_authentication<S>(
    service: S,
    key: String,
) -> tonic::service::interceptor::InterceptedService<S, impl tonic::service::Interceptor + Clone> {
    tonic::service::interceptor::InterceptedService::new(service, move |req: Request<()>| {
        let header_value = req
            .metadata()
            .get("authorization")
            .ok_or(Status::unauthenticated("Missing authorization"))?;
        let token = header_value
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        if token == key {
            Ok(req)
        } else {
            Err(Status::permission_denied("Invalid token"))
        }
    })
}

fn base32(data: &[u8]) -> String {
    let mut s = String::new();
    for chunk in data.chunks(5) {
        let mut value = 0u64;
        let mut i = 0;
        for c in chunk {
            value <<= 8;
            value |= u64::from(*c);
            i += 1;
        }
        value <<= 8 * (5 - i);
        if i == 8 {
            i = 0;
        } else {
            i = 8 - (i * 8 + 4) / 5;
        }
        for i in (i..8).rev() {
            let ch = match ((value >> (5 * i)) & 0b11111) as u8 {
                b @ 0..=9 => b'0' + b,
                b @ 10..=31 => b'A' + b - 10,
                b => unreachable!("{b:?}"),
            };
            s.push(ch as char);
        }
    }
    s
}

pub async fn start_server(
    logger: Logger,
    storage: Arc<Storage>,
    server_addr: SocketAddr,
    debug_config: Arc<DebugConfig>,
    enable_admin_services: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = secretbox::gen_key();
    info!(logger, "Listening on {server_addr}");
    let builder = Server::builder()
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(users::FILE_DESCRIPTOR_SET)
                .register_encoded_file_descriptor_set(friends::FILE_DESCRIPTOR_SET)
                .register_encoded_file_descriptor_set(misc::FILE_DESCRIPTOR_SET)
                .build_v1alpha()
                .unwrap(),
        )
        .add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(users::FILE_DESCRIPTOR_SET)
                .register_encoded_file_descriptor_set(friends::FILE_DESCRIPTOR_SET)
                .register_encoded_file_descriptor_set(misc::FILE_DESCRIPTOR_SET)
                .build_v1()
                .unwrap(),
        )
        .add_service(authenticated(
            FriendsServer::new(MyFriends {
                logger: logger.clone(),
                storage: Arc::clone(&storage),
                debug_config: Arc::clone(&debug_config),
            }),
            logger.clone(),
            key.clone(),
            Arc::clone(&storage),
        ))
        .add_service(authenticated(
            MiscServer::new(MyMisc {
                logger: logger.clone(),
                storage: Arc::clone(&storage),
                debug_config,
            }),
            logger.clone(),
            key.clone(),
            Arc::clone(&storage),
        ))
        .add_service(UsersServer::new(MyUsers {
            logger: logger.clone(),
            storage: Arc::clone(&storage),
            key,
        }));

    let builder = if enable_admin_services {
        warn!(logger, "Enabling admin services");
        let preshared = base32(&secretbox::gen_key().0);
        println!("Admin Key: {preshared}");
        builder
            .add_service(preshared_authentication(
                UsersAdminServer::new(MyUsersAdmin {
                    logger: logger.clone(),
                    storage: Arc::clone(&storage),
                }),
                preshared.clone(),
            ))
            .add_service(preshared_authentication(
                GamesAdminServer::new(MyGamesAdmin { logger, storage }),
                preshared,
            ))
    } else {
        builder
    };

    builder.serve(server_addr).await?;
    Ok(())
}
