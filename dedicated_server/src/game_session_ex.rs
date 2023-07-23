use std::sync::Arc;
use std::vec;

use quazal::rmc::types::Property;
use quazal::rmc::types::QList;
use quazal::rmc::types::StationURL;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::GameSessionExProtocol;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::GameSessionExProtocolTrait;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::SearchSessionsRequest;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::SearchSessionsResponse;
use crate::protocols::game_session_ex_service::types::GameSessionSearchResultEx;
use crate::protocols::game_session_service::types::GameSessionKey;
use crate::protocols::game_session_service::types::GameSessionParticipant;
use crate::protocols::game_session_service::types::GameSessionSearchResult;
use crate::storage::Storage;
use crate::SERVER_PID;

struct GameSessionExProtocolImpl {
    storage: Arc<Storage>,
}

impl<CI> GameSessionExProtocolTrait<CI> for GameSessionExProtocolImpl {
    fn search_sessions(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsRequest,
    ) -> Result<SearchSessionsResponse, Error> {
        #![allow(clippy::unreadable_literal)]

        login_required(&*ci)?;
        info!(logger, "Client searches for session: {:?}", request);
        let sessions = rmc_err!(
            self.storage
                .search_sessions(request.game_session_query.type_id, ci.user_id),
            logger,
            "Error searching game sessions"
        )?;
        info!(logger, "Found sessions {sessions:?}");
        Ok(SearchSessionsResponse {
            search_results: QList(
                sessions
                    .into_iter()
                    .map(|session| GameSessionSearchResultEx {
                        base: GameSessionSearchResult {
                            session_key: GameSessionKey {
                                session_id: session.session_id,
                                type_id: session.session_type,
                            },
                            host_pid: SERVER_PID,
                            host_ur_ls: QList(vec![
                                StationURL("prudp:/address=127.0.0.1;port=123456;hdrType=00000000;RVCID=1234;type=2".into())
                            ]),
                            attributes: QList(vec![
                                Property {
                                    id: 113,
                                    value: 0,
                                   },
                                Property { id: 109, value: 0, },
                                Property { id: 110, value: 0, },
                                Property { id: 106, value: 3564829, },
                                Property { id: 107, value: 3909881133, },
                                Property { id: 108, value: 0, },
                                Property { id: 3, value: 2, },
                                Property { id: 4, value: 0, },
                                Property { id: 101, value: 3578398534, },
                                Property { id: 102, value: 3, },
                                Property { id: 103, value: 0, },
                                Property { id: 105, value: 2, },
                                Property { id: 112, value: 2, },
                            ]),
                        },
                        participants: QList(
                            session
                                .participants
                                .into_iter()
                                .map(|participant| GameSessionParticipant {
                                    pid: participant.user_id,
                                    name: participant.name,
                                    station_ur_ls: QList(
                                        participant
                                            .station_urls
                                            .into_iter()
                                            .map(StationURL)
                                            .collect(),
                                    ),
                                })
                                .collect(),
                        ),
                    })
                    .collect(),
            ),
        })
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(GameSessionExProtocol::new(GameSessionExProtocolImpl {
        storage,
    }))
}
