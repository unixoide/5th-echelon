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
enum UplayWinProtocolMethod {
    GetActions = 1u32,
    GetActionsCompleted = 2u32,
    GetActionsCount = 3u32,
    GetActionsCompletedCount = 4u32,
    GetRewards = 5u32,
    GetRewardsPurchased = 6u32,
    UplayWelcome = 7u32,
    SetActionCompleted = 8u32,
    SetActionsCompleted = 9u32,
    GetUserToken = 10u32,
    GetVirtualCurrencyUserBalance = 11u32,
    GetSectionsByKey = 12u32,
    BuyReward = 13u32,
}
#[derive(Debug, FromStream)]
pub struct GetActionsRequest {
    pub start_row_index: i32,
    pub maximum_rows: i32,
    pub sort_expression: String,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetActionsResponse {
    pub action_list: quazal::rmc::types::QList<UplayAction>,
}
#[derive(Debug, FromStream)]
pub struct GetActionsCompletedRequest {
    pub start_row_index: i32,
    pub maximum_rows: i32,
    pub sort_expression: String,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetActionsCompletedResponse {
    pub action_list: quazal::rmc::types::QList<UplayAction>,
}
#[derive(Debug, FromStream)]
pub struct GetActionsCountRequest {
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetActionsCountResponse {
    pub actions_count: i32,
}
#[derive(Debug, FromStream)]
pub struct GetActionsCompletedCountRequest {
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetActionsCompletedCountResponse {
    pub actions_count: i32,
}
#[derive(Debug, FromStream)]
pub struct GetRewardsRequest {
    pub start_row_index: i32,
    pub maximum_rows: i32,
    pub sort_expression: String,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetRewardsResponse {
    pub reward_list: quazal::rmc::types::QList<UplayReward>,
}
#[derive(Debug, FromStream)]
pub struct GetRewardsPurchasedRequest {
    pub start_row_index: i32,
    pub maximum_rows: i32,
    pub sort_expression: String,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetRewardsPurchasedResponse {
    pub reward_list: quazal::rmc::types::QList<UplayReward>,
}
#[derive(Debug, FromStream)]
pub struct UplayWelcomeRequest {
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct UplayWelcomeResponse {
    pub action_list: quazal::rmc::types::QList<UplayAction>,
}
#[derive(Debug, FromStream)]
pub struct SetActionCompletedRequest {
    pub action_code: String,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct SetActionCompletedResponse {
    pub unlocked_action: UplayAction,
}
#[derive(Debug, FromStream)]
pub struct SetActionsCompletedRequest {
    pub action_code_list: quazal::rmc::types::QList<String>,
    pub culture_name: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct SetActionsCompletedResponse {
    pub action_list: quazal::rmc::types::QList<UplayAction>,
}
#[derive(Debug, FromStream)]
pub struct GetUserTokenRequest;
#[derive(Debug, ToStream)]
pub struct GetUserTokenResponse {
    pub token: String,
}
#[derive(Debug, FromStream)]
pub struct GetVirtualCurrencyUserBalanceRequest {
    pub platform_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetVirtualCurrencyUserBalanceResponse {
    pub virtual_currency_user_balance: i32,
}
#[derive(Debug, FromStream)]
pub struct GetSectionsByKeyRequest {
    pub culture_name: String,
    pub section_key: String,
    pub platform_code: String,
    pub game_code: String,
}
#[derive(Debug, ToStream)]
pub struct GetSectionsByKeyResponse {
    pub section_list: quazal::rmc::types::QList<UplaySection>,
}
#[derive(Debug, FromStream)]
pub struct BuyRewardRequest {
    pub reward_code: String,
    pub platform_code: String,
}
#[derive(Debug, ToStream)]
pub struct BuyRewardResponse {
    pub virtual_currency_user_balance: i32,
}
pub struct UplayWinProtocol<T: UplayWinProtocolTrait<CI>, CI>(T, ::std::marker::PhantomData<CI>);
impl<T: UplayWinProtocolTrait<CI>, CI> UplayWinProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: UplayWinProtocolTrait<CI>, CI> Protocol<CI> for UplayWinProtocol<T, CI> {
    fn id(&self) -> u16 {
        49u16
    }
    fn name(&self) -> String {
        "UplayWinProtocol".to_string()
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
        let method = UplayWinProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(UplayWinProtocolMethod::GetActions) => {
                let req = GetActionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_actions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetActionsCompleted) => {
                let req = GetActionsCompletedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_actions_completed(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetActionsCount) => {
                let req = GetActionsCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_actions_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetActionsCompletedCount) => {
                let req = GetActionsCompletedCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_actions_completed_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetRewards) => {
                let req = GetRewardsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_rewards(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetRewardsPurchased) => {
                let req = GetRewardsPurchasedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_rewards_purchased(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::UplayWelcome) => {
                let req = UplayWelcomeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.uplay_welcome(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::SetActionCompleted) => {
                let req = SetActionCompletedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.set_action_completed(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::SetActionsCompleted) => {
                let req = SetActionsCompletedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.set_actions_completed(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetUserToken) => {
                let req = GetUserTokenRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_user_token(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetVirtualCurrencyUserBalance) => {
                let req = GetVirtualCurrencyUserBalanceRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .get_virtual_currency_user_balance(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::GetSectionsByKey) => {
                let req = GetSectionsByKeyRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_sections_by_key(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(UplayWinProtocolMethod::BuyReward) => {
                let req = BuyRewardRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.buy_reward(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        UplayWinProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait UplayWinProtocolTrait<CI> {
    fn get_actions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetActionsRequest,
    ) -> Result<GetActionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_actions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_actions_completed(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetActionsCompletedRequest,
    ) -> Result<GetActionsCompletedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_actions_completed)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_actions_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetActionsCountRequest,
    ) -> Result<GetActionsCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_actions_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_actions_completed_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetActionsCompletedCountRequest,
    ) -> Result<GetActionsCompletedCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_actions_completed_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_rewards(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetRewardsRequest,
    ) -> Result<GetRewardsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_rewards)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_rewards_purchased(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetRewardsPurchasedRequest,
    ) -> Result<GetRewardsPurchasedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_rewards_purchased)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn uplay_welcome(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UplayWelcomeRequest,
    ) -> Result<UplayWelcomeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(uplay_welcome)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn set_action_completed(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SetActionCompletedRequest,
    ) -> Result<SetActionCompletedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(set_action_completed)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn set_actions_completed(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SetActionsCompletedRequest,
    ) -> Result<SetActionsCompletedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(set_actions_completed)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_user_token(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetUserTokenRequest,
    ) -> Result<GetUserTokenResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_user_token)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_virtual_currency_user_balance(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetVirtualCurrencyUserBalanceRequest,
    ) -> Result<GetVirtualCurrencyUserBalanceResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_virtual_currency_user_balance)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_sections_by_key(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSectionsByKeyRequest,
    ) -> Result<GetSectionsByKeyResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(get_sections_by_key)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn buy_reward(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: BuyRewardRequest,
    ) -> Result<BuyRewardResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "UplayWinProtocol",
            stringify!(buy_reward)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
