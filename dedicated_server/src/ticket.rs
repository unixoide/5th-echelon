use std::sync::Arc;

use quazal::kerberos::KerberosTicket;
use quazal::kerberos::KerberosTicketInternal;
use quazal::kerberos::SESSION_KEY_SIZE;
use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QResult;
use quazal::rmc::types::StationURL;
use quazal::rmc::Protocol;
use quazal::Context;
use rand::TryRngCore;

pub use crate::protocols::authentication_foundation::ticket_granting_protocol::*;
pub use crate::protocols::authentication_foundation::types::*;
use crate::protocols::ubi_authentication::types::UbiAuthenticationLoginCustomData;
use crate::storage::Storage;
use crate::SERVER_PID;

struct TicketGrantingProtocolServerImpl {
    storage: Arc<Storage>,
}

impl TicketGrantingProtocolServerImpl {
    fn get_session_key(&self, logger: &slog::Logger, user_id: u32) -> [u8; SESSION_KEY_SIZE] {
        let mut key = [0u8; SESSION_KEY_SIZE];
        rand::rngs::OsRng
            .try_fill_bytes(&mut key)
            .expect("Generating session key");

        if let Err(e) = self.storage.create_user_session(user_id, &key) {
            eprintln!("Error saving user session: {e}");
            error!(logger, "Error saving user session: {e}");
        }

        key
    }

    #[allow(unreachable_code)]
    fn get_password_by_pid(&self, logger: &slog::Logger, pid: u32) -> quazal::rmc::Result<Option<String>> {
        self.storage.find_password_for_user(pid).map_err(|e| {
            eprintln!("Error finding user password: {e}");
            error!(logger, "Error finding user password: {e}");
            quazal::rmc::Error::InternalError
        })
    }

    fn get_password_by_username(&self, logger: &slog::Logger, username: &str) -> quazal::rmc::Result<Option<String>> {
        Ok(self
            .get_pid_by_username(logger, username)?
            .map(|uid| self.get_password_by_pid(logger, uid))
            .transpose()?
            .flatten())
    }

    #[allow(unreachable_code)]
    fn get_pid_by_username(&self, logger: &slog::Logger, username: &str) -> quazal::rmc::Result<Option<u32>> {
        self.storage.find_user_id_by_name(username).map_err(|e| {
            eprintln!("Error finding user password: {e}");
            error!(logger, "Error finding user password: {e}");
            quazal::rmc::Error::InternalError
        })
    }

    #[allow(unreachable_code)]
    fn login(&self, logger: &slog::Logger, username: &str, password: &str) -> quazal::rmc::Result<Option<u32>> {
        self.storage
            .login_user(username, password)
            .map_err(|e| {
                eprintln!("Error finding user password: {e}");
                error!(logger, "Error finding user password: {e}");
                quazal::rmc::Error::InternalError
            })
            .map(Result::ok)
    }
}

fn get_connection_data(ctx: &Context, pid: u32) -> RVConnectionData {
    ctx.secure_server_addr.map_or_else(
        || RVConnectionData {
            url_regular_protocols: format!(
                "prudp:/address={};port={};CID=1;PID={};sid=2;stream=3;type=2",
                ctx.listen.ip(),
                ctx.listen.port(),
                pid
            )
            .parse()
            .unwrap(),
            lst_special_protocols: vec![],
            url_special_protocols: StationURL::default(),
        },
        |a| RVConnectionData {
            url_regular_protocols: format!(
                // CID => ConnectionID
                // PID => PrincipalID
                // RVCID => RVConnectionID
                // sid => StreamID
                // stream => StreamType
                "prudps:/address={};port={};CID=1;PID={};sid=1;stream=3;type=2",
                a.ip(),
                a.port(),
                pid
            )
            .parse()
            .unwrap(),
            lst_special_protocols: vec![],
            url_special_protocols: StationURL::default(),
        },
    )
}

impl<T> TicketGrantingProtocolServerTrait<T> for TicketGrantingProtocolServerImpl {
    fn login(
        &self,
        logger: &slog::Logger,
        ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LoginRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LoginResponse, quazal::rmc::Error> {
        let Some(user_id) = self.get_pid_by_username(logger, &request.str_user_name)? else {
            warn!(logger, "user {} not found", request.str_user_name);
            return Err(quazal::rmc::Error::AccessDenied);
        };
        let password = self
            .get_password_by_username(logger, &request.str_user_name)?
            .or_else(|| {
                warn!(logger, "user {} has no plaintext password", request.str_user_name);
                None
            });
        ci.user_id = Some(user_id);
        let session_key = self.get_session_key(logger, user_id);
        let ticket = KerberosTicket {
            session_key,
            pid: SERVER_PID,
            internal: KerberosTicketInternal {
                principle_id: user_id,
                valid_until: u64::MAX,
                session_key,
            },
        };
        Ok(LoginResponse {
            return_value: QResult::Ok,
            pid_principal: user_id,
            pbuf_response: ticket.as_bytes(user_id, password.as_deref(), &ctx.ticket_key),
            p_connection_data: get_connection_data(ctx, 2),
            str_return_msg: String::new(),
        })
    }

    fn login_ex(
        &self,
        logger: &slog::Logger,
        ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LoginExRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LoginExResponse, quazal::rmc::Error> {
        let username = request.str_user_name;
        let mut registry = quazal::rmc::types::ClassRegistry::default();
        registry.register_class::<UbiAuthenticationLoginCustomData>("UbiAuthenticationLoginCustomData");
        let ubi_data = request.o_extra_data.into_inner(&registry)?;
        let ubi_data: Option<&UbiAuthenticationLoginCustomData> = ubi_data.as_any().downcast_ref();
        let Some(UbiAuthenticationLoginCustomData {
            user_name: ubi_username,
            password,
            ..
        }) = ubi_data
        else {
            error!(logger, "Error parsing UbiAuthenticationLoginCustomData");
            return Err(quazal::rmc::Error::ParsingError);
        };

        info!(logger, "LoginEx attempt by {} ({})", ubi_username, username);

        let Some(user_id) = self.login(logger, ubi_username, password)? else {
            warn!(logger, "login failed for {}", ubi_username);
            return Err(quazal::rmc::Error::AccessDenied);
        };
        info!(logger, "login successful for {}", ubi_username);

        ci.user_id = Some(user_id);
        let session_key = self.get_session_key(logger, user_id);
        let ticket = KerberosTicket {
            session_key,
            pid: SERVER_PID,
            internal: KerberosTicketInternal {
                principle_id: user_id,
                valid_until: u64::MAX,
                session_key,
            },
        };
        Ok(LoginExResponse {
            return_value: QResult::Ok,
            pid_principal: user_id,
            pbuf_response: ticket.as_bytes(user_id, None, &ctx.ticket_key),
            p_connection_data: get_connection_data(ctx, SERVER_PID),
            str_return_msg: String::new(),
        })
    }

    fn request_ticket(
        &self,
        logger: &slog::Logger,
        ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: RequestTicketRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RequestTicketResponse, quazal::rmc::Error> {
        let user_id = request.id_source;
        let server_id = request.id_target;
        if !matches!(ci.user_id, Some(uid) if uid == user_id) {
            warn!(
                logger,
                "Ticket request for {} to {} denied (user: {:?})", user_id, server_id, ci.user_id
            );
            return Err(quazal::rmc::Error::AccessDenied);
        }
        let session_key = self.get_session_key(logger, user_id);
        let ticket = KerberosTicket {
            session_key,
            pid: server_id,
            internal: KerberosTicketInternal {
                principle_id: user_id,
                valid_until: u64::MAX,
                session_key,
            },
        };
        Ok(RequestTicketResponse {
            return_value: QResult::Ok,
            buf_response: ticket.as_bytes(
                user_id,
                self.get_password_by_pid(logger, user_id)?.as_deref(),
                &ctx.ticket_key,
            ),
        })
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(TicketGrantingProtocolServer::new(TicketGrantingProtocolServerImpl {
        storage,
    }))
}
