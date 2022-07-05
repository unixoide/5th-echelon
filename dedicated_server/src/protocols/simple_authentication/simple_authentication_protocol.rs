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
enum SimpleAuthenticationProtocolMethod {
    Authenticate = 1u32,
    LoginWithToken = 2u32,
    LoginWithTokenEx = 3u32,
    Login = 4u32,
    LoginWithSubAccount = 5u32,
    Register = 6u32,
    RegisterEx = 7u32,
    LoginWithTokenCafe = 8u32,
    LoginWithTokenCafeEx = 9u32,
}
#[derive(Debug, FromStream)]
pub struct AuthenticateRequest {
    pub str_user_name: String,
}
#[derive(Debug, ToStream)]
pub struct AuthenticateResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginWithTokenRequest {
    pub str_token: String,
}
#[derive(Debug, ToStream)]
pub struct LoginWithTokenResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginWithTokenExRequest {
    pub str_token: String,
    pub o_any_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LoginWithTokenExResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginRequest {
    pub str_username: String,
    pub str_password: String,
}
#[derive(Debug, ToStream)]
pub struct LoginResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginWithSubAccountRequest {
    pub login_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LoginWithSubAccountResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct RegisterRequest {
    pub vec_my_ur_ls: Vec<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream)]
pub struct RegisterResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_connection_id: u32,
    pub url_public: quazal::rmc::types::StationURL,
}
#[derive(Debug, FromStream)]
pub struct RegisterExRequest {
    pub vec_my_ur_ls: Vec<quazal::rmc::types::StationURL>,
    pub h_custom_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct RegisterExResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_connection_id: u32,
    pub url_public: quazal::rmc::types::StationURL,
}
#[derive(Debug, FromStream)]
pub struct LoginWithTokenCafeRequest {
    pub str_nintendo_token: String,
}
#[derive(Debug, ToStream)]
pub struct LoginWithTokenCafeResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
#[derive(Debug, FromStream)]
pub struct LoginWithTokenCafeExRequest {
    pub str_nintendo_token: String,
    pub o_any_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LoginWithTokenCafeExResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub pid_principal: u32,
    pub p_connection_data: RVConnectionData,
    pub str_return_msg: String,
}
pub struct SimpleAuthenticationProtocol<T: SimpleAuthenticationProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: SimpleAuthenticationProtocolTrait<CI>, CI> SimpleAuthenticationProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: SimpleAuthenticationProtocolTrait<CI>, CI> Protocol<CI>
    for SimpleAuthenticationProtocol<T, CI>
{
    fn id(&self) -> u16 {
        16u16
    }
    fn name(&self) -> String {
        "SimpleAuthenticationProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        9u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = SimpleAuthenticationProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(SimpleAuthenticationProtocolMethod::Authenticate) => {
                let req = AuthenticateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.authenticate(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::LoginWithToken) => {
                let req = LoginWithTokenRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_token(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::LoginWithTokenEx) => {
                let req = LoginWithTokenExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_token_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::Login) => {
                let req = LoginRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::LoginWithSubAccount) => {
                let req = LoginWithSubAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_sub_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::Register) => {
                let req = RegisterRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::RegisterEx) => {
                let req = RegisterExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::LoginWithTokenCafe) => {
                let req = LoginWithTokenCafeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_token_cafe(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(SimpleAuthenticationProtocolMethod::LoginWithTokenCafeEx) => {
                let req = LoginWithTokenCafeExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.login_with_token_cafe_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        SimpleAuthenticationProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait SimpleAuthenticationProtocolTrait<CI> {
    fn authenticate(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AuthenticateRequest,
    ) -> Result<AuthenticateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(authenticate)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_token(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithTokenRequest,
    ) -> Result<LoginWithTokenResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login_with_token)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_token_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithTokenExRequest,
    ) -> Result<LoginWithTokenExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login_with_token_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginRequest,
    ) -> Result<LoginResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_sub_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithSubAccountRequest,
    ) -> Result<LoginWithSubAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login_with_sub_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn register(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(register)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn register_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterExRequest,
    ) -> Result<RegisterExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(register_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_token_cafe(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithTokenCafeRequest,
    ) -> Result<LoginWithTokenCafeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login_with_token_cafe)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn login_with_token_cafe_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LoginWithTokenCafeExRequest,
    ) -> Result<LoginWithTokenCafeExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "SimpleAuthenticationProtocol",
            stringify!(login_with_token_cafe_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
