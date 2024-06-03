use std::sync::Arc;

use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::GameSessionExProtocolServer;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::GameSessionExProtocolServerTrait;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::SearchSessionsRequest;
use crate::protocols::game_session_ex_service::game_session_ex_protocol::SearchSessionsResponse;
use crate::protocols::game_session_ex_service::types::GameSessionSearchResultEx;
use crate::protocols::game_session_service::types::GameSessionKey;
use crate::protocols::game_session_service::types::GameSessionParticipant;
use crate::protocols::game_session_service::types::GameSessionSearchResult;
use crate::storage::Storage;

struct GameSessionExProtocolServerImpl {
    storage: Arc<Storage>,
}

impl<CI> GameSessionExProtocolServerTrait<CI> for GameSessionExProtocolServerImpl {
    fn search_sessions(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
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
                        game_session_search_result: GameSessionSearchResult {
                            session_key: GameSessionKey {
                                session_id: session.session_id,
                                type_id: session.session_type,
                            },
                            host_pid: session.creator_id,
                            host_urls: session
                                .participants
                                .iter()
                                .filter(|p| p.user_id == session.creator_id)
                                .flat_map(|p| p.station_urls.iter())
                                .map(|u| u.parse().unwrap())
                                .collect(),
                            attributes: session.attributes.parse().unwrap(),
                        },
                        participants: QList(
                            session
                                .participants
                                .into_iter()
                                .map(|participant| GameSessionParticipant {
                                    pid: participant.user_id,
                                    name: participant.name,
                                    station_urls: participant.station_urls.try_into().unwrap(),
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
    Box::new(GameSessionExProtocolServer::new(
        GameSessionExProtocolServerImpl { storage },
    ))
}
