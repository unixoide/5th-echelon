// AUTO GENERATED FILE
use std::convert::TryInto;
use std::net::SocketAddr;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use slog::Logger;

use crate::prudp::packet::QPacket;
use crate::rmc::basic::Any;
use crate::rmc::basic::AnyClass;
use crate::rmc::basic::ClassRegistry;
use crate::rmc::basic::FromStream;
use crate::rmc::basic::Qresult;
use crate::rmc::basic::StationURL;
use crate::rmc::basic::ToStream;
use crate::rmc::Protocol;
use crate::rmc::Request;
use crate::rmc::Response;
use crate::rmc::ResponseData;
use crate::rmc::ResponseError;
use crate::ClientInfo;

/// Represents the methods available in the Ticket Granting Protocol.
#[repr(u32)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
enum TicketGrantingProtocolMethod {
    Login = 1,
    LoginEx = 2,
    RequestTicket = 3,
    GetPID = 4,
    GetName = 5,
    LoginWithContext = 6,
}

/// Implements the Ticket Granting Protocol.
#[derive(Debug)]
pub struct TicketGrantingProtocol;

impl Protocol for TicketGrantingProtocol {
    /// Returns the ID of the protocol.
    fn id(&self) -> u16 {
        10
    }

    /// Returns the name of the protocol.
    fn name(&self) -> String {
        "TicketGrantingProtocol".into()
    }

    /// Returns the number of methods in the protocol.
    fn num_methods(&self) -> u32 {
        6
    }

    /// Returns the name of a method given its ID.
    fn method_name(&self, method_id: u32) -> Option<String> {
        let m: Option<TicketGrantingProtocolMethod> = method_id.try_into().ok();
        match m {
            Some(m) => format!("{:?}", m),
            None => None,
        }
    }

    /// Handles an incoming request for the Ticket Granting Protocol.
    /// Handles an incoming request for the Ticket Granting Protocol.
    /// This method dispatches the request to the appropriate handler based on the `method_id`.
    fn handle(&self, logger: &Logger, ci: &mut ClientInfo, request: &Request) -> Response
    where
        Self: TicketGrantingProtocolTrait,
    {
        let m: Option<TicketGrantingProtocolMethod> = request.method_id.try_into().ok();
        match m {
            Some(TicketGrantingProtocolMethod::Login) => self.handle_login(request, LoginRequest::from_bytes(&request.parameters)),
            Some(TicketGrantingProtocolMethod::LoginEx) => {
                let data = LoginExRequest::from_bytes(&request.parameters);
                debug!(logger, "Request data: {:?}", data);
                match self.handle_login_ex(logger, ci, request, data) {
                    Err(r) => return r,
                    Ok(r) => Response {
                        protocol_id: self.id(),
                        result: Ok(ResponseData {
                            call_id: request.call_id,
                            method_id: TicketGrantingProtocolMethod::LoginEx.into(),
                            data: r.as_bytes(),
                        }),
                    },
                }
            }
            Some(TicketGrantingProtocolMethod::RequestTicket) => self.handle_request_ticket(request, RequestTicketRequest::from_bytes(&request.parameters)),
            Some(TicketGrantingProtocolMethod::GetPID) => self.handle_get_pi_d(request, GetPIDRequest::from_bytes(&request.parameters)),
            Some(TicketGrantingProtocolMethod::GetName) => self.handle_get_name(request, GetNameRequest::from_bytes(&request.parameters)),
            Some(TicketGrantingProtocolMethod::LoginWithContext) => self.handle_login_with_context(request, LoginWithContextRequest::from_bytes(&request.parameters)),
            None => Response {
                protocol_id: request.protocol_id,
                result: Err(ResponseError {
                    error_code: 0x80010001,
                    call_id: request.call_id,
                }),
            },
        }
    }
}

/// Trait for implementing the Ticket Granting Protocol.
pub trait TicketGrantingProtocolTrait {
    /// Returns an unknown error response.
    fn unknown_error(&self, request: &Request) -> Response {
        Response {
            protocol_id: request.protocol_id,
            result: Err(ResponseError {
                error_code: 0x80010001,
                call_id: request.call_id,
            }),
        }
    }

    /// Handles the Login method.
    fn handle_login(&self, request: &Request, _data: std::io::Result<LoginRequest>) -> Response {
        self.unknown_error(request)
    }
    /// Handles the LoginEx method.
    /// This method demonstrates handling of `Any` type data, which requires
    /// registering the expected class with `ClassRegistry` and then attempting
    /// to downcast the `Any` type to the concrete type.
    fn handle_login_ex(&self, logger: &Logger, _ci: &mut ClientInfo, request: &Request, data: std::io::Result<LoginExRequest>) -> std::result::Result<LoginExResponse, Response> {
        if data.is_err() {
            return Err(self.unknown_error(request));
        }
        let data = data.unwrap();

        let mut class_registry = ClassRegistry::default();
        class_registry.register_class::<UbiAuthenticationLoginCustomData>("UbiAuthenticationLoginCustomData".to_string());
        let extra_data = data.o_extra_data.into_inner(&class_registry);
        if extra_data.is_err() {
            return Err(self.unknown_error(request));
        }
        let extra_data = extra_data.unwrap();
        let extra_data = extra_data.as_any().downcast_ref::<UbiAuthenticationLoginCustomData>();
        if extra_data.is_none() {
            return Err(self.unknown_error(request));
        }
        let extra_data = extra_data.unwrap();
        info!(logger, "Login requested from {}", extra_data.user_name);

        Err(self.unknown_error(request))
    }
    /// Handles the RequestTicket method.
    fn handle_request_ticket(&self, request: &Request, _data: std::io::Result<RequestTicketRequest>) -> Response {
        self.unknown_error(request)
    }
    /// Handles the GetPID method.
    fn handle_get_pi_d(&self, request: &Request, _data: std::io::Result<GetPIDRequest>) -> Response {
        self.unknown_error(request)
    }
    /// Handles the GetName method.
    fn handle_get_name(&self, request: &Request, _data: std::io::Result<GetNameRequest>) -> Response {
        self.unknown_error(request)
    }
    /// Handles the LoginWithContext method.
    fn handle_login_with_context(&self, request: &Request, _data: std::io::Result<LoginWithContextRequest>) -> Response {
        self.unknown_error(request)
    }
}

/// Represents a Login request.
#[derive(Debug, FromStream)]
pub struct LoginRequest {}

/// Represents a LoginEx request.
#[derive(Debug, FromStream)]
pub struct LoginExRequest {
    str_user_name: String,
    o_extra_data: Any,
}

/// Represents a LoginEx response.
#[derive(Debug, ToStream)]
pub struct LoginExResponse {
    result: Qresult,
    pid_principal: u32,
    str_return_msg: String,
    p_connection_data: RVConnectionData,
}

/// Represents a RequestTicket request.
#[derive(Debug, FromStream)]
pub struct RequestTicketRequest {}

/// Represents a GetPID request.
#[derive(Debug, FromStream)]
pub struct GetPIDRequest {}

/// Represents a GetName request.
#[derive(Debug, FromStream)]
pub struct GetNameRequest {}

/// Represents a LoginWithContext request.
#[derive(Debug, FromStream)]
pub struct LoginWithContextRequest {}

/// Represents custom data for Ubi Authentication Login.
#[derive(Debug, FromStream)]
pub struct UbiAuthenticationLoginCustomData {
    user_name: String,
    online_key: String,
    password: String,
}

impl AnyClass for UbiAuthenticationLoginCustomData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents RV Connection Data.
#[derive(Debug, ToStream)]
pub struct RVConnectionData {
    url_regular_protocols: StationURL,
    lst_special_protocols: Vec<u8>,
    url_special_protocols: StationURL,
}
