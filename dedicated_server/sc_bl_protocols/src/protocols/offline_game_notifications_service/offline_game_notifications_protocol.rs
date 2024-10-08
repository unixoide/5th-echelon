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

use super::super::protocol_foundation::types::NotificationEvent;
#[allow(unused)]
use super::types::*;
pub const OFFLINE_GAME_NOTIFICATIONS_PROTOCOL_ID: u16 = 71u16;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum OfflineGameNotificationsProtocolMethod {
    PollNotifications = 1u32,
    PollSpecificOfflineNotifications = 2u32,
    PollAnyOfflineNotifications = 3u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PollNotificationsRequest;
#[derive(Debug, ToStream, FromStream)]
pub struct PollNotificationsResponse {
    pub list_notifications: quazal::rmc::types::QList<NotificationEvent>,
    pub nb_remaining_notifs: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PollSpecificOfflineNotificationsRequest {
    pub majortype: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PollSpecificOfflineNotificationsResponse {
    pub list_timed_notification: quazal::rmc::types::QList<TimedNotification>,
    pub ret: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PollAnyOfflineNotificationsRequest;
#[derive(Debug, ToStream, FromStream)]
pub struct PollAnyOfflineNotificationsResponse {
    pub list_timed_notification: quazal::rmc::types::QList<TimedNotification>,
    pub nb_remaining_notifs: u32,
}
pub struct OfflineGameNotificationsProtocolServer<
    T: OfflineGameNotificationsProtocolServerTrait<CI>,
    CI,
>(T, ::std::marker::PhantomData<CI>);
impl<T: OfflineGameNotificationsProtocolServerTrait<CI>, CI>
    OfflineGameNotificationsProtocolServer<T, CI>
{
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: OfflineGameNotificationsProtocolServerTrait<CI>, CI> Protocol<CI>
    for OfflineGameNotificationsProtocolServer<T, CI>
{
    fn id(&self) -> u16 {
        OFFLINE_GAME_NOTIFICATIONS_PROTOCOL_ID
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
        client_registry: &ClientRegistry<CI>,
        socket: &std::net::UdpSocket,
    ) -> Result<Vec<u8>, Error> {
        let method = OfflineGameNotificationsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(OfflineGameNotificationsProtocolMethod::PollNotifications) => {
                let req = PollNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .poll_notifications(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(OfflineGameNotificationsProtocolMethod::PollSpecificOfflineNotifications) => {
                let req = PollSpecificOfflineNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.poll_specific_offline_notifications(
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
            Some(OfflineGameNotificationsProtocolMethod::PollAnyOfflineNotifications) => {
                let req = PollAnyOfflineNotificationsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.poll_any_offline_notifications(
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
        OfflineGameNotificationsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait OfflineGameNotificationsProtocolServerTrait<CI> {
    fn poll_notifications(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: PollNotificationsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
pub struct OfflineGameNotificationsProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> OfflineGameNotificationsProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> Default for OfflineGameNotificationsProtocolClient<CI> {
    fn default() -> Self {
        Self::new()
    }
}
impl<CI> ClientProtocol<CI> for OfflineGameNotificationsProtocolClient<CI> {
    fn id(&self) -> u16 {
        OFFLINE_GAME_NOTIFICATIONS_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "OfflineGameNotificationsProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        3u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        OfflineGameNotificationsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> OfflineGameNotificationsProtocolClient<CI> {
    pub fn poll_notifications(
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
        self.send(
            logger,
            ctx,
            ci,
            OfflineGameNotificationsProtocolMethod::PollNotifications as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn poll_specific_offline_notifications(
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
        self.send(
            logger,
            ctx,
            ci,
            OfflineGameNotificationsProtocolMethod::PollSpecificOfflineNotifications as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn poll_any_offline_notifications(
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
        self.send(
            logger,
            ctx,
            ci,
            OfflineGameNotificationsProtocolMethod::PollAnyOfflineNotifications as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
