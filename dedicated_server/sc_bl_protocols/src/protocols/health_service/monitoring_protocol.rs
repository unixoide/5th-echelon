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

#[allow(unused)]
use super::types::*;
pub const MONITORING_PROTOCOL_ID: u16 = 19u16;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum MonitoringProtocolMethod {
    PingDaemon = 1u32,
    GetClusterMembers = 2u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PingDaemonRequest;
#[derive(Debug, ToStream, FromStream)]
pub struct PingDaemonResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetClusterMembersRequest;
#[derive(Debug, ToStream, FromStream)]
pub struct GetClusterMembersResponse {
    pub str_values: Vec<String>,
}
pub struct MonitoringProtocolServer<T: MonitoringProtocolServerTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: MonitoringProtocolServerTrait<CI>, CI> MonitoringProtocolServer<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: MonitoringProtocolServerTrait<CI>, CI> Protocol<CI> for MonitoringProtocolServer<T, CI> {
    fn id(&self) -> u16 {
        MONITORING_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "MonitoringProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        2u32
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
        let method = MonitoringProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(MonitoringProtocolMethod::PingDaemon) => {
                let req = PingDaemonRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .ping_daemon(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(MonitoringProtocolMethod::GetClusterMembers) => {
                let req = GetClusterMembersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_cluster_members(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        MonitoringProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait MonitoringProtocolServerTrait<CI> {
    fn ping_daemon(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PingDaemonRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<PingDaemonResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "MonitoringProtocol",
            stringify!(ping_daemon)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_cluster_members(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetClusterMembersRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetClusterMembersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "MonitoringProtocol",
            stringify!(get_cluster_members)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
pub struct MonitoringProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> MonitoringProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> Default for MonitoringProtocolClient<CI> {
    fn default() -> Self {
        Self::new()
    }
}
impl<CI> ClientProtocol<CI> for MonitoringProtocolClient<CI> {
    fn id(&self) -> u16 {
        MONITORING_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "MonitoringProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        2u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        MonitoringProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> MonitoringProtocolClient<CI> {
    pub fn ping_daemon(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PingDaemonRequest,
    ) -> Result<PingDaemonResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "MonitoringProtocol",
            stringify!(ping_daemon)
        );
        self.send(
            logger,
            ctx,
            ci,
            MonitoringProtocolMethod::PingDaemon as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_cluster_members(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetClusterMembersRequest,
    ) -> Result<GetClusterMembersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "MonitoringProtocol",
            stringify!(get_cluster_members)
        );
        self.send(
            logger,
            ctx,
            ci,
            MonitoringProtocolMethod::GetClusterMembers as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
