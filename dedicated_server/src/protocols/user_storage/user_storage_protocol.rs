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
enum UserStorageProtocolMethod {
    SearchContents = 1u32,
    SearchContentsWithTotal = 2u32,
    DeleteContent = 3u32,
    SaveMetaData = 4u32,
    SaveContentDb = 5u32,
    SaveContentAndGetUploadInfo = 6u32,
    UploadEnd = 7u32,
    GetContentDb = 8u32,
    GetContentUrl = 9u32,
    GetSlotCount = 10u32,
    GetMetaData = 11u32,
    Like = 12u32,
    Unlike = 13u32,
    IsLiked = 14u32,
    GetFavourites = 15u32,
    MakeFavourite = 16u32,
    RemoveFromFavourites = 17u32,
    ReportInappropriate = 18u32,
    IncrementPlayCount = 19u32,
    UpdateCustomStat = 20u32,
    GetOwnContents = 21u32,
    GetMostPopularTags = 22u32,
    GetTags = 23u32,
    TagContent = 24u32,
    SearchContentsByPlayers = 25u32,
    SearchContentsByPlayersWithTotal = 26u32,
}
#[derive(Debug, FromStream)]
pub struct SearchContentsRequest {
    pub query: UserStorageQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchContentsResponse {
    pub search_results: quazal::rmc::types::QList<UserContent>,
}
#[derive(Debug, FromStream)]
pub struct SearchContentsWithTotalRequest {
    pub query: UserStorageQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchContentsWithTotalResponse {
    pub search_results: quazal::rmc::types::QList<UserContent>,
    pub total_results: u32,
}
#[derive(Debug, FromStream)]
pub struct DeleteContentRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct DeleteContentResponse;
#[derive(Debug, FromStream)]
pub struct SaveMetaDataRequest {
    pub properties: quazal::rmc::types::QList<ContentProperty>,
}
#[derive(Debug, ToStream)]
pub struct SaveMetaDataResponse {
    pub content_key: UserContentKey,
}
#[derive(Debug, FromStream)]
pub struct SaveContentDbRequest {
    pub properties: quazal::rmc::types::QList<ContentProperty>,
    pub data: Vec<u8>,
}
#[derive(Debug, ToStream)]
pub struct SaveContentDbResponse {
    pub content_key: UserContentKey,
}
#[derive(Debug, FromStream)]
pub struct SaveContentAndGetUploadInfoRequest {
    pub properties: quazal::rmc::types::QList<ContentProperty>,
    pub size: u32,
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct SaveContentAndGetUploadInfoResponse {
    pub upload_info: UserContentURL,
    pub pending_id: u64,
    pub headers: Vec<String>,
}
#[derive(Debug, FromStream)]
pub struct UploadEndRequest {
    pub pending_id: u64,
    pub result: bool,
}
#[derive(Debug, ToStream)]
pub struct UploadEndResponse {
    pub content_key: UserContentKey,
}
#[derive(Debug, FromStream)]
pub struct GetContentDbRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct GetContentDbResponse {
    pub data: Vec<u8>,
}
#[derive(Debug, FromStream)]
pub struct GetContentUrlRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct GetContentUrlResponse {
    pub download_info: UserContentURL,
}
#[derive(Debug, FromStream)]
pub struct GetSlotCountRequest {
    pub type_id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetSlotCountResponse {
    pub slot_count: UserSlotCount,
}
#[derive(Debug, FromStream)]
pub struct GetMetaDataRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct GetMetaDataResponse {
    pub content: UserContent,
}
#[derive(Debug, FromStream)]
pub struct LikeRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct LikeResponse;
#[derive(Debug, FromStream)]
pub struct UnlikeRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct UnlikeResponse;
#[derive(Debug, FromStream)]
pub struct IsLikedRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct IsLikedResponse {
    pub liked: bool,
}
#[derive(Debug, FromStream)]
pub struct GetFavouritesRequest {
    pub content_types: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GetFavouritesResponse {
    pub favourites: quazal::rmc::types::QList<UserContent>,
}
#[derive(Debug, FromStream)]
pub struct MakeFavouriteRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct MakeFavouriteResponse;
#[derive(Debug, FromStream)]
pub struct RemoveFromFavouritesRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct RemoveFromFavouritesResponse;
#[derive(Debug, FromStream)]
pub struct ReportInappropriateRequest {
    pub content_key: UserContentKey,
    pub reason: String,
}
#[derive(Debug, ToStream)]
pub struct ReportInappropriateResponse;
#[derive(Debug, FromStream)]
pub struct IncrementPlayCountRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct IncrementPlayCountResponse;
#[derive(Debug, FromStream)]
pub struct UpdateCustomStatRequest {
    pub content_key: UserContentKey,
    pub stat_id: u16,
    pub inc_value: i64,
}
#[derive(Debug, ToStream)]
pub struct UpdateCustomStatResponse;
#[derive(Debug, FromStream)]
pub struct GetOwnContentsRequest {
    pub type_id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetOwnContentsResponse {
    pub results: quazal::rmc::types::QList<UserContent>,
}
#[derive(Debug, FromStream)]
pub struct GetMostPopularTagsRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct GetMostPopularTagsResponse {
    pub tags: quazal::rmc::types::QList<WeightedTag>,
    pub total_number_of_taggings: u32,
}
#[derive(Debug, FromStream)]
pub struct GetTagsRequest {
    pub content_key: UserContentKey,
}
#[derive(Debug, ToStream)]
pub struct GetTagsResponse {
    pub tag_ids: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, FromStream)]
pub struct TagContentRequest {
    pub content_key: UserContentKey,
    pub new_tag_ids: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct TagContentResponse;
#[derive(Debug, FromStream)]
pub struct SearchContentsByPlayersRequest {
    pub pids: quazal::rmc::types::QList<u32>,
    pub query: UserStorageQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchContentsByPlayersResponse {
    pub search_results: quazal::rmc::types::QList<UserContent>,
}
#[derive(Debug, FromStream)]
pub struct SearchContentsByPlayersWithTotalRequest {
    pub pids: quazal::rmc::types::QList<u32>,
    pub query: UserStorageQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchContentsByPlayersWithTotalResponse {
    pub search_results: quazal::rmc::types::QList<UserContent>,
    pub total_results: u32,
}
pub struct UserStorageProtocol<T: UserStorageProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: UserStorageProtocolTrait<CI>, CI> UserStorageProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: UserStorageProtocolTrait<CI>, CI> Protocol<CI> for UserStorageProtocol<T, CI> {
    fn id(&self) -> u16 {
        53u16
    }
    fn name(&self) -> String {
        "UserStorageProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        26u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = UserStorageProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(UserStorageProtocolMethod::SearchContents) => {
                let req = SearchContentsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_contents(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SearchContentsWithTotal) => {
                let req = SearchContentsWithTotalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_contents_with_total(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::DeleteContent) => {
                let req = DeleteContentRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.delete_content(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SaveMetaData) => {
                let req = SaveMetaDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.save_meta_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SaveContentDb) => {
                let req = SaveContentDbRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.save_content_db(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SaveContentAndGetUploadInfo) => {
                let req = SaveContentAndGetUploadInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .save_content_and_get_upload_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::UploadEnd) => {
                let req = UploadEndRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.upload_end(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetContentDb) => {
                let req = GetContentDbRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_content_db(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetContentUrl) => {
                let req = GetContentUrlRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_content_url(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetSlotCount) => {
                let req = GetSlotCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_slot_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetMetaData) => {
                let req = GetMetaDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_meta_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::Like) => {
                let req = LikeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.like(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::Unlike) => {
                let req = UnlikeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.unlike(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::IsLiked) => {
                let req = IsLikedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.is_liked(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetFavourites) => {
                let req = GetFavouritesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_favourites(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::MakeFavourite) => {
                let req = MakeFavouriteRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.make_favourite(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::RemoveFromFavourites) => {
                let req = RemoveFromFavouritesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.remove_from_favourites(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::ReportInappropriate) => {
                let req = ReportInappropriateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.report_inappropriate(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::IncrementPlayCount) => {
                let req = IncrementPlayCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.increment_play_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::UpdateCustomStat) => {
                let req = UpdateCustomStatRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_custom_stat(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetOwnContents) => {
                let req = GetOwnContentsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_own_contents(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetMostPopularTags) => {
                let req = GetMostPopularTagsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_most_popular_tags(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::GetTags) => {
                let req = GetTagsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_tags(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::TagContent) => {
                let req = TagContentRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.tag_content(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SearchContentsByPlayers) => {
                let req = SearchContentsByPlayersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_contents_by_players(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UserStorageProtocolMethod::SearchContentsByPlayersWithTotal) => {
                let req = SearchContentsByPlayersWithTotalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .search_contents_by_players_with_total(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        UserStorageProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait UserStorageProtocolTrait<CI> {
    fn search_contents(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchContentsRequest,
    ) -> Result<SearchContentsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(search_contents)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_contents_with_total(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchContentsWithTotalRequest,
    ) -> Result<SearchContentsWithTotalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(search_contents_with_total)
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
            "UserStorageProtocol",
            stringify!(delete_content)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn save_meta_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SaveMetaDataRequest,
    ) -> Result<SaveMetaDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(save_meta_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn save_content_db(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SaveContentDbRequest,
    ) -> Result<SaveContentDbResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(save_content_db)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn save_content_and_get_upload_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SaveContentAndGetUploadInfoRequest,
    ) -> Result<SaveContentAndGetUploadInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(save_content_and_get_upload_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn upload_end(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UploadEndRequest,
    ) -> Result<UploadEndResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(upload_end)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_content_db(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetContentDbRequest,
    ) -> Result<GetContentDbResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_content_db)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_content_url(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetContentUrlRequest,
    ) -> Result<GetContentUrlResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_content_url)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_slot_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSlotCountRequest,
    ) -> Result<GetSlotCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_slot_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_meta_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetMetaDataRequest,
    ) -> Result<GetMetaDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_meta_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn like(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LikeRequest,
    ) -> Result<LikeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(like)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unlike(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnlikeRequest,
    ) -> Result<UnlikeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(unlike)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn is_liked(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: IsLikedRequest,
    ) -> Result<IsLikedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(is_liked)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_favourites(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetFavouritesRequest,
    ) -> Result<GetFavouritesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_favourites)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn make_favourite(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: MakeFavouriteRequest,
    ) -> Result<MakeFavouriteResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(make_favourite)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn remove_from_favourites(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RemoveFromFavouritesRequest,
    ) -> Result<RemoveFromFavouritesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(remove_from_favourites)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn report_inappropriate(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReportInappropriateRequest,
    ) -> Result<ReportInappropriateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(report_inappropriate)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn increment_play_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: IncrementPlayCountRequest,
    ) -> Result<IncrementPlayCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(increment_play_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_custom_stat(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateCustomStatRequest,
    ) -> Result<UpdateCustomStatResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(update_custom_stat)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_own_contents(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetOwnContentsRequest,
    ) -> Result<GetOwnContentsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_own_contents)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_most_popular_tags(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetMostPopularTagsRequest,
    ) -> Result<GetMostPopularTagsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_most_popular_tags)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_tags(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetTagsRequest,
    ) -> Result<GetTagsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(get_tags)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn tag_content(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: TagContentRequest,
    ) -> Result<TagContentResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(tag_content)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_contents_by_players(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchContentsByPlayersRequest,
    ) -> Result<SearchContentsByPlayersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(search_contents_by_players)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_contents_by_players_with_total(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchContentsByPlayersWithTotalRequest,
    ) -> Result<SearchContentsByPlayersWithTotalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UserStorageProtocol",
            stringify!(search_contents_by_players_with_total)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
