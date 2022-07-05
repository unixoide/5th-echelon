// AUTO GENERATED FILE
use std::convert::TryInto;
use std::net::SocketAddr;

use crate::prudp::packet::QPacket;
use crate::rmc::basic::{Any, AnyClass, ClassRegistry, FromStream, Qresult, StationURL, ToStream};
use crate::rmc::{Protocol, ResponseData};
use crate::rmc::{Request, Response, ResponseError};
use crate::ClientInfo;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use slog::Logger;

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

#[derive(Debug)]
pub struct TicketGrantingProtocol;

impl Protocol for TicketGrantingProtocol {
    fn id(&self) -> u16 {
        10
    }

    fn name(&self) -> String {
        "TicketGrantingProtocol".into()
    }

    fn num_methods(&self) -> u32 {
        6
    }

    fn method_name(&self, method_id: u32) -> Option<String> {
        let m: Option<TicketGrantingProtocolMethod> = method_id.try_into().ok();
        match m {
            Some(m) => format!("{:?}", m),
            None => None,
        }
    }

    fn handle(&self, logger: &Logger, ci: &mut ClientInfo, request: &Request) -> Response
    where
        Self: TicketGrantingProtocolTrait,
    {
        let m: Option<TicketGrantingProtocolMethod> = request.method_id.try_into().ok();
        match m {
            Some(TicketGrantingProtocolMethod::Login) => {
                self.handle_login(request, LoginRequest::from_bytes(&request.parameters))
            }
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
            Some(TicketGrantingProtocolMethod::RequestTicket) => self.handle_request_ticket(
                request,
                RequestTicketRequest::from_bytes(&request.parameters),
            ),
            Some(TicketGrantingProtocolMethod::GetPID) => {
                self.handle_get_pi_d(request, GetPIDRequest::from_bytes(&request.parameters))
            }
            Some(TicketGrantingProtocolMethod::GetName) => {
                self.handle_get_name(request, GetNameRequest::from_bytes(&request.parameters))
            }
            Some(TicketGrantingProtocolMethod::LoginWithContext) => self.handle_login_with_context(
                request,
                LoginWithContextRequest::from_bytes(&request.parameters),
            ),
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

pub trait TicketGrantingProtocolTrait {
    fn unknown_error(&self, request: &Request) -> Response {
        Response {
            protocol_id: request.protocol_id,
            result: Err(ResponseError {
                error_code: 0x80010001,
                call_id: request.call_id,
            }),
        }
    }

    fn handle_login(&self, request: &Request, _data: std::io::Result<LoginRequest>) -> Response {
        self.unknown_error(request)
    }
    fn handle_login_ex(
        &self,
        logger: &Logger,
        _ci: &mut ClientInfo,
        request: &Request,
        data: std::io::Result<LoginExRequest>,
    ) -> std::result::Result<LoginExResponse, Response> {
        if data.is_err() {
            return Err(self.unknown_error(request));
        }
        let data = data.unwrap();

        let mut class_registry = ClassRegistry::default();
        class_registry.register_class::<UbiAuthenticationLoginCustomData>(
            "UbiAuthenticationLoginCustomData".to_string(),
        );
        let extra_data = data.o_extra_data.into_inner(&class_registry);
        if extra_data.is_err() {
            return Err(self.unknown_error(request));
        }
        let extra_data = extra_data.unwrap();
        let extra_data = extra_data
            .as_any()
            .downcast_ref::<UbiAuthenticationLoginCustomData>();
        if extra_data.is_none() {
            return Err(self.unknown_error(request));
        }
        let extra_data = extra_data.unwrap();
        info!(logger, "Login requested from {}", extra_data.user_name);

        Err(self.unknown_error(request))
    }
    fn handle_request_ticket(
        &self,
        request: &Request,
        _data: std::io::Result<RequestTicketRequest>,
    ) -> Response {
        self.unknown_error(request)
    }
    fn handle_get_pi_d(
        &self,
        request: &Request,
        _data: std::io::Result<GetPIDRequest>,
    ) -> Response {
        self.unknown_error(request)
    }
    fn handle_get_name(
        &self,
        request: &Request,
        _data: std::io::Result<GetNameRequest>,
    ) -> Response {
        self.unknown_error(request)
    }
    fn handle_login_with_context(
        &self,
        request: &Request,
        _data: std::io::Result<LoginWithContextRequest>,
    ) -> Response {
        self.unknown_error(request)
    }
}

#[derive(Debug, FromStream)]
pub struct LoginRequest {}

#[derive(Debug, FromStream)]
pub struct LoginExRequest {
    str_user_name: String,
    o_extra_data: Any,
}

#[derive(Debug, ToStream)]
pub struct LoginExResponse {
    result: Qresult,
    pid_principal: u32,
    str_return_msg: String,
    p_connection_data: RVConnectionData,
}

#[derive(Debug, FromStream)]
pub struct RequestTicketRequest {}

#[derive(Debug, FromStream)]
pub struct GetPIDRequest {}

#[derive(Debug, FromStream)]
pub struct GetNameRequest {}

#[derive(Debug, FromStream)]
pub struct LoginWithContextRequest {}

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

#[derive(Debug, ToStream)]
pub struct RVConnectionData {
    url_regular_protocols: StationURL,
    lst_special_protocols: Vec<u8>,
    url_special_protocols: StationURL,
}
