#![allow(
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::convert::TryFrom;

use num_enum::TryFromPrimitive;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum NatTraversalProtocolMethod {
    RequestProbeInitiation = 1u32,
    InitiateProbe = 2u32,
    RequestProbeInitiationExt = 3u32,
}
#[derive(Debug, FromStream)]
pub struct RequestProbeInitiationRequest {
    pub url_target_list: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream)]
pub struct RequestProbeInitiationResponse;
#[derive(Debug, FromStream)]
pub struct InitiateProbeRequest {
    pub url_station_to_probe: quazal::rmc::types::StationURL,
}
#[derive(Debug, ToStream)]
pub struct InitiateProbeResponse;
#[derive(Debug, FromStream)]
pub struct RequestProbeInitiationExtRequest {
    pub url_target_list: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
    pub url_station_to_probe: quazal::rmc::types::StationURL,
}
#[derive(Debug, ToStream)]
pub struct RequestProbeInitiationExtResponse;
pub struct NatTraversalProtocol<T: NatTraversalProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: NatTraversalProtocolTrait<CI>, CI> NatTraversalProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: NatTraversalProtocolTrait<CI>, CI> Protocol<CI> for NatTraversalProtocol<T, CI> {
    fn id(&self) -> u16 {
        3
    }
    fn name(&self) -> String {
        "NatTraversalProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        3u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = NatTraversalProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(NatTraversalProtocolMethod::RequestProbeInitiation) => {
                let req = RequestProbeInitiationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_probe_initiation(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NatTraversalProtocolMethod::InitiateProbe) => {
                let req = InitiateProbeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.initiate_probe(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NatTraversalProtocolMethod::RequestProbeInitiationExt) => {
                let req = RequestProbeInitiationExtRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_probe_initiation_ext(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NatTraversalProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait NatTraversalProtocolTrait<CI> {
    fn request_probe_initiation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestProbeInitiationRequest,
    ) -> Result<RequestProbeInitiationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NatTraversalProtocol",
            stringify!(request_probe_initiation)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn initiate_probe(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: InitiateProbeRequest,
    ) -> Result<InitiateProbeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NatTraversalProtocol",
            stringify!(initiate_probe)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn request_probe_initiation_ext(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestProbeInitiationExtRequest,
    ) -> Result<RequestProbeInitiationExtResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NatTraversalProtocol",
            stringify!(request_probe_initiation_ext)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
