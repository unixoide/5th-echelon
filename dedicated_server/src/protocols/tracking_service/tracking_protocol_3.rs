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

use super::types::*;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum TrackingProtocol3Method {
    SendTag = 1u32,
    SendTagAndUpdateUserInfo = 2u32,
    SendUserInfo = 3u32,
    GetConfiguration = 4u32,
    SendTags = 5u32,
}
#[derive(Debug, FromStream)]
pub struct SendTagRequest {
    pub tracking_id: u32,
    pub tag: String,
    pub attributes: String,
    pub delta_time: u32,
}
#[derive(Debug, ToStream)]
pub struct SendTagResponse;
#[derive(Debug, FromStream)]
pub struct SendTagAndUpdateUserInfoRequest {
    pub tracking_id: u32,
    pub tag: String,
    pub attributes: String,
    pub delta_time: u32,
    pub user_id: String,
}
#[derive(Debug, ToStream)]
pub struct SendTagAndUpdateUserInfoResponse;
#[derive(Debug, FromStream)]
pub struct SendUserInfoRequest {
    pub delta_time: u32,
}
#[derive(Debug, ToStream)]
pub struct SendUserInfoResponse {
    pub user_info: TrackingInformation,
    pub tracking_id: u32,
}
#[derive(Debug, FromStream)]
pub struct GetConfigurationRequest;
#[derive(Debug, ToStream)]
pub struct GetConfigurationResponse {
    pub tags: Vec<String>,
}
#[derive(Debug, FromStream)]
pub struct SendTagsRequest {
    pub tag_data: Vec<TrackingTag>,
}
#[derive(Debug, ToStream)]
pub struct SendTagsResponse;
pub struct TrackingProtocol3<T: TrackingProtocol3Trait<CI>, CI>(T, ::std::marker::PhantomData<CI>);
impl<T: TrackingProtocol3Trait<CI>, CI> TrackingProtocol3<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: TrackingProtocol3Trait<CI>, CI> Protocol<CI> for TrackingProtocol3<T, CI> {
    fn id(&self) -> u16 {
        36
    }
    fn name(&self) -> String {
        "TrackingProtocol3".to_string()
    }
    fn num_methods(&self) -> u32 {
        5u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = TrackingProtocol3Method::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(TrackingProtocol3Method::SendTag) => {
                let req = SendTagRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_tag(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TrackingProtocol3Method::SendTagAndUpdateUserInfo) => {
                let req = SendTagAndUpdateUserInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_tag_and_update_user_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TrackingProtocol3Method::SendUserInfo) => {
                let req = SendUserInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_user_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TrackingProtocol3Method::GetConfiguration) => {
                let req = GetConfigurationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_configuration(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TrackingProtocol3Method::SendTags) => {
                let req = SendTagsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_tags(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        TrackingProtocol3Method::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait TrackingProtocol3Trait<CI> {
    fn send_tag(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendTagRequest,
    ) -> Result<SendTagResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingProtocol3",
            stringify!(send_tag)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn send_tag_and_update_user_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendTagAndUpdateUserInfoRequest,
    ) -> Result<SendTagAndUpdateUserInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingProtocol3",
            stringify!(send_tag_and_update_user_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn send_user_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendUserInfoRequest,
    ) -> Result<SendUserInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingProtocol3",
            stringify!(send_user_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_configuration(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetConfigurationRequest,
    ) -> Result<GetConfigurationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingProtocol3",
            stringify!(get_configuration)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn send_tags(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendTagsRequest,
    ) -> Result<SendTagsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingProtocol3",
            stringify!(send_tags)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
