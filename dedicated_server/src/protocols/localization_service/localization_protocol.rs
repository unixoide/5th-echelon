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
enum LocalizationProtocolMethod {
    GetLocaleCode = 1u32,
    SetLocaleCode = 2u32,
}
#[derive(Debug, FromStream)]
pub struct GetLocaleCodeRequest;
#[derive(Debug, ToStream)]
pub struct GetLocaleCodeResponse {
    pub local_code: String,
}
#[derive(Debug, FromStream)]
pub struct SetLocaleCodeRequest {
    pub local_code: String,
}
#[derive(Debug, ToStream)]
pub struct SetLocaleCodeResponse;
pub struct LocalizationProtocol<T: LocalizationProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: LocalizationProtocolTrait<CI>, CI> LocalizationProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: LocalizationProtocolTrait<CI>, CI> Protocol<CI> for LocalizationProtocol<T, CI> {
    fn id(&self) -> u16 {
        39u16
    }
    fn name(&self) -> String {
        "LocalizationProtocol".to_string()
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
        let method = LocalizationProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(LocalizationProtocolMethod::GetLocaleCode) => {
                let req = GetLocaleCodeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_locale_code(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(LocalizationProtocolMethod::SetLocaleCode) => {
                let req = SetLocaleCodeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.set_locale_code(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        LocalizationProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait LocalizationProtocolTrait<CI> {
    fn get_locale_code(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetLocaleCodeRequest,
    ) -> Result<GetLocaleCodeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LocalizationProtocol",
            stringify!(get_locale_code)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn set_locale_code(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SetLocaleCodeRequest,
    ) -> Result<SetLocaleCodeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LocalizationProtocol",
            stringify!(set_locale_code)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
