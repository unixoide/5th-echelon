use std::sync::Arc;

use quazal::prudp::ClientRegistry;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use sc_bl_protocols::game_session_service::game_session_protocol::JoinSessionRequest;
use sc_bl_protocols::game_session_service::game_session_protocol::JoinSessionResponse;
use sc_bl_protocols::game_session_service::game_session_protocol::RemoveParticipantsRequest;
use sc_bl_protocols::game_session_service::game_session_protocol::RemoveParticipantsResponse;
use slog::Logger;

use crate::login_required;
use crate::protocols::game_session_service::game_session_protocol::AbandonSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::AbandonSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::AddParticipantsRequest;
use crate::protocols::game_session_service::game_session_protocol::AddParticipantsResponse;
use crate::protocols::game_session_service::game_session_protocol::CreateSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::CreateSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::DeleteSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::DeleteSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocolServer;
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocolServerTrait;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsRequest;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsResponse;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsWithParticipantsRequest;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsWithParticipantsResponse;
use crate::protocols::game_session_service::game_session_protocol::SplitSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::SplitSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionResponse;
use crate::protocols::game_session_service::types::GameSessionKey;
use crate::protocols::game_session_service::types::GameSessionSearchResult;
use crate::protocols::game_session_service::types::GameSessionSearchWithParticipantsResult;
use crate::storage::Storage;

struct GameSessionProtocolServerImpl {
    storage: Arc<Storage>,
}

impl<CI> GameSessionProtocolServerTrait<CI> for GameSessionProtocolServerImpl {
    fn create_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<CreateSessionResponse, Error> {
        info!(logger, "Client creates session: {:?}", request);
        let user_id = login_required(&*ci)?;

        let attributes = request
            .game_session
            .attributes
            .0
            .into_iter()
            /*
            101 => map
            102 => game mode
            https://github.com/GitHubProUser67/MultiServer3/blob/dc189cfac27589356a52d2ad64c31c8a124c68f7/SpecializedServers/QuazalServer/RDVServices/DDL/Models/GameSessionService/GameSession.cs#L15
             */
            .map(|p| format!("{} => {}", p.id, p.value))
            .collect::<Vec<_>>()
            .join(";");
        let session_id = rmc_err!(
            self.storage.create_game_session(user_id, request.game_session.type_id, attributes),
            logger,
            "error creating game session"
        )?;
        Ok(CreateSessionResponse {
            game_session_key: GameSessionKey {
                type_id: request.game_session.type_id,
                session_id,
            },
        })
    }

    fn update_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<UpdateSessionResponse, Error> {
        login_required(&*ci)?;
        info!(logger, "Client updates session: {:?}", request);
        let attributes = request
            .game_session_update
            .attributes
            .0
            .into_iter()
            /*
            101 => map
            102 => game mode
            https://github.com/GitHubProUser67/MultiServer3/blob/dc189cfac27589356a52d2ad64c31c8a124c68f7/SpecializedServers/QuazalServer/RDVServices/DDL/Models/GameSessionService/GameSession.cs#L15
             */
            .map(|p| format!("{} => {}", p.id, p.value))
            .collect::<Vec<_>>()
            .join(";");
        rmc_err!(
            self.storage.update_game_session(
                request.game_session_update.session_key.type_id,
                request.game_session_update.session_key.session_id,
                attributes,
            ),
            logger,
            "error updating game session"
        )?;
        Ok(UpdateSessionResponse)
    }

    fn delete_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<DeleteSessionResponse, Error> {
        let user_id = login_required(&*ci)?;
        if rmc_err!(
            self.storage
                .delete_game_session(user_id, request.game_session_key.type_id, request.game_session_key.session_id),
            logger,
            "error deleting session"
        )? != 1
        {
            warn!(logger, "Unexpected amount of sessions deleted");
        }
        Ok(DeleteSessionResponse)
    }

    fn leave_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: LeaveSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LeaveSessionResponse, Error> {
        login_required(&*ci)?;
        Ok(LeaveSessionResponse)
    }

    fn add_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<AddParticipantsResponse, Error> {
        login_required(&*ci)?;
        info!(logger, "Client adds participants: {:?}", request);
        rmc_err!(
            self.storage.add_participants(
                request.game_session_key.type_id,
                request.game_session_key.session_id,
                request.private_participant_ids.0.clone(),
                request.public_participant_ids.0.clone(),
            ),
            logger,
            "error adding participants"
        )?;
        Ok(AddParticipantsResponse)
    }

    fn remove_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RemoveParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RemoveParticipantsResponse, Error> {
        login_required(&*ci)?;
        info!(logger, "Client removes participants: {:?}", request);
        rmc_err!(
            self.storage
                .remove_participants(request.game_session_key.type_id, request.game_session_key.session_id, request.participant_ids.0.clone(),),
            logger,
            "error removing participants"
        )?;
        Ok(RemoveParticipantsResponse)
    }

    fn abandon_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: AbandonSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<AbandonSessionResponse, Error> {
        login_required(&*ci)?;
        Ok(AbandonSessionResponse)
    }

    fn register_urls(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterUrLsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RegisterUrLsResponse, Error> {
        let user_id = login_required(&*ci)?;
        info!(logger, "Client registers urls: {:?}", request);
        rmc_err!(
            self.storage.register_urls(user_id, request.station_urls.0.into_iter().map(|su| su.to_string()).collect()),
            logger,
            "error adding participants"
        )?;
        Ok(RegisterUrLsResponse)
    }

    fn search_sessions_with_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsWithParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SearchSessionsWithParticipantsResponse, Error> {
        let _user_id = login_required(&*ci)?;
        info!(logger, "Searches for sessions with {request:?}");

        let sessions = self
            .storage
            .search_sessions_with_participants(request.game_session_type_id, request.participant_ids.0.as_slice())
            .map_err(|e| {
                error!(logger, "Error searching game sessions: {e}");
                Error::InternalError
            })?;

        info!(logger, "Found sessions: {sessions:#?}");

        Ok(SearchSessionsWithParticipantsResponse {
            search_results: sessions
                .into_iter()
                .map(|session| {
                    let host = session.participants.iter().find(|p| p.user_id == session.creator_id).unwrap();
                    GameSessionSearchWithParticipantsResult {
                        game_session_search_result: GameSessionSearchResult {
                            session_key: GameSessionKey {
                                type_id: session.session_type,
                                session_id: session.session_id,
                            },
                            host_pid: host.user_id,
                            host_urls: host.station_urls.clone().try_into().unwrap(),
                            attributes: session.attributes.as_str().parse().unwrap(),
                        },
                        participant_ids: session.participants.into_iter().map(|p| p.user_id).collect(),
                    }
                })
                .collect(),
        })
    }

    fn split_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SplitSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SplitSessionResponse, Error> {
        let _user_id = login_required(&*ci)?;
        Ok(SplitSessionResponse {
            game_session_key_migrated: request.game_session_key,
        })
    }

    fn join_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: JoinSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<JoinSessionResponse, Error> {
        Ok(JoinSessionResponse)
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(GameSessionProtocolServer::new(GameSessionProtocolServerImpl { storage }))
}
