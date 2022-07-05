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
enum WebNotificationsStorageProtocolMethod {
    RegisterUser = 1u32,
    PollNotifications = 2u32,
    UnregisterUser = 3u32,
}
#[derive(Debug, FromStream)]
pub struct RegisterUserRequest;
#[derive(Debug, ToStream)]
pub struct RegisterUserResponse;
#[derive(Debug, FromStream)]
pub struct PollNotificationsRequest;
#[derive(Debug, ToStream)]
pub struct PollNotificationsResponse {
    pub list_notifications: String,
    pub nb_notifications: i32,
}
#[derive(Debug, FromStream)]
pub struct UnregisterUserRequest;
#[derive(Debug, ToStream)]
pub struct UnregisterUserResponse;
pub struct WebNotificationsStorageProtocol<T: WebNotificationsStorageProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: WebNotificationsStorageProtocolTrait<CI>, CI> WebNotificationsStorageProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: WebNotificationsStorageProtocolTrait<CI>, CI> Protocol<CI>
    for WebNotificationsStorageProtocol<T, CI>
{
    fn id(&self) -> u16 {
        todo!()
    }
    fn name(&self) -> String {
        "WebNotificationsStorageProtocol".to_string()
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
        let method = WebNotificationsStorageProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(WebNotificationsStorageProtocolMethod::RegisterUser) => {
                let req = RegisterUserRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register_user(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(WebNotificationsStorageProtocolMethod::PollNotifications) => {
                let req = PollNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.poll_notifications(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(WebNotificationsStorageProtocolMethod::UnregisterUser) => {
                let req = UnregisterUserRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.unregister_user(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        WebNotificationsStorageProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait WebNotificationsStorageProtocolTrait<CI> {
    fn register_user(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "WebNotificationsStorageProtocol",
            stringify!(register_user)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn poll_notifications(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PollNotificationsRequest,
    ) -> Result<PollNotificationsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "WebNotificationsStorageProtocol",
            stringify!(poll_notifications)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unregister_user(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnregisterUserRequest,
    ) -> Result<UnregisterUserResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "WebNotificationsStorageProtocol",
            stringify!(unregister_user)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
