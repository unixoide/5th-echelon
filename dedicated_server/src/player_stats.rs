use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::player_stats_service::player_stats_protocol::PlayerStatsProtocol;
use crate::protocols::player_stats_service::player_stats_protocol::PlayerStatsProtocolTrait;
use crate::protocols::player_stats_service::player_stats_protocol::ReadStatsByPlayersRequest;
use crate::protocols::player_stats_service::player_stats_protocol::ReadStatsByPlayersResponse;
use crate::protocols::player_stats_service::player_stats_protocol::WriteStatsRequest;
use crate::protocols::player_stats_service::player_stats_protocol::WriteStatsResponse;

struct PlayerStatsProtocolImpl;

impl<T> PlayerStatsProtocolTrait<T> for PlayerStatsProtocolImpl {
    fn read_stats_by_players(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: ReadStatsByPlayersRequest,
    ) -> Result<ReadStatsByPlayersResponse, Error> {
        login_required(&*ci)?;
        Ok(ReadStatsByPlayersResponse {
            results: QList::default(),
        })
    }

    fn write_stats(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: WriteStatsRequest,
    ) -> Result<WriteStatsResponse, Error> {
        login_required(&*ci)?;
        Ok(WriteStatsResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PlayerStatsProtocol::new(PlayerStatsProtocolImpl))
}
