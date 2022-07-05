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
enum OfflineGameNotificationsProtocolMethod {
    PollNotifications = 1u32,
    PollSpecificOfflineNotifications = 2u32,
    PollAnyOfflineNotifications = 3u32,
}
#[derive(Debug, FromStream)]
pub struct PollNotificationsRequest;
#[derive(Debug, ToStream)]
pub struct PollNotificationsResponse {
    pub list_notifications: quazal::rmc::types::QList<NotificationEvent>,
    pub nb_remaining_notifs: u32,
}
#[derive(Debug, FromStream)]
pub struct PollSpecificOfflineNotificationsRequest {
    pub majortype: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct PollSpecificOfflineNotificationsResponse {
    pub list_timed_notification: quazal::rmc::types::QList<TimedNotification>,
    pub ret: u32,
}
#[derive(Debug, FromStream)]
pub struct PollAnyOfflineNotificationsRequest;
#[derive(Debug, ToStream)]
pub struct PollAnyOfflineNotificationsResponse {
    pub list_timed_notification: quazal::rmc::types::QList<TimedNotification>,
    pub nb_remaining_notifs: u32,
}
pub struct OfflineGameNotificationsProtocol<T: OfflineGameNotificationsProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: OfflineGameNotificationsProtocolTrait<CI>, CI> OfflineGameNotificationsProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: OfflineGameNotificationsProtocolTrait<CI>, CI> Protocol<CI>
    for OfflineGameNotificationsProtocol<T, CI>
{
    fn id(&self) -> u16 {
        71u16
    }
    fn name(&self) -> String {
        "OfflineGameNotificationsProtocol".to_string()
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
        let method = OfflineGameNotificationsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(OfflineGameNotificationsProtocolMethod::PollNotifications) => {
                let req = PollNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.poll_notifications(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(OfflineGameNotificationsProtocolMethod::PollSpecificOfflineNotifications) => {
                let req = PollSpecificOfflineNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .poll_specific_offline_notifications(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(OfflineGameNotificationsProtocolMethod::PollAnyOfflineNotifications) => {
                let req = PollAnyOfflineNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.poll_any_offline_notifications(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        OfflineGameNotificationsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait OfflineGameNotificationsProtocolTrait<CI> {
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
            "OfflineGameNotificationsProtocol",
            stringify!(poll_notifications)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn poll_specific_offline_notifications(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PollSpecificOfflineNotificationsRequest,
    ) -> Result<PollSpecificOfflineNotificationsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "OfflineGameNotificationsProtocol",
            stringify!(poll_specific_offline_notifications)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn poll_any_offline_notifications(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PollAnyOfflineNotificationsRequest,
    ) -> Result<PollAnyOfflineNotificationsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "OfflineGameNotificationsProtocol",
            stringify!(poll_any_offline_notifications)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
