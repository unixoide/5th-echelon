use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::uplay_win_service::uplay_win_protocol::GetActionsCompletedRequest;
use crate::protocols::uplay_win_service::uplay_win_protocol::GetActionsCompletedResponse;
use crate::protocols::uplay_win_service::uplay_win_protocol::GetRewardsPurchasedRequest;
use crate::protocols::uplay_win_service::uplay_win_protocol::GetRewardsPurchasedResponse;
use crate::protocols::uplay_win_service::uplay_win_protocol::UplayWelcomeRequest;
use crate::protocols::uplay_win_service::uplay_win_protocol::UplayWelcomeResponse;
use crate::protocols::uplay_win_service::uplay_win_protocol::UplayWinProtocolServer;
use crate::protocols::uplay_win_service::uplay_win_protocol::UplayWinProtocolServerTrait;

struct UplayWinProtocolServerImpl;

impl<CI> UplayWinProtocolServerTrait<CI> for UplayWinProtocolServerImpl {
    fn uplay_welcome(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: UplayWelcomeRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<UplayWelcomeResponse, Error> {
        login_required(&*ci)?;
        Ok(UplayWelcomeResponse { action_list: QList::default() })
    }
    fn get_actions_completed(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GetActionsCompletedRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetActionsCompletedResponse, Error> {
        login_required(&*ci)?;
        Ok(GetActionsCompletedResponse { action_list: QList::default() })
    }

    fn get_rewards_purchased(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GetRewardsPurchasedRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetRewardsPurchasedResponse, Error> {
        login_required(&*ci)?;
        Ok(GetRewardsPurchasedResponse { reward_list: QList::default() })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(UplayWinProtocolServer::new(UplayWinProtocolServerImpl))
}
