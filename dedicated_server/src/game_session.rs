use std::sync::Arc;

use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
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
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocol;
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocolTrait;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsRequest;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsResponse;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionResponse;
use crate::protocols::game_session_service::types::GameSessionKey;
use crate::storage::Storage;

struct GameSessionProtocolImpl {
    storage: Arc<Storage>,
}

impl<CI> GameSessionProtocolTrait<CI> for GameSessionProtocolImpl {
    fn create_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateSessionRequest,
    ) -> Result<CreateSessionResponse, Error> {
        info!(logger, "Client creates session: {:?}", request);
        let user_id = login_required(&*ci)?;

        let attributes = request
            .game_session
            .attributes
            .0
            .into_iter()
            .map(|p| format!("{} => {}", p.id, p.value))
            .collect::<Vec<_>>()
            .join(";");
        let session_id = rmc_err!(
            self.storage
                .create_game_session(user_id, request.game_session.type_id, attributes),
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
    ) -> Result<UpdateSessionResponse, Error> {
        login_required(&*ci)?;
        info!(logger, "Client updates session: {:?}", request);
        Ok(UpdateSessionResponse)
    }
    fn delete_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteSessionRequest,
    ) -> Result<DeleteSessionResponse, Error> {
        let user_id = login_required(&*ci)?;
        if rmc_err!(
            self.storage.delete_game_session(
                user_id,
                request.game_session_key.type_id,
                request.game_session_key.session_id
            ),
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
    ) -> Result<AddParticipantsResponse, Error> {
        login_required(&*ci)?;
        info!(logger, "Client adds participants: {:?}", request);
        rmc_err!(
            self.storage.add_participants(
                request.game_session_key.type_id,
                request.game_session_key.session_id,
                request.private_participant_i_ds.0.clone(),
                request.public_participant_i_ds.0.clone(),
            ),
            logger,
            "error adding participants"
        )?;
        Ok(AddParticipantsResponse)
    }
    fn abandon_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: AbandonSessionRequest,
    ) -> Result<AbandonSessionResponse, Error> {
        login_required(&*ci)?;
        Ok(AbandonSessionResponse)
    }
    fn register_ur_ls(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterUrLsRequest,
    ) -> Result<RegisterUrLsResponse, Error> {
        let user_id = login_required(&*ci)?;
        info!(logger, "Client registers urls: {:?}", request);
        rmc_err!(
            self.storage.register_urls(
                user_id,
                request.station_ur_ls.0.into_iter().map(|su| su.0).collect()
            ),
            logger,
            "error adding participants"
        )?;
        Ok(RegisterUrLsResponse)
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(GameSessionProtocol::new(GameSessionProtocolImpl {
        storage,
    }))
}
