#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionKey {
    pub type_id: u32,
    pub session_id: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSession {
    pub type_id: u32,
    pub attributes: quazal::rmc::types::QList<quazal::rmc::types::Property>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionSearchResult {
    pub session_key: GameSessionKey,
    pub host_pid: u32,
    pub host_ur_ls: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
    pub attributes: quazal::rmc::types::QList<quazal::rmc::types::Property>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionUpdate {
    pub session_key: GameSessionKey,
    pub attributes: quazal::rmc::types::QList<quazal::rmc::types::Property>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionParticipant {
    pub pid: u32,
    pub name: String,
    pub station_ur_ls: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionInvitation {
    pub session_key: GameSessionKey,
    pub recipient_pi_ds: quazal::rmc::types::QList<u32>,
    pub message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionInvitationSent {
    pub session_key: GameSessionKey,
    pub recipient_pid: u32,
    pub message: String,
    pub creation_time: quazal::rmc::types::DateTime,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionInvitationReceived {
    pub session_key: GameSessionKey,
    pub sender_pid: u32,
    pub message: String,
    pub creation_time: quazal::rmc::types::DateTime,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionQuery {
    pub type_id: u32,
    pub query_id: u32,
    pub parameters: quazal::rmc::types::QList<quazal::rmc::types::Property>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionSocialQuery {
    pub type_id: u32,
    pub query_id: u32,
    pub parameters: quazal::rmc::types::QList<quazal::rmc::types::Property>,
    pub participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionMessage {
    pub session_key: GameSessionKey,
    pub message: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionSearchWithParticipantsResult {
    pub participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionUnsuccessfulJoinSession {
    pub session_key: GameSessionKey,
    pub error_category: u32,
    pub error_code: i32,
}
