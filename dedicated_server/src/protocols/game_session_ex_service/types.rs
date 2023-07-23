use crate::protocols::game_session_service::types::GameSessionParticipant;
use crate::protocols::game_session_service::types::GameSessionSearchResult;

#[derive(Debug, ToStream, FromStream)]
pub struct GameSessionSearchResultEx {
    pub base: GameSessionSearchResult,
    pub participants: quazal::rmc::types::QList<GameSessionParticipant>,
}
