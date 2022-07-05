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
enum UserStorageAdminProtocolMethod {
    GetContentsToModerate = 1u32,
    FlagContentAsVerified = 2u32,
    BanContent = 3u32,
    BanUser = 4u32,
    BanUserFromContentType = 5u32,
    UnbanUser = 6u32,
    UnbanUserFromContentType = 7u32,
    GetContentsToModerateWithThreshold = 8u32,
    UpdateMetaData = 9u32,
    UpdateContentDb = 10u32,
    UpdateContentAndGetUploadInfo = 11u32,
    DeleteContent = 12u32,
    BrowseContents = 13u32,
    IsUserbanned = 14u32,
    GetBannedUsers = 15u32,
}
#[derive(Debug, FromStream)]
pub struct GetContentsToModerateRequest {
    pub type_id: u32,
    pub offset: u32,
    pub size: u32,
}
#[derive(Debug, ToStream)]
pub struct GetContentsToModerateResponse {
    pub contents: quazal::rmc::types::QList<UserContent>,
    pub total_results: u32,
}
#[derive(Debug, FromStream)]
pub struct FlagContentAsVerifiedRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct FlagContentAsVerifiedResponse;
#[derive(Debug, FromStream)]
pub struct BanContentRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct BanContentResponse;
#[derive(Debug, FromStream)]
pub struct BanUserRequest {
    pub pid: u32,
    pub reason: String,
    pub ban_contents: bool,
    pub expire_date: quazal::rmc::types::DateTime,
}
#[derive(Debug, ToStream)]
pub struct BanUserResponse;
#[derive(Debug, FromStream)]
pub struct BanUserFromContentTypeRequest {
    pub type_id: u32,
    pub pid: u32,
    pub reason: String,
    pub ban_contents: bool,
    pub expire_date: quazal::rmc::types::DateTime,
}
#[derive(Debug, ToStream)]
pub struct BanUserFromContentTypeResponse;
#[derive(Debug, FromStream)]
pub struct UnbanUserRequest {
    pub pid: u32,
}
#[derive(Debug, ToStream)]
pub struct UnbanUserResponse;
#[derive(Debug, FromStream)]
pub struct UnbanUserFromContentTypeRequest {
    pub type_id: u32,
    pub pid: u32,
}
#[derive(Debug, ToStream)]
pub struct UnbanUserFromContentTypeResponse;
#[derive(Debug, FromStream)]
pub struct GetContentsToModerateWithThresholdRequest {
    pub type_id: u32,
    pub threshold: u32,
    pub offset: u32,
    pub size: u32,
}
#[derive(Debug, ToStream)]
pub struct GetContentsToModerateWithThresholdResponse {
    pub contents: quazal::rmc::types::QList<UserContent>,
    pub total_results: u32,
}
#[derive(Debug, FromStream)]
pub struct UpdateMetaDataRequest {
    pub content_key: UserContentKey,
    pub properties: quazal::rmc::types::QList<ContentProperty>,
}
#[derive(Debug, ToStream)]
pub struct UpdateMetaDataResponse;
#[derive(Debug, FromStream)]
pub struct UpdateContentDbRequest {
    pub content_key: UserContentKey,
    pub properties: quazal::rmc::types::QList<ContentProperty>,
    pub data: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateContentDbResponse;
#[derive(Debug, FromStream)]
pub struct UpdateContentAndGetUploadInfoRequest {
    pub content_key: UserContentKey,
    pub properties: quazal::rmc::types::QList<ContentProperty>,
    pub size: u32,
}
#[derive(Debug, ToStream)]
pub struct UpdateContentAndGetUploadInfoResponse {
    pub upload_info: UserContentURL,
    pub pending_id: u64,
    pub headers: Vec<String>,
}
#[derive(Debug, FromStream)]
pub struct DeleteContentRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct DeleteContentResponse;
#[derive(Debug, FromStream)]
pub struct BrowseContentsRequest {
    pub type_id: u32,
    pub offset: u32,
    pub size: u32,
}
#[derive(Debug, ToStream)]
pub struct BrowseContentsResponse {
    pub contents: quazal::rmc::types::QList<AdminContent>,
    pub total_results: u32,
}
#[derive(Debug, FromStream)]
pub struct IsUserbannedRequest {
    pub type_id: u32,
    pub pid: u32,
}
#[derive(Debug, ToStream)]
pub struct IsUserbannedResponse {
    pub banned: bool,
    pub reason: String,
}
#[derive(Debug, FromStream)]
pub struct GetBannedUsersRequest {
    pub type_id: u32,
    pub offset: u32,
    pub size: u32,
}
#[derive(Debug, ToStream)]
pub struct GetBannedUsersResponse {
    pub banned_users: quazal::rmc::types::QList<BannedUser>,
    pub total_banned_users: u32,
}
pub struct UserStorageAdminProtocol<T: UserStorageAdminProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: UserStorageAdminProtocolTrait<CI>, CI> UserStorageAdminProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: UserStorageAdminProtocolTrait<CI>, CI> Protocol<CI> for UserStorageAdminProtocol<T, CI> {
    fn id(&self) -> u16 {
        todo!()
    }
    fn name(&self) -> String {
        "UserStorageAdminProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        15u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = UserStorageAdminProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(UserStorageAdminProtocolMethod::GetContentsToModerate) => {
                let req = GetContentsToModerateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_contents_to_moderate(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::FlagContentAsVerified) => {
                let req = FlagContentAsVerifiedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.flag_content_as_verified(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::BanContent) => {
                let req = BanContentRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ban_content(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::BanUser) => {
                let req = BanUserRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ban_user(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::BanUserFromContentType) => {
                let req = BanUserFromContentTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.ban_user_from_content_type(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::UnbanUser) => {
                let req = UnbanUserRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.unban_user(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::UnbanUserFromContentType) => {
                let req = UnbanUserFromContentTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.unban_user_from_content_type(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::GetContentsToModerateWithThreshold) => {
                let req =
                    GetContentsToModerateWithThresholdRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_contents_to_moderate_with_threshold(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::UpdateMetaData) => {
                let req = UpdateMetaDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_meta_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::UpdateContentDb) => {
                let req = UpdateContentDbRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_content_db(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::UpdateContentAndGetUploadInfo) => {
                let req = UpdateContentAndGetUploadInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .update_content_and_get_upload_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::DeleteContent) => {
                let req = DeleteContentRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.delete_content(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::BrowseContents) => {
                let req = BrowseContentsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.browse_contents(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::IsUserbanned) => {
                let req = IsUserbannedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.is_userbanned(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageAdminProtocolMethod::GetBannedUsers) => {
                let req = GetBannedUsersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_banned_users(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        UserStorageAdminProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait UserStorageAdminProtocolTrait<CI> {
    fn get_contents_to_moderate(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetContentsToModerateRequest,
    ) -> Result<GetContentsToModerateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(get_contents_to_moderate)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn flag_content_as_verified(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: FlagContentAsVerifiedRequest,
    ) -> Result<FlagContentAsVerifiedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(flag_content_as_verified)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn ban_content(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BanContentRequest,
    ) -> Result<BanContentResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(ban_content)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn ban_user(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BanUserRequest,
    ) -> Result<BanUserResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(ban_user)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn ban_user_from_content_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BanUserFromContentTypeRequest,
    ) -> Result<BanUserFromContentTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(ban_user_from_content_type)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unban_user(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnbanUserRequest,
    ) -> Result<UnbanUserResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(unban_user)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unban_user_from_content_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnbanUserFromContentTypeRequest,
    ) -> Result<UnbanUserFromContentTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(unban_user_from_content_type)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_contents_to_moderate_with_threshold(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetContentsToModerateWithThresholdRequest,
    ) -> Result<GetContentsToModerateWithThresholdResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(get_contents_to_moderate_with_threshold)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_meta_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateMetaDataRequest,
    ) -> Result<UpdateMetaDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(update_meta_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_content_db(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateContentDbRequest,
    ) -> Result<UpdateContentDbResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(update_content_db)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_content_and_get_upload_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateContentAndGetUploadInfoRequest,
    ) -> Result<UpdateContentAndGetUploadInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(update_content_and_get_upload_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn delete_content(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteContentRequest,
    ) -> Result<DeleteContentResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(delete_content)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn browse_contents(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BrowseContentsRequest,
    ) -> Result<BrowseContentsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(browse_contents)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn is_userbanned(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: IsUserbannedRequest,
    ) -> Result<IsUserbannedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(is_userbanned)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_banned_users(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetBannedUsersRequest,
    ) -> Result<GetBannedUsersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageAdminProtocol",
            stringify!(get_banned_users)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
