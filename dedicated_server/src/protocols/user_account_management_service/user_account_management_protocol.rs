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
enum UserAccountManagementProtocolMethod {
    LookupSceNpIds = 1u32,
    LookupPrincipalIDs = 2u32,
    LookupFirstPartyIds = 3u32,
    UserHasPlayed = 4u32,
    IsUserPlaying = 5u32,
    UpdateSonyAccountInfo = 6u32,
    LookupUsernames = 7u32,
}
#[derive(Debug, FromStream)]
pub struct LookupSceNpIdsRequest {
    pub pids: Vec<u32>,
}
#[derive(Debug, ToStream)]
pub struct LookupSceNpIdsResponse {
    pub npids: std::collections::HashMap<u32, quazal::rmc::types::QBuffer>,
}
#[derive(Debug, FromStream)]
pub struct LookupPrincipalIDsRequest {
    pub first_party_ids: Vec<String>,
    pub platform_id: u32,
}
#[derive(Debug, ToStream)]
pub struct LookupPrincipalIDsResponse {
    pub pids: std::collections::HashMap<String, u32>,
}
#[derive(Debug, FromStream)]
pub struct LookupFirstPartyIdsRequest {
    pub pids: Vec<u32>,
    pub platform_id: u32,
}
#[derive(Debug, ToStream)]
pub struct LookupFirstPartyIdsResponse {
    pub first_party_ids: std::collections::HashMap<u32, String>,
}
#[derive(Debug, FromStream)]
pub struct UserHasPlayedRequest {
    pub first_party_ids: Vec<String>,
    pub platform_id: u32,
}
#[derive(Debug, ToStream)]
pub struct UserHasPlayedResponse {
    pub user_presence: std::collections::HashMap<String, bool>,
}
#[derive(Debug, FromStream)]
pub struct IsUserPlayingRequest {
    pub first_party_ids: Vec<String>,
    pub platform_id: u32,
}
#[derive(Debug, ToStream)]
pub struct IsUserPlayingResponse {
    pub user_presence: std::collections::HashMap<String, bool>,
}
#[derive(Debug, FromStream)]
pub struct UpdateSonyAccountInfoRequest {
    pub ticket_data: quazal::rmc::types::QBuffer,
    pub ticket_size: u32,
}
#[derive(Debug, ToStream)]
pub struct UpdateSonyAccountInfoResponse;
#[derive(Debug, FromStream)]
pub struct LookupUsernamesRequest {
    pub pids: Vec<u32>,
    pub platform_id: u32,
}
#[derive(Debug, ToStream)]
pub struct LookupUsernamesResponse {
    pub user_names: std::collections::HashMap<u32, String>,
}
pub struct UserAccountManagementProtocol<T: UserAccountManagementProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: UserAccountManagementProtocolTrait<CI>, CI> UserAccountManagementProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: UserAccountManagementProtocolTrait<CI>, CI> Protocol<CI>
    for UserAccountManagementProtocol<T, CI>
{
    fn id(&self) -> u16 {
        todo!()
    }
    fn name(&self) -> String {
        "UserAccountManagementProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        7u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = UserAccountManagementProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(UserAccountManagementProtocolMethod::LookupSceNpIds) => {
                let req = LookupSceNpIdsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_sce_np_ids(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::LookupPrincipalIDs) => {
                let req = LookupPrincipalIDsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_principal_i_ds(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::LookupFirstPartyIds) => {
                let req = LookupFirstPartyIdsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_first_party_ids(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::UserHasPlayed) => {
                let req = UserHasPlayedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.user_has_played(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::IsUserPlaying) => {
                let req = IsUserPlayingRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.is_user_playing(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::UpdateSonyAccountInfo) => {
                let req = UpdateSonyAccountInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_sony_account_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserAccountManagementProtocolMethod::LookupUsernames) => {
                let req = LookupUsernamesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_usernames(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        UserAccountManagementProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait UserAccountManagementProtocolTrait<CI> {
    fn lookup_sce_np_ids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupSceNpIdsRequest,
    ) -> Result<LookupSceNpIdsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserAccountManagementProtocol",
            stringify!(lookup_sce_np_ids)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_principal_i_ds(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupPrincipalIDsRequest,
    ) -> Result<LookupPrincipalIDsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserAccountManagementProtocol",
            stringify!(lookup_principal_i_ds)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_first_party_ids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupFirstPartyIdsRequest,
    ) -> Result<LookupFirstPartyIdsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserAccountManagementProtocol",
            stringify!(lookup_first_party_ids)
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
            "UserAccountManagementProtocol",
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
            "UserAccountManagementProtocol",
            stringify!(is_user_playing)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_sony_account_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateSonyAccountInfoRequest,
    ) -> Result<UpdateSonyAccountInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserAccountManagementProtocol",
            stringify!(update_sony_account_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_usernames(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupUsernamesRequest,
    ) -> Result<LookupUsernamesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserAccountManagementProtocol",
            stringify!(lookup_usernames)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
