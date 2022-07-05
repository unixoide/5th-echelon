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
enum PrivilegesProtocolMethod {
    GetPrivileges = 1u32,
    ActivateKey = 2u32,
    ActivateKeyWithExpectedPrivileges = 3u32,
    GetPrivilegeRemainDuration = 4u32,
    GetExpiredPrivileges = 5u32,
    GetPrivilegesEx = 6u32,
}
#[derive(Debug, FromStream)]
pub struct GetPrivilegesRequest {
    pub locale_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetPrivilegesResponse {
    pub privileges: std::collections::HashMap<u32, Privilege>,
}
#[derive(Debug, FromStream)]
pub struct ActivateKeyRequest {
    pub unique_key: String,
    pub language_code: String,
}
#[derive(Debug, ToStream)]
pub struct ActivateKeyResponse {
    pub privilege: PrivilegeGroup,
}
#[derive(Debug, FromStream)]
pub struct ActivateKeyWithExpectedPrivilegesRequest {
    pub unique_key: String,
    pub language_code: String,
    pub expected_privilege_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct ActivateKeyWithExpectedPrivilegesResponse {
    pub privilege: PrivilegeGroup,
}
#[derive(Debug, FromStream)]
pub struct GetPrivilegeRemainDurationRequest {
    pub privilege_id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetPrivilegeRemainDurationResponse {
    pub seconds: i32,
}
#[derive(Debug, FromStream)]
pub struct GetExpiredPrivilegesRequest;
#[derive(Debug, ToStream)]
pub struct GetExpiredPrivilegesResponse {
    pub expired_privileges: quazal::rmc::types::QList<PrivilegeEx>,
}
#[derive(Debug, FromStream)]
pub struct GetPrivilegesExRequest {
    pub locale_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetPrivilegesExResponse {
    pub privileges_ex: quazal::rmc::types::QList<PrivilegeEx>,
}
pub struct PrivilegesProtocol<T: PrivilegesProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: PrivilegesProtocolTrait<CI>, CI> PrivilegesProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: PrivilegesProtocolTrait<CI>, CI> Protocol<CI> for PrivilegesProtocol<T, CI> {
    fn id(&self) -> u16 {
        35u16
    }
    fn name(&self) -> String {
        "PrivilegesProtocol".to_string()
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
        let method = PrivilegesProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(PrivilegesProtocolMethod::GetPrivileges) => {
                let req = GetPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_privileges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PrivilegesProtocolMethod::ActivateKey) => {
                let req = ActivateKeyRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.activate_key(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PrivilegesProtocolMethod::ActivateKeyWithExpectedPrivileges) => {
                let req =
                    ActivateKeyWithExpectedPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .activate_key_with_expected_privileges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PrivilegesProtocolMethod::GetPrivilegeRemainDuration) => {
                let req = GetPrivilegeRemainDurationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_privilege_remain_duration(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PrivilegesProtocolMethod::GetExpiredPrivileges) => {
                let req = GetExpiredPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_expired_privileges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PrivilegesProtocolMethod::GetPrivilegesEx) => {
                let req = GetPrivilegesExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_privileges_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        PrivilegesProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait PrivilegesProtocolTrait<CI> {
    fn get_privileges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPrivilegesRequest,
    ) -> Result<GetPrivilegesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(get_privileges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn activate_key(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ActivateKeyRequest,
    ) -> Result<ActivateKeyResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(activate_key)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn activate_key_with_expected_privileges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ActivateKeyWithExpectedPrivilegesRequest,
    ) -> Result<ActivateKeyWithExpectedPrivilegesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(activate_key_with_expected_privileges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_privilege_remain_duration(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPrivilegeRemainDurationRequest,
    ) -> Result<GetPrivilegeRemainDurationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(get_privilege_remain_duration)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_expired_privileges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetExpiredPrivilegesRequest,
    ) -> Result<GetExpiredPrivilegesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(get_expired_privileges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_privileges_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPrivilegesExRequest,
    ) -> Result<GetPrivilegesExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PrivilegesProtocol",
            stringify!(get_privileges_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
