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
pub const NAT_TRAVERSAL_PROTOCOL_ID: u16 = 3u16;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum NatTraversalProtocolMethod {
    RequestProbeInitiation = 1u32,
    InitiateProbe = 2u32,
    RequestProbeInitiationExt = 3u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct RequestProbeInitiationRequest {
    pub url_target_list: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct RequestProbeInitiationResponse;
#[derive(Debug, ToStream, FromStream)]
pub struct InitiateProbeRequest {
    pub url_station_to_probe: quazal::rmc::types::StationURL,
}
#[derive(Debug, ToStream, FromStream)]
pub struct InitiateProbeResponse;
#[derive(Debug, ToStream, FromStream)]
pub struct RequestProbeInitiationExtRequest {
    pub url_target_list: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
    pub url_station_to_probe: quazal::rmc::types::StationURL,
}
#[derive(Debug, ToStream, FromStream)]
pub struct RequestProbeInitiationExtResponse;
pub struct NatTraversalProtocolServer<T: NatTraversalProtocolServerTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: NatTraversalProtocolServerTrait<CI>, CI> NatTraversalProtocolServer<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: NatTraversalProtocolServerTrait<CI>, CI> Protocol<CI>
    for NatTraversalProtocolServer<T, CI>
{
    fn id(&self) -> u16 {
        NAT_TRAVERSAL_PROTOCOL_ID
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
        client_registry: &ClientRegistry<CI>,
        socket: &std::net::UdpSocket,
    ) -> Result<Vec<u8>, Error> {
        let method = NatTraversalProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(NatTraversalProtocolMethod::RequestProbeInitiation) => {
                let req = RequestProbeInitiationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .request_probe_initiation(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NatTraversalProtocolMethod::InitiateProbe) => {
                let req = InitiateProbeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .initiate_probe(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NatTraversalProtocolMethod::RequestProbeInitiationExt) => {
                let req = RequestProbeInitiationExtRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.request_probe_initiation_ext(
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
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NatTraversalProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait NatTraversalProtocolServerTrait<CI> {
    fn request_probe_initiation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RequestProbeInitiationRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
pub struct NatTraversalProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> NatTraversalProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> ClientProtocol<CI> for NatTraversalProtocolClient<CI> {
    fn id(&self) -> u16 {
        NAT_TRAVERSAL_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "NatTraversalProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        3u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NatTraversalProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> NatTraversalProtocolClient<CI> {
    pub fn request_probe_initiation(
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
        self.send(
            logger,
            ctx,
            ci,
            NatTraversalProtocolMethod::RequestProbeInitiation as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn initiate_probe(
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
        self.send(
            logger,
            ctx,
            ci,
            NatTraversalProtocolMethod::InitiateProbe as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn request_probe_initiation_ext(
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
        self.send(
            logger,
            ctx,
            ci,
            NatTraversalProtocolMethod::RequestProbeInitiationExt as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}