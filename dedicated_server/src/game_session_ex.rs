use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use quazal::prudp::ClientRegistry;
use quazal::rmc::types::Property;
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
        // search svm
        // 103 => 2165463540
        // 106 => 3564829
        // 107 => 3909881133
        // 108 => 0
        // 109 => 0
        // 110 => 0
        // 112 => 1
        // search coop
        // 106 => 3564829
        // 107 => 3909881133
        // 108 => 0
        // 109 => 0
        // 110 => 0
        // coop: 113 => 0;109 => 0;110 => 0;106 => 3564829;107 => 3909881133;108 => 0;3 => 2;4 => 0;101 => 3578398534;102 => 3;103 => 0;105 => 2;112 => 2
        // svm:  113 => 0;109 => 0;110 => 0;106 => 3564829;107 => 3909881133;108 => 0;3 => 4;4 => 0;101 => 72621668;102 => 8;103 => 2165463540;105 => 0;112 => 2
        // coop: 113 => 0;109 => 0;110 => 0;106 => 3564829;107 => 3909881133;108 => 0;3 => 0;4 => 2;101 => 3578398534;102 => 3;103 => 0;105 => 2;112 => 2
        // coop: 113 => 0;109 => 0;110 => 0;106 => 3564829;107 => 3909881133;108 => 0;3 => 0;4 => 2;101 => 1328467886;102 => 5;103 => 0;105 => 2;112 => 2
        // coop: 113 => 0;109 => 0;110 => 0;106 => 3564829;107 => 3909881133;108 => 0;3 => 0;4 => 2;101 => 2573003522;102 => 4;103 => 0;105 => 2;112 => 2

        // 101 => might be map id
        // 102 => might be game mode
        // 4 => might be number of players? (svm)
        // 112 => might be number of players? (svm) first is set to 4, but 8 after opening and closening the match settings
        let mut req_attrs = request
            .game_session_query
            .parameters
            .0
            .into_iter()
            .map(|p| (p.id, p.value))
            .collect::<HashMap<_, _>>();
        // if not set assume 0 (required for keeping coop and svm apart)
        if let Entry::Vacant(entry) = req_attrs.entry(103) {
            entry.insert(0);
        }
        let sessions: Vec<_> = sessions
            .into_iter()
            .filter(|session| {
                let sess_attrs: QList<Property> = session.attributes.parse().unwrap();
                let sess_attrs = sess_attrs
                    .0
                    .into_iter()
                    .map(|p| (p.id, p.value))
                    .collect::<HashMap<_, _>>();
                for (id, value) in &req_attrs {
                    if *id == 112 {
                        // search request says 1 but in the session create request it says 2.
                        // no idea what they mean, but we would like to find those sessions, so skip this check
                        continue;
                    }
                    if sess_attrs.get(id) != Some(value) {
                        return false;
                    }
                }
                true
            })
            .collect();
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
    Box::new(GameSessionExProtocolServer::new(GameSessionExProtocolServerImpl {
        storage,
    }))
}
