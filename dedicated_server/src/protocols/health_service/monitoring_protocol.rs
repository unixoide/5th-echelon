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
enum MonitoringProtocolMethod {
    PingDaemon = 1u32,
    GetClusterMembers = 2u32,
}
#[derive(Debug, FromStream)]
pub struct PingDaemonRequest;
#[derive(Debug, ToStream)]
pub struct PingDaemonResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct GetClusterMembersRequest;
#[derive(Debug, ToStream)]
pub struct GetClusterMembersResponse {
    pub str_values: Vec<String>,
}
pub struct MonitoringProtocol<T: MonitoringProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: MonitoringProtocolTrait<CI>, CI> MonitoringProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: MonitoringProtocolTrait<CI>, CI> Protocol<CI> for MonitoringProtocol<T, CI> {
    fn id(&self) -> u16 {
        19u16
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
    ) -> Result<Vec<u8>, Error> {
        let method = MonitoringProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(MonitoringProtocolMethod::PingDaemon) => {
                let req = PingDaemonRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ping_daemon(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(MonitoringProtocolMethod::GetClusterMembers) => {
                let req = GetClusterMembersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_cluster_members(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
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
pub trait MonitoringProtocolTrait<CI> {
    fn ping_daemon(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_cluster_members(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
