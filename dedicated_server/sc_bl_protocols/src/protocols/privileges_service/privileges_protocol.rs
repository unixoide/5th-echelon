// AUTOGENERATED with quazal-tools
#![allow(
    clippy::enum_variant_names,
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::upper_case_acronyms,
    clippy::wildcard_imports
)]
use std::convert::TryFrom;

use num_enum::TryFromPrimitive;
use quazal::prudp::ClientRegistry;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::ClientProtocol;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use super::types::*;
pub const PRIVILEGES_PROTOCOL_ID: u16 = 35u16;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum PrivilegesProtocolMethod {
    GetPrivileges = 1u32,
    ActivateKey = 2u32,
    ActivateKeyWithExpectedPrivileges = 3u32,
    GetPrivilegeRemainDuration = 4u32,
    GetExpiredPrivileges = 5u32,
    GetPrivilegesEx = 6u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegesRequest {
    pub locale_code: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegesResponse {
    pub privileges: std::collections::HashMap<u32, Privilege>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ActivateKeyRequest {
    pub unique_key: String,
    pub language_code: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ActivateKeyResponse {
    pub privilege: PrivilegeGroup,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ActivateKeyWithExpectedPrivilegesRequest {
    pub unique_key: String,
    pub language_code: String,
    pub expected_privilege_ids: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ActivateKeyWithExpectedPrivilegesResponse {
    pub privilege: PrivilegeGroup,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegeRemainDurationRequest {
    pub privilege_id: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegeRemainDurationResponse {
    pub seconds: i32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetExpiredPrivilegesRequest;
#[derive(Debug, ToStream, FromStream)]
pub struct GetExpiredPrivilegesResponse {
    pub expired_privileges: quazal::rmc::types::QList<PrivilegeEx>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegesExRequest {
    pub locale_code: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetPrivilegesExResponse {
    pub privileges_ex: quazal::rmc::types::QList<PrivilegeEx>,
}
pub struct PrivilegesProtocolServer<T: PrivilegesProtocolServerTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: PrivilegesProtocolServerTrait<CI>, CI> PrivilegesProtocolServer<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: PrivilegesProtocolServerTrait<CI>, CI> Protocol<CI> for PrivilegesProtocolServer<T, CI> {
    fn id(&self) -> u16 {
        PRIVILEGES_PROTOCOL_ID
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
        client_registry: &ClientRegistry<CI>,
        socket: &std::net::UdpSocket,
    ) -> Result<Vec<u8>, Error> {
        let method = PrivilegesProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(PrivilegesProtocolMethod::GetPrivileges) => {
                let req = GetPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_privileges(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(PrivilegesProtocolMethod::ActivateKey) => {
                let req = ActivateKeyRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .activate_key(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(PrivilegesProtocolMethod::ActivateKeyWithExpectedPrivileges) => {
                let req =
                    ActivateKeyWithExpectedPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.activate_key_with_expected_privileges(
                    logger,
                    ctx,
                    ci,
                    req,
                    client_registry,
                    socket,
                );
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(PrivilegesProtocolMethod::GetPrivilegeRemainDuration) => {
                let req = GetPrivilegeRemainDurationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_privilege_remain_duration(
                    logger,
                    ctx,
                    ci,
                    req,
                    client_registry,
                    socket,
                );
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(PrivilegesProtocolMethod::GetExpiredPrivileges) => {
                let req = GetExpiredPrivilegesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_expired_privileges(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(PrivilegesProtocolMethod::GetPrivilegesEx) => {
                let req = GetPrivilegesExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_privileges_ex(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
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
pub trait PrivilegesProtocolServerTrait<CI> {
    fn get_privileges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPrivilegesRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
pub struct PrivilegesProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> PrivilegesProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> ClientProtocol<CI> for PrivilegesProtocolClient<CI> {
    fn id(&self) -> u16 {
        PRIVILEGES_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "PrivilegesProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        6u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        PrivilegesProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> PrivilegesProtocolClient<CI> {
    pub fn get_privileges(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::GetPrivileges as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn activate_key(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::ActivateKey as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn activate_key_with_expected_privileges(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::ActivateKeyWithExpectedPrivileges as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_privilege_remain_duration(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::GetPrivilegeRemainDuration as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_expired_privileges(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::GetExpiredPrivileges as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_privileges_ex(
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
        self.send(
            logger,
            ctx,
            ci,
            PrivilegesProtocolMethod::GetPrivilegesEx as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}