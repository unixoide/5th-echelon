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
pub const NEWS_PROTOCOL_ID: u16 = todo!();
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum NewsProtocolMethod {
    GetChannels = 1u32,
    GetChannelsByTypes = 2u32,
    GetSubscribableChannels = 3u32,
    GetChannelsByIDs = 4u32,
    GetSubscribedChannels = 5u32,
    SubscribeChannel = 6u32,
    UnsubscribeChannel = 7u32,
    GetNewsHeaders = 8u32,
    GetNewsMessages = 9u32,
    GetNumberOfNews = 10u32,
    GetChannelByType = 11u32,
    GetNewsHeadersByType = 12u32,
    GetNewsMessagesByType = 13u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsByTypesRequest {
    pub news_channel_types: quazal::rmc::types::QList<String>,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsByTypesResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetSubscribableChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetSubscribableChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsByIDsRequest {
    pub news_channel_ids: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelsByIDsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetSubscribedChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetSubscribedChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct SubscribeChannelRequest {
    pub news_channel_id: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct SubscribeChannelResponse;
#[derive(Debug, ToStream, FromStream)]
pub struct UnsubscribeChannelRequest {
    pub news_channel_id: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UnsubscribeChannelResponse;
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsHeadersRequest {
    pub recipient: NewsRecipient,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsHeadersResponse {
    pub news_headers: quazal::rmc::types::QList<NewsHeader>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsMessagesRequest {
    pub news_message_ids: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsMessagesResponse {
    pub news_messages: quazal::rmc::types::QList<NewsMessage>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNumberOfNewsRequest {
    pub recipient: NewsRecipient,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNumberOfNewsResponse {
    pub number_of_news: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelByTypeRequest {
    pub news_channel_type: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetChannelByTypeResponse {
    pub channel: NewsChannel,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsHeadersByTypeRequest {
    pub news_channel_type: String,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsHeadersByTypeResponse {
    pub news_headers: quazal::rmc::types::QList<NewsHeader>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsMessagesByTypeRequest {
    pub news_channel_type: String,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GetNewsMessagesByTypeResponse {
    pub news_messages: quazal::rmc::types::QList<NewsMessage>,
}
pub struct NewsProtocolServer<T: NewsProtocolServerTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: NewsProtocolServerTrait<CI>, CI> NewsProtocolServer<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: NewsProtocolServerTrait<CI>, CI> Protocol<CI> for NewsProtocolServer<T, CI> {
    fn id(&self) -> u16 {
        NEWS_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "NewsProtocol".to_string()
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
        let method = NewsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(NewsProtocolMethod::GetChannels) => {
                let req = GetChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_channels(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetChannelsByTypes) => {
                let req = GetChannelsByTypesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_channels_by_types(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetSubscribableChannels) => {
                let req = GetSubscribableChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_subscribable_channels(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetChannelsByIDs) => {
                let req = GetChannelsByIDsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_channels_by_ids(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetSubscribedChannels) => {
                let req = GetSubscribedChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_subscribed_channels(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::SubscribeChannel) => {
                let req = SubscribeChannelRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .subscribe_channel(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::UnsubscribeChannel) => {
                let req = UnsubscribeChannelRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .unsubscribe_channel(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetNewsHeaders) => {
                let req = GetNewsHeadersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_news_headers(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetNewsMessages) => {
                let req = GetNewsMessagesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_news_messages(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetNumberOfNews) => {
                let req = GetNumberOfNewsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_number_of_news(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetChannelByType) => {
                let req = GetChannelByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_channel_by_type(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetNewsHeadersByType) => {
                let req = GetNewsHeadersByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_news_headers_by_type(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
            Some(NewsProtocolMethod::GetNewsMessagesByType) => {
                let req = GetNewsMessagesByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp =
                    self.0
                        .get_news_messages_by_type(logger, ctx, ci, req, client_registry, socket);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.to_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NewsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait NewsProtocolServerTrait<CI> {
    fn get_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channels_by_types(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsByTypesRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetChannelsByTypesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels_by_types)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_subscribable_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSubscribableChannelsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetSubscribableChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_subscribable_channels)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channels_by_ids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsByIDsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetChannelsByIDsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels_by_ids)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_subscribed_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSubscribedChannelsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetSubscribedChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_subscribed_channels)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn subscribe_channel(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SubscribeChannelRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SubscribeChannelResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(subscribe_channel)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unsubscribe_channel(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnsubscribeChannelRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<UnsubscribeChannelResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(unsubscribe_channel)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_headers(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsHeadersRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetNewsHeadersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_headers)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_messages(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsMessagesRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetNewsMessagesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_messages)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_number_of_news(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNumberOfNewsRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetNumberOfNewsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_number_of_news)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channel_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelByTypeRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetChannelByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channel_by_type)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_headers_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsHeadersByTypeRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetNewsHeadersByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_headers_by_type)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_messages_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsMessagesByTypeRequest,
        client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetNewsMessagesByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_messages_by_type)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
pub struct NewsProtocolClient<CI>(::std::marker::PhantomData<CI>);
impl<CI> NewsProtocolClient<CI> {
    pub fn new() -> Self {
        Self(::std::marker::PhantomData)
    }
}
impl<CI> ClientProtocol<CI> for NewsProtocolClient<CI> {
    fn id(&self) -> u16 {
        NEWS_PROTOCOL_ID
    }
    fn name(&self) -> String {
        "NewsProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        13u32
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        NewsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
impl<CI> NewsProtocolClient<CI> {
    pub fn get_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsRequest,
    ) -> Result<GetChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetChannels as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_channels_by_types(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsByTypesRequest,
    ) -> Result<GetChannelsByTypesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels_by_types)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetChannelsByTypes as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_subscribable_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSubscribableChannelsRequest,
    ) -> Result<GetSubscribableChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_subscribable_channels)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetSubscribableChannels as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_channels_by_ids(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelsByIDsRequest,
    ) -> Result<GetChannelsByIDsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channels_by_ids)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetChannelsByIDs as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_subscribed_channels(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSubscribedChannelsRequest,
    ) -> Result<GetSubscribedChannelsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_subscribed_channels)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetSubscribedChannels as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn subscribe_channel(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SubscribeChannelRequest,
    ) -> Result<SubscribeChannelResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(subscribe_channel)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::SubscribeChannel as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn unsubscribe_channel(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UnsubscribeChannelRequest,
    ) -> Result<UnsubscribeChannelResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(unsubscribe_channel)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::UnsubscribeChannel as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_news_headers(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsHeadersRequest,
    ) -> Result<GetNewsHeadersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_headers)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetNewsHeaders as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_news_messages(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsMessagesRequest,
    ) -> Result<GetNewsMessagesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_messages)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetNewsMessages as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_number_of_news(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNumberOfNewsRequest,
    ) -> Result<GetNumberOfNewsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_number_of_news)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetNumberOfNews as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_channel_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetChannelByTypeRequest,
    ) -> Result<GetChannelByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_channel_by_type)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetChannelByType as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_news_headers_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsHeadersByTypeRequest,
    ) -> Result<GetNewsHeadersByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_headers_by_type)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetNewsHeadersByType as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    pub fn get_news_messages_by_type(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNewsMessagesByTypeRequest,
    ) -> Result<GetNewsMessagesByTypeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "NewsProtocol",
            stringify!(get_news_messages_by_type)
        );
        self.send(
            logger,
            ctx,
            ci,
            NewsProtocolMethod::GetNewsMessagesByType as u32,
            request.to_bytes(),
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
