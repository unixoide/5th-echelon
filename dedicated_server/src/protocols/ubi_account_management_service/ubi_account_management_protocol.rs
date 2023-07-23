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
enum UbiAccountManagementProtocolMethod {
    CreateAccount = 1u32,
    UpdateAccount = 2u32,
    GetAccount = 3u32,
    LinkAccount = 4u32,
    GetTos = 5u32,
    ValidateUsername = 6u32,
    ValidatePassword = 7u32,
    ValidateEmail = 8u32,
    GetCountryList = 9u32,
    ForgetPassword = 10u32,
    LookupPrincipalIds = 11u32,
    LookupUbiAccountIDsByPids = 12u32,
    LookupUbiAccountIDsByUsernames = 13u32,
    LookupUsernamesByUbiAccountIDs = 14u32,
    LookupUbiAccountIDsByUsernameSubString = 15u32,
    UserHasPlayed = 16u32,
    IsUserPlaying = 17u32,
    LookupUbiAccountIDsByUsernamesGlobal = 18u32,
    LookupUbiAccountIDsByEmailsGlobal = 19u32,
    LookupUsernamesByUbiAccountIDsGlobal = 20u32,
    GetTosEx = 21u32,
    HasAcceptedLatestTos = 22u32,
    AcceptLatestTos = 23u32,
}
#[derive(Debug, FromStream)]
pub struct CreateAccountRequest;
#[derive(Debug, ToStream)]
pub struct CreateAccountResponse {
    pub ubi_account: UbiAccount,
    pub failed_reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, FromStream)]
pub struct UpdateAccountRequest;
#[derive(Debug, ToStream)]
pub struct UpdateAccountResponse {
    pub ubi_account: UbiAccount,
    pub failed_reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, FromStream)]
pub struct GetAccountRequest;
#[derive(Debug, ToStream)]
pub struct GetAccountResponse {
    pub ubi_account: UbiAccount,
    pub exists: bool,
}
#[derive(Debug, FromStream)]
pub struct LinkAccountRequest {
    pub ubi_account_username: String,
    pub ubi_account_password: String,
}
#[derive(Debug, ToStream)]
pub struct LinkAccountResponse;
#[derive(Debug, FromStream)]
pub struct GetTosRequest {
    pub country_code: String,
    pub language_code: String,
    pub html_version: bool,
}
#[derive(Debug, ToStream)]
pub struct GetTosResponse {
    pub tos: TOS,
}
#[derive(Debug, FromStream)]
pub struct ValidateUsernameRequest {
    pub username: String,
}
#[derive(Debug, ToStream)]
pub struct ValidateUsernameResponse {
    pub username_validation: UsernameValidation,
}
#[derive(Debug, FromStream)]
pub struct ValidatePasswordRequest {
    pub password: String,
    pub username: String,
}
#[derive(Debug, ToStream)]
pub struct ValidatePasswordResponse {
    pub failed_reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, FromStream)]
pub struct ValidateEmailRequest {
    pub email: String,
}
#[derive(Debug, ToStream)]
pub struct ValidateEmailResponse {
    pub failed_reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, FromStream)]
pub struct GetCountryListRequest {
    pub language_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetCountryListResponse {
    pub countries: Vec<Country>,
}
#[derive(Debug, FromStream)]
pub struct ForgetPasswordRequest {
    pub username_or_email: String,
}
#[derive(Debug, ToStream)]
pub struct ForgetPasswordResponse;
#[derive(Debug, FromStream)]
pub struct LookupPrincipalIdsRequest {
    pub ubi_account_ids: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupPrincipalIdsResponse {
    pub pids: std::collections::HashMap<String, u32>,
}
#[derive(Debug, FromStream)]
pub struct LookupUbiAccountIDsByPidsRequest {
    pub pids: Vec<u32>,
}
#[derive(Debug, ToStream)]
pub struct LookupUbiAccountIDsByPidsResponse {
    pub ubiaccount_i_ds: std::collections::HashMap<u32, String>,
}
#[derive(Debug, FromStream)]
pub struct LookupUbiAccountIDsByUsernamesRequest {
    pub usernames: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupUbiAccountIDsByUsernamesResponse {
    pub ubi_account_i_ds: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct LookupUsernamesByUbiAccountIDsRequest {
    pub ubi_account_ids: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupUsernamesByUbiAccountIDsResponse {
    pub usernames: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct LookupUbiAccountIDsByUsernameSubStringRequest {
    pub username_sub_string: String,
}
#[derive(Debug, ToStream)]
pub struct LookupUbiAccountIDsByUsernameSubStringResponse {
    pub ubi_account_i_ds: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct UserHasPlayedRequest {
    pub ubi_account_i_ds: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct UserHasPlayedResponse {
    pub user_presence: std::collections::HashMap<String, bool>,
}
#[derive(Debug, FromStream)]
pub struct IsUserPlayingRequest {
    pub ubi_account_i_ds: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct IsUserPlayingResponse {
    pub user_presence: std::collections::HashMap<String, bool>,
}
#[derive(Debug, FromStream)]
pub struct LookupUbiAccountIDsByUsernamesGlobalRequest {
    pub usernames: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupUbiAccountIDsByUsernamesGlobalResponse {
    pub ubi_account_i_ds: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct LookupUbiAccountIDsByEmailsGlobalRequest {
    pub emails: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupUbiAccountIDsByEmailsGlobalResponse {
    pub ubi_account_i_ds: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct LookupUsernamesByUbiAccountIDsGlobalRequest {
    pub ubi_account_ids: Vec<String>,
}
#[derive(Debug, ToStream)]
pub struct LookupUsernamesByUbiAccountIDsGlobalResponse {
    pub usernames: std::collections::HashMap<String, String>,
}
#[derive(Debug, FromStream)]
pub struct GetTosExRequest {
    pub country_code: String,
    pub language_code: String,
    pub html_version: bool,
}
#[derive(Debug, ToStream)]
pub struct GetTosExResponse {
    pub tosex: TOSEx,
}
#[derive(Debug, FromStream)]
pub struct HasAcceptedLatestTosRequest;
#[derive(Debug, ToStream)]
pub struct HasAcceptedLatestTosResponse {
    pub has_accepted: bool,
    pub failed_reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, FromStream)]
pub struct AcceptLatestTosRequest;
#[derive(Debug, ToStream)]
pub struct AcceptLatestTosResponse {
    pub failed_reasons: Vec<ValidationFailureReason>,
}
pub struct UbiAccountManagementProtocol<T: UbiAccountManagementProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: UbiAccountManagementProtocolTrait<CI>, CI> UbiAccountManagementProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: UbiAccountManagementProtocolTrait<CI>, CI> Protocol<CI>
    for UbiAccountManagementProtocol<T, CI>
{
    fn id(&self) -> u16 {
        29u16
    }
    fn name(&self) -> String {
        "UbiAccountManagementProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        23u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = UbiAccountManagementProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(UbiAccountManagementProtocolMethod::CreateAccount) => {
                let req = CreateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.create_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::UpdateAccount) => {
                let req = UpdateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::GetAccount) => {
                let req = GetAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LinkAccount) => {
                let req = LinkAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.link_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::GetTos) => {
                let req = GetTosRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_tos(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::ValidateUsername) => {
                let req = ValidateUsernameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.validate_username(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::ValidatePassword) => {
                let req = ValidatePasswordRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.validate_password(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::ValidateEmail) => {
                let req = ValidateEmailRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.validate_email(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::GetCountryList) => {
                let req = GetCountryListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_country_list(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::ForgetPassword) => {
                let req = ForgetPasswordRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.forget_password(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupPrincipalIds) => {
                let req = LookupPrincipalIdsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_principal_ids(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUbiAccountIDsByPids) => {
                let req = LookupUbiAccountIDsByPidsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_ubi_account_i_ds_by_pids(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUbiAccountIDsByUsernames) => {
                let req = LookupUbiAccountIDsByUsernamesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_ubi_account_i_ds_by_usernames(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUsernamesByUbiAccountIDs) => {
                let req = LookupUsernamesByUbiAccountIDsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_usernames_by_ubi_account_i_ds(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUbiAccountIDsByUsernameSubString) => {
                let req =
                    LookupUbiAccountIDsByUsernameSubStringRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_ubi_account_i_ds_by_username_sub_string(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::UserHasPlayed) => {
                let req = UserHasPlayedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.user_has_played(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::IsUserPlaying) => {
                let req = IsUserPlayingRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.is_user_playing(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUbiAccountIDsByUsernamesGlobal) => {
                let req =
                    LookupUbiAccountIDsByUsernamesGlobalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_ubi_account_i_ds_by_usernames_global(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUbiAccountIDsByEmailsGlobal) => {
                let req =
                    LookupUbiAccountIDsByEmailsGlobalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_ubi_account_i_ds_by_emails_global(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::LookupUsernamesByUbiAccountIDsGlobal) => {
                let req =
                    LookupUsernamesByUbiAccountIDsGlobalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .lookup_usernames_by_ubi_account_i_ds_global(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::GetTosEx) => {
                let req = GetTosExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_tos_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::HasAcceptedLatestTos) => {
                let req = HasAcceptedLatestTosRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.has_accepted_latest_tos(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UbiAccountManagementProtocolMethod::AcceptLatestTos) => {
                let req = AcceptLatestTosRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.accept_latest_tos(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        UbiAccountManagementProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait UbiAccountManagementProtocolTrait<CI> {
    fn create_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(create_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountRequest,
    ) -> Result<UpdateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(update_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetAccountRequest,
    ) -> Result<GetAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(get_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn link_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LinkAccountRequest,
    ) -> Result<LinkAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(link_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_tos(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetTosRequest,
    ) -> Result<GetTosResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(get_tos)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn validate_username(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ValidateUsernameRequest,
    ) -> Result<ValidateUsernameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(validate_username)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn validate_password(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ValidatePasswordRequest,
    ) -> Result<ValidatePasswordResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(validate_password)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn validate_email(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ValidateEmailRequest,
    ) -> Result<ValidateEmailResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(validate_email)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_country_list(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetCountryListRequest,
    ) -> Result<GetCountryListResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(get_country_list)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn forget_password(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ForgetPasswordRequest,
    ) -> Result<ForgetPasswordResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(forget_password)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_principal_ids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupPrincipalIdsRequest,
    ) -> Result<LookupPrincipalIdsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_principal_ids)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_ubi_account_i_ds_by_pids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUbiAccountIDsByPidsRequest,
    ) -> Result<LookupUbiAccountIDsByPidsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_ubi_account_i_ds_by_pids)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_ubi_account_i_ds_by_usernames(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUbiAccountIDsByUsernamesRequest,
    ) -> Result<LookupUbiAccountIDsByUsernamesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_ubi_account_i_ds_by_usernames)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_usernames_by_ubi_account_i_ds(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUsernamesByUbiAccountIDsRequest,
    ) -> Result<LookupUsernamesByUbiAccountIDsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_usernames_by_ubi_account_i_ds)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_ubi_account_i_ds_by_username_sub_string(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUbiAccountIDsByUsernameSubStringRequest,
    ) -> Result<LookupUbiAccountIDsByUsernameSubStringResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_ubi_account_i_ds_by_username_sub_string)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn user_has_played(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UserHasPlayedRequest,
    ) -> Result<UserHasPlayedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(user_has_played)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn is_user_playing(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: IsUserPlayingRequest,
    ) -> Result<IsUserPlayingResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(is_user_playing)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_ubi_account_i_ds_by_usernames_global(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUbiAccountIDsByUsernamesGlobalRequest,
    ) -> Result<LookupUbiAccountIDsByUsernamesGlobalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_ubi_account_i_ds_by_usernames_global)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_ubi_account_i_ds_by_emails_global(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUbiAccountIDsByEmailsGlobalRequest,
    ) -> Result<LookupUbiAccountIDsByEmailsGlobalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_ubi_account_i_ds_by_emails_global)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_usernames_by_ubi_account_i_ds_global(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUsernamesByUbiAccountIDsGlobalRequest,
    ) -> Result<LookupUsernamesByUbiAccountIDsGlobalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(lookup_usernames_by_ubi_account_i_ds_global)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_tos_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetTosExRequest,
    ) -> Result<GetTosExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(get_tos_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn has_accepted_latest_tos(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: HasAcceptedLatestTosRequest,
    ) -> Result<HasAcceptedLatestTosResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(has_accepted_latest_tos)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn accept_latest_tos(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AcceptLatestTosRequest,
    ) -> Result<AcceptLatestTosResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UbiAccountManagementProtocol",
            stringify!(accept_latest_tos)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
