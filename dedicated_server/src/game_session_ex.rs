use quazal::{
    rmc::{
        types::{QList, StationURL},
        Error, Protocol,
    },
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::{
    game_session_ex_service::{
        game_session_ex_protocol::{
            GameSessionExProtocol, GameSessionExProtocolTrait, SearchSessionsRequest,
            SearchSessionsResponse,
        },
        types::GameSessionSearchResultEx,
    },
    game_session_service::types::{
        GameSessionKey, GameSessionParticipant, GameSessionSearchResult,
    },
};

struct GameSessionExProtocolImpl;

impl<CI> GameSessionExProtocolTrait<CI> for GameSessionExProtocolImpl {
    fn search_sessions(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: SearchSessionsRequest,
    ) -> Result<SearchSessionsResponse, Error> {
        Ok(SearchSessionsResponse {
            search_results: QList(vec![GameSessionSearchResultEx {
                base: GameSessionSearchResult {
                    session_key: GameSessionKey {
                        type_id: 1,
                        session_id: 0x1234_5678,
                    },
                    host_pid: 0x1000,
                    host_ur_ls: Default::default(),
                    attributes: Default::default(),
                },
                participants: QList(vec![GameSessionParticipant {
                    pid: 1337,
                    name: "CEO".to_string(),
                    station_ur_ls: QList(vec![StationURL(
                        "udp:/address=192.168.100.1;port=1337".to_owned(),
                    )]),
                }]),
            }]),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(GameSessionExProtocol::new(GameSessionExProtocolImpl))
}
