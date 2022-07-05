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
enum RemoteLogDeviceProtocolMethod {
    Log = 1u32,
}
#[derive(Debug, FromStream)]
pub struct LogRequest {
    pub str_line: String,
}
#[derive(Debug, ToStream)]
pub struct LogResponse;
pub struct RemoteLogDeviceProtocol<T: RemoteLogDeviceProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: RemoteLogDeviceProtocolTrait<CI>, CI> RemoteLogDeviceProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: RemoteLogDeviceProtocolTrait<CI>, CI> Protocol<CI> for RemoteLogDeviceProtocol<T, CI> {
    fn id(&self) -> u16 {
        todo!()
    }
    fn name(&self) -> String {
        "RemoteLogDeviceProtocol".to_string()
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
        let method = RemoteLogDeviceProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(RemoteLogDeviceProtocolMethod::Log) => {
                let req = LogRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.log(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        RemoteLogDeviceProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait RemoteLogDeviceProtocolTrait<CI> {
    fn log(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LogRequest,
    ) -> Result<LogResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "RemoteLogDeviceProtocol",
            stringify!(log)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
