use super::types::*;
use num_enum::TryFromPrimitive;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;
use std::convert::TryFrom;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum TicketGrantingProtocolMethod {
    Login = 1u32,
    LoginEx = 2u32,
    RequestTicket = 3u32,
    GetPid = 4u32,
    GetName = 5u32,
    LoginWithContext = 6u32,
}
#[derive(Debug, FromStream)]
pub struct LoginRequest {
    pub str_user_name: String,
}
#[derive(Debug, ToStream)]
pub struct LoginResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub pbuf_response: Vec<u8>,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginExRequest {
    pub str_user_name: String,
    pub o_extra_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LoginExResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub pbuf_response: Vec<u8>,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct RequestTicketRequest {
    pub id_source: u32,
    pub id_target: u32,
}
#[derive(Debug, ToStream)]
pub struct RequestTicketResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub buf_response: Vec<u8>,
}
#[derive(Debug, FromStream)]
pub struct GetPidRequest {
    pub str_user_name: String,
}
#[derive(Debug, ToStream)]
pub struct GetPidResponse {
    pub return_value: u32,
}
#[derive(Debug, FromStream)]
pub struct GetNameRequest {
    pub id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetNameResponse {
    pub return_value: String,
}
#[derive(Debug, FromStream)]
pub struct LoginWithContextRequest {
    pub login_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LoginWithContextResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub pbuf_response: Vec<u8>,
    pub p_connection_data: RVConnectionData,
}
pub struct TicketGrantingProtocol<T: TicketGrantingProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: TicketGrantingProtocolTrait<CI>, CI> TicketGrantingProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: TicketGrantingProtocolTrait<CI>, CI> Protocol<CI> for TicketGrantingProtocol<T, CI> {
    fn id(&self) -> u16 {
        10u16
    }
    fn name(&self) -> String {
        "TicketGrantingProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        6u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = TicketGrantingProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(TicketGrantingProtocolMethod::Login) => {
                let req = LoginRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TicketGrantingProtocolMethod::LoginEx) => {
                let req = LoginExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TicketGrantingProtocolMethod::RequestTicket) => {
                let req = RequestTicketRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_ticket(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TicketGrantingProtocolMethod::GetPid) => {
                let req = GetPidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_pid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TicketGrantingProtocolMethod::GetName) => {
                let req = GetNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_name(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TicketGrantingProtocolMethod::LoginWithContext) => {
                let req = LoginWithContextRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_context(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        TicketGrantingProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait TicketGrantingProtocolTrait<CI> {
    fn login(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginRequest,
    ) -> Result<LoginResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(login)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginExRequest,
    ) -> Result<LoginExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(login_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn request_ticket(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestTicketRequest,
    ) -> Result<RequestTicketResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(request_ticket)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_pid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPidRequest,
    ) -> Result<GetPidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(get_pid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_name(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNameRequest,
    ) -> Result<GetNameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(get_name)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_context(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithContextRequest,
    ) -> Result<LoginWithContextResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TicketGrantingProtocol",
            stringify!(login_with_context)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
