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
enum HealthProtocolMethod {
    PingDaemon = 1u32,
    PingDatabase = 2u32,
    RunSanityCheck = 3u32,
    FixSanityErrors = 4u32,
}
#[derive(Debug, FromStream)]
pub struct PingDaemonRequest;
#[derive(Debug, ToStream)]
pub struct PingDaemonResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct PingDatabaseRequest;
#[derive(Debug, ToStream)]
pub struct PingDatabaseResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct RunSanityCheckRequest;
#[derive(Debug, ToStream)]
pub struct RunSanityCheckResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct FixSanityErrorsRequest;
#[derive(Debug, ToStream)]
pub struct FixSanityErrorsResponse {
    pub return_value: bool,
}
pub struct HealthProtocol<T: HealthProtocolTrait<CI>, CI>(T, ::std::marker::PhantomData<CI>);
impl<T: HealthProtocolTrait<CI>, CI> HealthProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: HealthProtocolTrait<CI>, CI> Protocol<CI> for HealthProtocol<T, CI> {
    fn id(&self) -> u16 {
        18u16
    }
    fn name(&self) -> String {
        "HealthProtocol".to_string()
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
        let method = HealthProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(HealthProtocolMethod::PingDaemon) => {
                let req = PingDaemonRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ping_daemon(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(HealthProtocolMethod::PingDatabase) => {
                let req = PingDatabaseRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ping_database(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(HealthProtocolMethod::RunSanityCheck) => {
                let req = RunSanityCheckRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.run_sanity_check(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(HealthProtocolMethod::FixSanityErrors) => {
                let req = FixSanityErrorsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.fix_sanity_errors(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        HealthProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait HealthProtocolTrait<CI> {
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
            "HealthProtocol",
            stringify!(ping_daemon)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn ping_database(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PingDatabaseRequest,
    ) -> Result<PingDatabaseResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "HealthProtocol",
            stringify!(ping_database)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn run_sanity_check(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RunSanityCheckRequest,
    ) -> Result<RunSanityCheckResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "HealthProtocol",
            stringify!(run_sanity_check)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn fix_sanity_errors(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: FixSanityErrorsRequest,
    ) -> Result<FixSanityErrorsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "HealthProtocol",
            stringify!(fix_sanity_errors)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
