use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::uplay_win_service::uplay_win_protocol::{
    GetActionsCompletedRequest, GetActionsCompletedResponse, GetRewardsPurchasedRequest,
    GetRewardsPurchasedResponse, UplayWelcomeRequest, UplayWelcomeResponse, UplayWinProtocol,
    UplayWinProtocolTrait,
};

struct UplayWinProtocolImpl;

impl<CI> UplayWinProtocolTrait<CI> for UplayWinProtocolImpl {
    fn uplay_welcome(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: UplayWelcomeRequest,
    ) -> Result<UplayWelcomeResponse, Error> {
        Ok(UplayWelcomeResponse {
            action_list: Default::default(),
        })
    }
    fn get_actions_completed(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GetActionsCompletedRequest,
    ) -> Result<GetActionsCompletedResponse, Error> {
        Ok(GetActionsCompletedResponse {
            action_list: Default::default(),
        })
    }

    fn get_rewards_purchased(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GetRewardsPurchasedRequest,
    ) -> Result<GetRewardsPurchasedResponse, Error> {
        Ok(GetRewardsPurchasedResponse {
            reward_list: Default::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(UplayWinProtocol::new(UplayWinProtocolImpl))
}
