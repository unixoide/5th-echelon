#[derive(Debug, ToStream, FromStream)]
pub struct PlayerStatUpdate {
    pub board_id: u32,
    pub context_i_ds: quazal::rmc::types::QList<u32>,
    pub stats: quazal::rmc::types::QList<quazal::rmc::types::PropertyVariant>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PlayerStatSortCriteria {
    pub sort_stat_id: u32,
    pub sort_order: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct StatboardQuery {
    pub board_id: u32,
    pub context_i_ds: quazal::rmc::types::QList<u32>,
    pub reset_frequency: u32,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
    pub sort_criterias: quazal::rmc::types::QList<PlayerStatSortCriteria>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct LeaderboardQuery {
    pub board_id: u32,
    pub context_id: u32,
    pub reset_frequency: u32,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct LeaderboardQuery2 {
    pub board_id: u32,
    pub context_id: u32,
    pub reset_frequency: u32,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
    pub estimated_pi_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PlayerStatSet {
    pub player_pid: u32,
    pub player_name: String,
    pub submitted_time: quazal::rmc::types::DateTime,
    pub stats: quazal::rmc::types::QList<quazal::rmc::types::PropertyVariant>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct StatboardResult {
    pub board_id: u32,
    pub context_id: u32,
    pub reset_frequency: u32,
    pub player_stat_sets: quazal::rmc::types::QList<PlayerStatSet>,
    pub default_stat_values: quazal::rmc::types::QList<quazal::rmc::types::PropertyVariant>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PlayerRank {
    pub player_stat_set: PlayerStatSet,
    pub rank_status: u32,
    pub rank: u32,
    pub score: quazal::rmc::types::Variant,
}
#[derive(Debug, ToStream, FromStream)]
pub struct LeaderboardResult {
    pub board_id: u32,
    pub context_id: u32,
    pub reset_frequency: u32,
    pub leaderboard_total_player_count: u32,
    pub player_ranks: quazal::rmc::types::QList<PlayerRank>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct DateRange {
    pub starting_datetime: quazal::rmc::types::DateTime,
    pub ending_datetime: quazal::rmc::types::DateTime,
}
#[derive(Debug, ToStream, FromStream)]
pub struct StatboardHistoryQuery {
    pub player_pi_ds: quazal::rmc::types::QList<u32>,
    pub board_id: u32,
    pub context_i_ds: quazal::rmc::types::QList<u32>,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
    pub result_range: quazal::rmc::types::ResultRange,
    pub player_stats: quazal::rmc::types::QList<quazal::rmc::types::PropertyVariant>,
    pub date_ranges: quazal::rmc::types::QList<DateRange>,
    pub sort_criterias: quazal::rmc::types::QList<PlayerStatSortCriteria>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct StatboardHistoryAggregatedQuery {
    pub player_pi_ds: quazal::rmc::types::QList<u32>,
    pub board_id: u32,
    pub context_i_ds: quazal::rmc::types::QList<u32>,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
    pub result_range: quazal::rmc::types::ResultRange,
    pub player_stats: quazal::rmc::types::QList<quazal::rmc::types::PropertyVariant>,
    pub date_ranges: quazal::rmc::types::QList<DateRange>,
    pub sort_criterias: quazal::rmc::types::QList<PlayerStatSortCriteria>,
    pub filter_option: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct LeaderboardHistoryQuery {
    pub player_pi_ds: quazal::rmc::types::QList<u32>,
    pub board_id: u32,
    pub context_id: u32,
    pub result_range: quazal::rmc::types::ResultRange,
    pub date_ranges: quazal::rmc::types::QList<DateRange>,
    pub filter_option: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PopulationStatQuery {
    pub board_id: u32,
    pub context_id: u32,
    pub stat_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PopulationStatResult {
    pub board_id: u32,
    pub context_id: u32,
    pub stat_id: u32,
    pub sum: f64,
    pub average: f64,
    pub standard_deviation: f64,
}
