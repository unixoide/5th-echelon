use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::player_stats_service::player_stats_protocol::{
    PlayerStatsProtocol, PlayerStatsProtocolTrait, ReadStatsByPlayersRequest,
    ReadStatsByPlayersResponse,
};

struct PlayerStatsProtocolImpl;

impl<T> PlayerStatsProtocolTrait<T> for PlayerStatsProtocolImpl {
    fn read_stats_by_players(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<T>,
        _request: ReadStatsByPlayersRequest,
    ) -> Result<ReadStatsByPlayersResponse, Error> {
        Ok(ReadStatsByPlayersResponse {
            results: Default::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PlayerStatsProtocol::new(PlayerStatsProtocolImpl))
}
