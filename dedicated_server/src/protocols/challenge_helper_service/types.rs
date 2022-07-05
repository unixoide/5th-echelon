#[derive(Debug, ToStream, FromStream)]
pub struct FriendChallenge {
    pub challenge_type: u32,
    pub map_id: u32,
    pub friend_to_beat_pid: u32,
    pub value_on_hand: u32,
    pub value_to_beat: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct OnlineChallenge {
    pub challenge_id: u32,
    pub static_data: String,
    pub start_time: quazal::rmc::types::DateTime,
    pub end_time: quazal::rmc::types::DateTime,
    pub is_complete: bool,
}
