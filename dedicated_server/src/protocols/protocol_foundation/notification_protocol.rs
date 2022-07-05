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
enum NotificationProtocolMethod {
    ProcessNotificationEvent = 1u32,
}
#[derive(Debug, FromStream)]
pub struct ProcessNotificationEventRequest {
    pub o_event: NotificationEvent,
}
#[derive(Debug, ToStream)]
pub struct ProcessNotificationEventResponse;
pub struct NotificationProtocol<T: NotificationProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: NotificationProtocolTrait<CI>, CI> NotificationProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: NotificationProtocolTrait<CI>, CI> Protocol<CI> for NotificationProtocol<T, CI> {
    fn id(&self) -> u16 {
        todo!()
    }
    fn name(&self) -> String {
        "NotificationProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        1u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = NotificationProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(NotificationProtocolMethod::ProcessNotificationEvent) => {
                let req = ProcessNotificationEventRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.process_notification_event(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NotificationProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait NotificationProtocolTrait<CI> {
    fn process_notification_event(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ProcessNotificationEventRequest,
    ) -> Result<ProcessNotificationEventResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NotificationProtocol",
            stringify!(process_notification_event)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
