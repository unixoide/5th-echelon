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
enum NewsProtocolMethod {
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
#[derive(Debug, FromStream)]
pub struct GetChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, FromStream)]
pub struct GetChannelsByTypesRequest {
    pub news_channel_types: quazal::rmc::types::QList<String>,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetChannelsByTypesResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, FromStream)]
pub struct GetSubscribableChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetSubscribableChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, FromStream)]
pub struct GetChannelsByIDsRequest {
    pub news_channel_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GetChannelsByIDsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, FromStream)]
pub struct GetSubscribedChannelsRequest {
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetSubscribedChannelsResponse {
    pub channels: quazal::rmc::types::QList<NewsChannel>,
}
#[derive(Debug, FromStream)]
pub struct SubscribeChannelRequest {
    pub news_channel_id: u32,
}
#[derive(Debug, ToStream)]
pub struct SubscribeChannelResponse;
#[derive(Debug, FromStream)]
pub struct UnsubscribeChannelRequest {
    pub news_channel_id: u32,
}
#[derive(Debug, ToStream)]
pub struct UnsubscribeChannelResponse;
#[derive(Debug, FromStream)]
pub struct GetNewsHeadersRequest {
    pub recipient: NewsRecipient,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetNewsHeadersResponse {
    pub news_headers: quazal::rmc::types::QList<NewsHeader>,
}
#[derive(Debug, FromStream)]
pub struct GetNewsMessagesRequest {
    pub news_message_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GetNewsMessagesResponse {
    pub news_messages: quazal::rmc::types::QList<NewsMessage>,
}
#[derive(Debug, FromStream)]
pub struct GetNumberOfNewsRequest {
    pub recipient: NewsRecipient,
}
#[derive(Debug, ToStream)]
pub struct GetNumberOfNewsResponse {
    pub number_of_news: u32,
}
#[derive(Debug, FromStream)]
pub struct GetChannelByTypeRequest {
    pub news_channel_type: String,
}
#[derive(Debug, ToStream)]
pub struct GetChannelByTypeResponse {
    pub channel: NewsChannel,
}
#[derive(Debug, FromStream)]
pub struct GetNewsHeadersByTypeRequest {
    pub news_channel_type: String,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetNewsHeadersByTypeResponse {
    pub news_headers: quazal::rmc::types::QList<NewsHeader>,
}
#[derive(Debug, FromStream)]
pub struct GetNewsMessagesByTypeRequest {
    pub news_channel_type: String,
    pub range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetNewsMessagesByTypeResponse {
    pub news_messages: quazal::rmc::types::QList<NewsMessage>,
}
pub struct NewsProtocol<T: NewsProtocolTrait<CI>, CI>(T, ::std::marker::PhantomData<CI>);
impl<T: NewsProtocolTrait<CI>, CI> NewsProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: NewsProtocolTrait<CI>, CI> Protocol<CI> for NewsProtocol<T, CI> {
    fn id(&self) -> u16 {
        todo!()
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
    ) -> Result<Vec<u8>, Error> {
        let method = NewsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(NewsProtocolMethod::GetChannels) => {
                let req = GetChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_channels(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetChannelsByTypes) => {
                let req = GetChannelsByTypesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_channels_by_types(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetSubscribableChannels) => {
                let req = GetSubscribableChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_subscribable_channels(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetChannelsByIDs) => {
                let req = GetChannelsByIDsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_channels_by_i_ds(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetSubscribedChannels) => {
                let req = GetSubscribedChannelsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_subscribed_channels(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::SubscribeChannel) => {
                let req = SubscribeChannelRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.subscribe_channel(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::UnsubscribeChannel) => {
                let req = UnsubscribeChannelRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.unsubscribe_channel(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetNewsHeaders) => {
                let req = GetNewsHeadersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_news_headers(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetNewsMessages) => {
                let req = GetNewsMessagesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_news_messages(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetNumberOfNews) => {
                let req = GetNumberOfNewsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_number_of_news(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetChannelByType) => {
                let req = GetChannelByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_channel_by_type(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetNewsHeadersByType) => {
                let req = GetNewsHeadersByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_news_headers_by_type(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(NewsProtocolMethod::GetNewsMessagesByType) => {
                let req = GetNewsMessagesByTypeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_news_messages_by_type(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
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
pub trait NewsProtocolTrait<CI> {
    fn get_channels(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channels_by_types(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_subscribable_channels(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channels_by_i_ds(
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
            stringify!(get_channels_by_i_ds)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_subscribed_channels(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn subscribe_channel(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn unsubscribe_channel(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_headers(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_messages(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_number_of_news(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_channel_by_type(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_headers_by_type(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_news_messages_by_type(
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
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
