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
enum FriendsProtocolMethod {
    AddFriend = 1u32,
    AddFriendByName = 2u32,
    AddFriendWithDetails = 3u32,
    AddFriendByNameWithDetails = 4u32,
    AcceptFriendship = 5u32,
    DeclineFriendship = 6u32,
    BlackList = 7u32,
    BlackListByName = 8u32,
    ClearRelationship = 9u32,
    UpdateDetails = 10u32,
    GetList = 11u32,
    GetDetailedList = 12u32,
    GetRelationships = 13u32,
}
#[derive(Debug, FromStream)]
pub struct AddFriendRequest {
    pub ui_player: u32,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream)]
pub struct AddFriendResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct AddFriendByNameRequest {
    pub str_player_name: String,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream)]
pub struct AddFriendByNameResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct AddFriendWithDetailsRequest {
    pub ui_player: u32,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream)]
pub struct AddFriendWithDetailsResponse {
    pub relationship_data: RelationshipData,
}
#[derive(Debug, FromStream)]
pub struct AddFriendByNameWithDetailsRequest {
    pub str_player_name: String,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream)]
pub struct AddFriendByNameWithDetailsResponse {
    pub relationship_data: RelationshipData,
}
#[derive(Debug, FromStream)]
pub struct AcceptFriendshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream)]
pub struct AcceptFriendshipResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct DeclineFriendshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream)]
pub struct DeclineFriendshipResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct BlackListRequest {
    pub ui_player: u32,
    pub ui_details: u32,
}
#[derive(Debug, ToStream)]
pub struct BlackListResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct BlackListByNameRequest {
    pub str_player_name: String,
    pub ui_details: u32,
}
#[derive(Debug, ToStream)]
pub struct BlackListByNameResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct ClearRelationshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream)]
pub struct ClearRelationshipResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct UpdateDetailsRequest {
    pub ui_player: u32,
    pub ui_details: u32,
}
#[derive(Debug, ToStream)]
pub struct UpdateDetailsResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct GetListRequest {
    pub by_relationship: u8,
    pub b_reversed: bool,
}
#[derive(Debug, ToStream)]
pub struct GetListResponse {
    pub lst_friends_list: Vec<u32>,
}
#[derive(Debug, FromStream)]
pub struct GetDetailedListRequest {
    pub by_relationship: u8,
    pub b_reversed: bool,
}
#[derive(Debug, ToStream)]
pub struct GetDetailedListResponse {
    pub lst_friends_list: Vec<FriendData>,
}
#[derive(Debug, FromStream)]
pub struct GetRelationshipsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetRelationshipsResponse {
    pub ui_total_count: u32,
    pub lst_relationships_list: Vec<RelationshipData>,
}
pub struct FriendsProtocol<T: FriendsProtocolTrait<CI>, CI>(T, ::std::marker::PhantomData<CI>);
impl<T: FriendsProtocolTrait<CI>, CI> FriendsProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: FriendsProtocolTrait<CI>, CI> Protocol<CI> for FriendsProtocol<T, CI> {
    fn id(&self) -> u16 {
        20u16
    }
    fn name(&self) -> String {
        "FriendsProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        13u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = FriendsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(FriendsProtocolMethod::AddFriend) => {
                let req = AddFriendRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_friend(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendByName) => {
                let req = AddFriendByNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_friend_by_name(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendWithDetails) => {
                let req = AddFriendWithDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_friend_with_details(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendByNameWithDetails) => {
                let req = AddFriendByNameWithDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_friend_by_name_with_details(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::AcceptFriendship) => {
                let req = AcceptFriendshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.accept_friendship(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::DeclineFriendship) => {
                let req = DeclineFriendshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.decline_friendship(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::BlackList) => {
                let req = BlackListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.black_list(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::BlackListByName) => {
                let req = BlackListByNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.black_list_by_name(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::ClearRelationship) => {
                let req = ClearRelationshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.clear_relationship(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::UpdateDetails) => {
                let req = UpdateDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_details(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::GetList) => {
                let req = GetListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_list(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::GetDetailedList) => {
                let req = GetDetailedListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_detailed_list(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(FriendsProtocolMethod::GetRelationships) => {
                let req = GetRelationshipsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_relationships(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        FriendsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait FriendsProtocolTrait<CI> {
    fn add_friend(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddFriendRequest,
    ) -> Result<AddFriendResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(add_friend)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn add_friend_by_name(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddFriendByNameRequest,
    ) -> Result<AddFriendByNameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(add_friend_by_name)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn add_friend_with_details(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddFriendWithDetailsRequest,
    ) -> Result<AddFriendWithDetailsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(add_friend_with_details)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn add_friend_by_name_with_details(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddFriendByNameWithDetailsRequest,
    ) -> Result<AddFriendByNameWithDetailsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(add_friend_by_name_with_details)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn accept_friendship(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AcceptFriendshipRequest,
    ) -> Result<AcceptFriendshipResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(accept_friendship)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn decline_friendship(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeclineFriendshipRequest,
    ) -> Result<DeclineFriendshipResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(decline_friendship)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn black_list(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BlackListRequest,
    ) -> Result<BlackListResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(black_list)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn black_list_by_name(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BlackListByNameRequest,
    ) -> Result<BlackListByNameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(black_list_by_name)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn clear_relationship(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ClearRelationshipRequest,
    ) -> Result<ClearRelationshipResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(clear_relationship)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_details(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateDetailsRequest,
    ) -> Result<UpdateDetailsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(update_details)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_list(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetListRequest,
    ) -> Result<GetListResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(get_list)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_detailed_list(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetDetailedListRequest,
    ) -> Result<GetDetailedListResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(get_detailed_list)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_relationships(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetRelationshipsRequest,
    ) -> Result<GetRelationshipsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "FriendsProtocol",
            stringify!(get_relationships)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
