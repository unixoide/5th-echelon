pub use crate::protocols::authentication_foundation::ticket_granting_protocol::*;
pub use crate::protocols::authentication_foundation::types::*;
use quazal::kerberos::KerberosTicket;
use quazal::kerberos::KerberosTicketInternal;
use quazal::kerberos::SESSION_KEY_SIZE;
use quazal::rmc::types::*;
use quazal::rmc::Protocol;
use quazal::Context;

const SERVER_PID: u32 = 0x1000;

struct TicketGrantingProtocolImpl;

impl TicketGrantingProtocolImpl {
    fn get_session_key() -> [u8; SESSION_KEY_SIZE] {
        // TODO: use random key
        [1; SESSION_KEY_SIZE]
    }

    fn get_password_by_pid(&self, pid: u32) -> Option<&'static str> {
        if pid == 105 {
            Some("JaDe!")
        } else {
            None
        }
    }

    fn get_password_by_username(&self, username: &str) -> Option<&'static str> {
        if username == "Tracking" {
            Some("JaDe!")
        } else {
            None
        }
    }

    fn get_pid_by_username(&self, username: &str) -> u32 {
        match username {
            "Tracking" => 105,
            _ => 0x1234,
        }
    }
}

fn get_connection_data(ctx: &Context, pid: u32) -> RVConnectionData {
    ctx.secure_server_addr
        .map(|a| RVConnectionData {
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
            .into(),
            lst_special_protocols: vec![],
            url_special_protocols: StationURL::default(),
        })
        .unwrap_or_else(|| RVConnectionData {
            url_regular_protocols: format!(
                "prudp:/address={};port={};CID=1;PID={};sid=2;stream=3;type=2",
                ctx.listen.ip(),
                ctx.listen.port(),
                pid
            )
            .into(),
            lst_special_protocols: vec![],
            url_special_protocols: StationURL::default(),
        })
}

impl<T> TicketGrantingProtocolTrait<T> for TicketGrantingProtocolImpl {
    fn login(
        &self,
        _logger: &slog::Logger,
        ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LoginRequest,
    ) -> Result<LoginResponse, quazal::rmc::Error> {
        let password = self.get_password_by_username(&request.str_user_name);
        let user_id = self.get_pid_by_username(&request.str_user_name);
        ci.user_id = Some(user_id);
        let session_key = Self::get_session_key();
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
            pbuf_response: ticket.as_bytes(user_id, password),
            p_connection_data: get_connection_data(ctx, 2),
            str_return_msg: String::new(),
        })
    }

    fn login_ex(
        &self,
        _logger: &slog::Logger,
        ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: LoginExRequest,
    ) -> Result<LoginExResponse, quazal::rmc::Error> {
        let user_id = self.get_pid_by_username("TODO");
        ci.user_id = Some(user_id);
        let session_key = Self::get_session_key();
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
            pbuf_response: ticket.as_bytes(user_id, None),
            p_connection_data: get_connection_data(ctx, SERVER_PID),
            str_return_msg: String::new(),
        })
    }

    fn request_ticket(
        &self,
        _logger: &slog::Logger,
        _ctx: &Context,
        _ci: &mut quazal::ClientInfo<T>,
        request: RequestTicketRequest,
    ) -> Result<RequestTicketResponse, quazal::rmc::Error> {
        let user_id = request.id_source;
        let server_id = request.id_target;
        let session_key = Self::get_session_key();
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
            buf_response: ticket.as_bytes(user_id, self.get_password_by_pid(user_id)),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(TicketGrantingProtocol::new(TicketGrantingProtocolImpl))
}
