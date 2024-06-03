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

use super::types::*;
pub const FRIENDS_PROTOCOL_ID: u16 = 20u16;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum FriendsProtocolMethod {
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
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendRequest {
    pub ui_player: u32,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendByNameRequest {
    pub str_player_name: String,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendByNameResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendWithDetailsRequest {
    pub ui_player: u32,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendWithDetailsResponse {
    pub relationship_data: RelationshipData,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendByNameWithDetailsRequest {
    pub str_player_name: String,
    pub ui_details: u32,
    pub str_message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AddFriendByNameWithDetailsResponse {
    pub relationship_data: RelationshipData,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AcceptFriendshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct AcceptFriendshipResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct DeclineFriendshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct DeclineFriendshipResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BlackListRequest {
    pub ui_player: u32,
    pub ui_details: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BlackListResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BlackListByNameRequest {
    pub str_player_name: String,
    pub ui_details: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BlackListByNameResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ClearRelationshipRequest {
    pub ui_player: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ClearRelationshipResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UpdateDetailsRequest {
    pub ui_player: u32,
    pub ui_details: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UpdateDetailsResponse {
    pub return_value: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetListRequest {
    pub by_relationship: u8,
    pub b_reversed: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetListResponse {
    pub lst_friends_list: Vec<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetDetailedListRequest {
    pub by_relationship: u8,
    pub b_reversed: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetDetailedListResponse {
    pub lst_friends_list: Vec<FriendData>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetRelationshipsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetRelationshipsResponse {
    pub ui_total_count: u32,
    pub lst_relationships_list: Vec<RelationshipData>,
}
pub struct FriendsProtocolServer<T: FriendsProtocolServerTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: FriendsProtocolServerTrait<CI>, CI> FriendsProtocolServer<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: FriendsProtocolServerTrait<CI>, CI> Protocol<CI> for FriendsProtocolServer<T, CI> {
    fn id(&self) -> u16 {
        FRIENDS_PROTOCOL_ID
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
        client_registry: &ClientRegistry<CI>,
        socket: &std::net::UdpSocket,
    ) -> Result<Vec<u8>, Error> {
        let method = FriendsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(FriendsProtocolMethod::AddFriend) => {
                let req = AddFriendRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .add_friend(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendByName) => {
                let req = AddFriendByNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .add_friend_by_name(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendWithDetails) => {
                let req = AddFriendWithDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .add_friend_with_details(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::AddFriendByNameWithDetails) => {
                let req = AddFriendByNameWithDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_friend_by_name_with_details(
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
            Some(FriendsProtocolMethod::AcceptFriendship) => {
                let req = AcceptFriendshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .accept_friendship(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::DeclineFriendship) => {
                let req = DeclineFriendshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .decline_friendship(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::BlackList) => {
                let req = BlackListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .black_list(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::BlackListByName) => {
                let req = BlackListByNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .black_list_by_name(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::ClearRelationship) => {
                let req = ClearRelationshipRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .clear_relationship(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::UpdateDetails) => {
                let req = UpdateDetailsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .update_details(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::GetList) => {
                let req = GetListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_list(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::GetDetailedList) => {
                let req = GetDetailedListRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_detailed_list(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(FriendsProtocolMethod::GetRelationships) => {
                let req = GetRelationshipsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_relationships(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
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
pub trait FriendsProtocolServerTrait<CI> {
    fn add_friend(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddFriendRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
pub struct FriendsProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> FriendsProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> ClientProtocol<CI> for FriendsProtocolClient<CI> {
    fn id(&self) -> u16 {
        FRIENDS_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "FriendsProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        13u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        FriendsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> FriendsProtocolClient<CI> {
    pub fn add_friend(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::AddFriend as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn add_friend_by_name(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::AddFriendByName as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn add_friend_with_details(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::AddFriendWithDetails as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn add_friend_by_name_with_details(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::AddFriendByNameWithDetails as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn accept_friendship(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::AcceptFriendship as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn decline_friendship(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::DeclineFriendship as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn black_list(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::BlackList as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn black_list_by_name(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::BlackListByName as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn clear_relationship(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::ClearRelationship as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn update_details(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::UpdateDetails as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_list(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::GetList as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_detailed_list(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::GetDetailedList as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_relationships(
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
        self.send(
            logger,
            ctx,
            ci,
            FriendsProtocolMethod::GetRelationships as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}