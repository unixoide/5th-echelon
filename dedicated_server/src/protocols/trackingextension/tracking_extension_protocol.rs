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
enum TrackingExtensionProtocolMethod {
    GetTrackingUserGroup = 1u32,
    GetTrackingUserGroupTags = 2u32,
}
#[derive(Debug, FromStream)]
pub struct GetTrackingUserGroupRequest {
    pub pid: u32,
}
#[derive(Debug, ToStream)]
pub struct GetTrackingUserGroupResponse {
    pub usergroup: u32,
}
#[derive(Debug, FromStream)]
pub struct GetTrackingUserGroupTagsRequest {
    pub usergroup: u32,
}
#[derive(Debug, ToStream)]
pub struct GetTrackingUserGroupTagsResponse {
    pub tags: Vec<String>,
}
pub struct TrackingExtensionProtocol<T: TrackingExtensionProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: TrackingExtensionProtocolTrait<CI>, CI> TrackingExtensionProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: TrackingExtensionProtocolTrait<CI>, CI> Protocol<CI> for TrackingExtensionProtocol<T, CI> {
    fn id(&self) -> u16 {
        1001u16
    }
    fn name(&self) -> String {
        "TrackingExtensionProtocol".to_string()
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
        let method = TrackingExtensionProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(TrackingExtensionProtocolMethod::GetTrackingUserGroup) => {
                let req = GetTrackingUserGroupRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_tracking_user_group(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(TrackingExtensionProtocolMethod::GetTrackingUserGroupTags) => {
                let req = GetTrackingUserGroupTagsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_tracking_user_group_tags(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        TrackingExtensionProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait TrackingExtensionProtocolTrait<CI> {
    fn get_tracking_user_group(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetTrackingUserGroupRequest,
    ) -> Result<GetTrackingUserGroupResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingExtensionProtocol",
            stringify!(get_tracking_user_group)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_tracking_user_group_tags(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetTrackingUserGroupTagsRequest,
    ) -> Result<GetTrackingUserGroupTagsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "TrackingExtensionProtocol",
            stringify!(get_tracking_user_group_tags)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
