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
enum PlayerStatsProtocolMethod {
    WriteStats = 1u32,
    ReadStatsByPlayers = 2u32,
    ReadLeaderboardsNearPlayer = 3u32,
    ReadLeaderboardsByRank = 4u32,
    ReadLeaderboardsByPlayers = 5u32,
    ReadStatboardHistory = 6u32,
    ReadLeaderboardHistory = 7u32,
    ReadStatboardHistoryAggregated = 8u32,
    GetStatboardNextPurgeDate = 9u32,
    ReadLeaderboardsNearPlayer2 = 10u32,
    ReadLeaderboardsByRank2 = 11u32,
    ReadLeaderboardsByPlayers2 = 12u32,
    ReadPopulationStats = 13u32,
}
#[derive(Debug, FromStream)]
pub struct WriteStatsRequest {
    pub player_stat_updates: quazal::rmc::types::QList<PlayerStatUpdate>,
}
#[derive(Debug, ToStream)]
pub struct WriteStatsResponse;
#[derive(Debug, FromStream)]
pub struct ReadStatsByPlayersRequest {
    pub player_pi_ds: quazal::rmc::types::QList<u32>,
    pub queries: quazal::rmc::types::QList<StatboardQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadStatsByPlayersResponse {
    pub results: quazal::rmc::types::QList<StatboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsNearPlayerRequest {
    pub player_pid: u32,
    pub count: u32,
    pub queries: quazal::rmc::types::QList<LeaderboardQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsNearPlayerResponse {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsByRankRequest {
    pub starting_rank: u32,
    pub count: u32,
    pub queries: quazal::rmc::types::QList<LeaderboardQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsByRankResponse {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsByPlayersRequest {
    pub player_pi_ds: quazal::rmc::types::QList<u32>,
    pub queries: quazal::rmc::types::QList<LeaderboardQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsByPlayersResponse {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadStatboardHistoryRequest {
    pub queries: quazal::rmc::types::QList<StatboardHistoryQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadStatboardHistoryResponse {
    pub results: quazal::rmc::types::QList<StatboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardHistoryRequest {
    pub queries: quazal::rmc::types::QList<LeaderboardHistoryQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardHistoryResponse {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadStatboardHistoryAggregatedRequest {
    pub queries: quazal::rmc::types::QList<StatboardHistoryAggregatedQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadStatboardHistoryAggregatedResponse {
    pub results: quazal::rmc::types::QList<StatboardResult>,
}
#[derive(Debug, FromStream)]
pub struct GetStatboardNextPurgeDateRequest {
    pub board_id: u32,
    pub reset_frequency: u32,
}
#[derive(Debug, ToStream)]
pub struct GetStatboardNextPurgeDateResponse {
    pub purge_date: quazal::rmc::types::DateTime,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsNearPlayer2Request {
    pub player_pid: u32,
    pub count: u32,
    pub queries: quazal::rmc::types::QList<LeaderboardQuery2>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsNearPlayer2Response {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsByRank2Request {
    pub starting_rank: u32,
    pub count: u32,
    pub queries: quazal::rmc::types::QList<LeaderboardQuery2>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsByRank2Response {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadLeaderboardsByPlayers2Request {
    pub queries: quazal::rmc::types::QList<LeaderboardQuery2>,
}
#[derive(Debug, ToStream)]
pub struct ReadLeaderboardsByPlayers2Response {
    pub results: quazal::rmc::types::QList<LeaderboardResult>,
}
#[derive(Debug, FromStream)]
pub struct ReadPopulationStatsRequest {
    pub queries: quazal::rmc::types::QList<PopulationStatQuery>,
}
#[derive(Debug, ToStream)]
pub struct ReadPopulationStatsResponse {
    pub results: quazal::rmc::types::QList<PopulationStatResult>,
}
pub struct PlayerStatsProtocol<T: PlayerStatsProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: PlayerStatsProtocolTrait<CI>, CI> PlayerStatsProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: PlayerStatsProtocolTrait<CI>, CI> Protocol<CI> for PlayerStatsProtocol<T, CI> {
    fn id(&self) -> u16 {
        55u16
    }
    fn name(&self) -> String {
        "PlayerStatsProtocol".to_string()
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
        let method = PlayerStatsProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(PlayerStatsProtocolMethod::WriteStats) => {
                let req = WriteStatsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.write_stats(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadStatsByPlayers) => {
                let req = ReadStatsByPlayersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_stats_by_players(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsNearPlayer) => {
                let req = ReadLeaderboardsNearPlayerRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_near_player(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsByRank) => {
                let req = ReadLeaderboardsByRankRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_by_rank(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsByPlayers) => {
                let req = ReadLeaderboardsByPlayersRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_by_players(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadStatboardHistory) => {
                let req = ReadStatboardHistoryRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_statboard_history(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardHistory) => {
                let req = ReadLeaderboardHistoryRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboard_history(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadStatboardHistoryAggregated) => {
                let req = ReadStatboardHistoryAggregatedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .read_statboard_history_aggregated(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::GetStatboardNextPurgeDate) => {
                let req = GetStatboardNextPurgeDateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_statboard_next_purge_date(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsNearPlayer2) => {
                let req = ReadLeaderboardsNearPlayer2Request::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_near_player_2(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsByRank2) => {
                let req = ReadLeaderboardsByRank2Request::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_by_rank_2(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadLeaderboardsByPlayers2) => {
                let req = ReadLeaderboardsByPlayers2Request::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_leaderboards_by_players_2(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(PlayerStatsProtocolMethod::ReadPopulationStats) => {
                let req = ReadPopulationStatsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.read_population_stats(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        PlayerStatsProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait PlayerStatsProtocolTrait<CI> {
    fn write_stats(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: WriteStatsRequest,
    ) -> Result<WriteStatsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(write_stats)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_stats_by_players(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadStatsByPlayersRequest,
    ) -> Result<ReadStatsByPlayersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_stats_by_players)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_near_player(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsNearPlayerRequest,
    ) -> Result<ReadLeaderboardsNearPlayerResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_near_player)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_by_rank(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsByRankRequest,
    ) -> Result<ReadLeaderboardsByRankResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_by_rank)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_by_players(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsByPlayersRequest,
    ) -> Result<ReadLeaderboardsByPlayersResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_by_players)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_statboard_history(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadStatboardHistoryRequest,
    ) -> Result<ReadStatboardHistoryResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_statboard_history)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboard_history(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardHistoryRequest,
    ) -> Result<ReadLeaderboardHistoryResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboard_history)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_statboard_history_aggregated(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadStatboardHistoryAggregatedRequest,
    ) -> Result<ReadStatboardHistoryAggregatedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_statboard_history_aggregated)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_statboard_next_purge_date(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetStatboardNextPurgeDateRequest,
    ) -> Result<GetStatboardNextPurgeDateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(get_statboard_next_purge_date)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_near_player_2(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsNearPlayer2Request,
    ) -> Result<ReadLeaderboardsNearPlayer2Response, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_near_player_2)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_by_rank_2(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsByRank2Request,
    ) -> Result<ReadLeaderboardsByRank2Response, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_by_rank_2)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_leaderboards_by_players_2(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadLeaderboardsByPlayers2Request,
    ) -> Result<ReadLeaderboardsByPlayers2Response, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_leaderboards_by_players_2)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn read_population_stats(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReadPopulationStatsRequest,
    ) -> Result<ReadPopulationStatsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "PlayerStatsProtocol",
            stringify!(read_population_stats)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
