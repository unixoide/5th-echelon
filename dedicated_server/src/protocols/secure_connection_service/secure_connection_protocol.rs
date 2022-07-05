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
enum SecureConnectionProtocolMethod {
    Register = 1u32,
    RequestConnectionData = 2u32,
    RequestUrLs = 3u32,
    RegisterEx = 4u32,
}
#[derive(Debug, FromStream)]
pub struct RegisterRequest {
    pub vec_my_ur_ls: Vec<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream)]
pub struct RegisterResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_connection_id: u32,
    pub url_public: quazal::rmc::types::StationURL,
}
#[derive(Debug, FromStream)]
pub struct RequestConnectionDataRequest {
    pub cid_target: u32,
    pub pid_target: u32,
}
#[derive(Debug, ToStream)]
pub struct RequestConnectionDataResponse {
    pub return_value: bool,
    pub pvec_connections_data: Vec<ConnectionData>,
}
#[derive(Debug, FromStream)]
pub struct RequestUrLsRequest {
    pub cid_target: u32,
    pub pid_target: u32,
}
#[derive(Debug, ToStream)]
pub struct RequestUrLsResponse {
    pub return_value: bool,
    pub plst_ur_ls: Vec<quazal::rmc::types::StationURL>,
}
#[derive(Debug, FromStream)]
pub struct RegisterExRequest {
    pub vec_my_ur_ls: Vec<quazal::rmc::types::StationURL>,
    pub h_custom_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct RegisterExResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_connection_id: u32,
    pub url_public: quazal::rmc::types::StationURL,
}
pub struct SecureConnectionProtocol<T: SecureConnectionProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: SecureConnectionProtocolTrait<CI>, CI> SecureConnectionProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: SecureConnectionProtocolTrait<CI>, CI> Protocol<CI> for SecureConnectionProtocol<T, CI> {
    fn id(&self) -> u16 {
        11u16
    }
    fn name(&self) -> String {
        "SecureConnectionProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        4u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = SecureConnectionProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(SecureConnectionProtocolMethod::Register) => {
                let req = RegisterRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SecureConnectionProtocolMethod::RequestConnectionData) => {
                let req = RequestConnectionDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_connection_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SecureConnectionProtocolMethod::RequestUrLs) => {
                let req = RequestUrLsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_ur_ls(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SecureConnectionProtocolMethod::RegisterEx) => {
                let req = RegisterExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        SecureConnectionProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait SecureConnectionProtocolTrait<CI> {
    fn register(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SecureConnectionProtocol",
            stringify!(register)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn request_connection_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestConnectionDataRequest,
    ) -> Result<RequestConnectionDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SecureConnectionProtocol",
            stringify!(request_connection_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn request_ur_ls(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestUrLsRequest,
    ) -> Result<RequestUrLsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SecureConnectionProtocol",
            stringify!(request_ur_ls)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn register_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterExRequest,
    ) -> Result<RegisterExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SecureConnectionProtocol",
            stringify!(register_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
