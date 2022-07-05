use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::game_session_service::{
    game_session_protocol::{
        AbandonSessionRequest, AbandonSessionResponse, AddParticipantsRequest,
        AddParticipantsResponse, CreateSessionRequest, CreateSessionResponse, DeleteSessionRequest,
        DeleteSessionResponse, GameSessionProtocol, GameSessionProtocolTrait, LeaveSessionRequest,
        LeaveSessionResponse, RegisterUrLsRequest, RegisterUrLsResponse, UpdateSessionRequest,
        UpdateSessionResponse,
    },
    types::GameSessionKey,
};

struct GameSessionProtocolImpl;

impl<CI> GameSessionProtocolTrait<CI> for GameSessionProtocolImpl {
    fn create_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        request: CreateSessionRequest,
    ) -> Result<CreateSessionResponse, Error> {
        Ok(CreateSessionResponse {
            game_session_key: GameSessionKey {
                type_id: request.game_session.type_id,
                session_id: 0x1234_5678,
            },
        })
    }
    fn update_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: UpdateSessionRequest,
    ) -> Result<UpdateSessionResponse, Error> {
        Ok(UpdateSessionResponse)
    }
    fn delete_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: DeleteSessionRequest,
    ) -> Result<DeleteSessionResponse, Error> {
        Ok(DeleteSessionResponse)
    }
    fn leave_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: LeaveSessionRequest,
    ) -> Result<LeaveSessionResponse, Error> {
        Ok(LeaveSessionResponse)
    }
    fn add_participants(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: AddParticipantsRequest,
    ) -> Result<AddParticipantsResponse, Error> {
        Ok(AddParticipantsResponse)
    }
    fn abandon_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: AbandonSessionRequest,
    ) -> Result<AbandonSessionResponse, Error> {
        Ok(AbandonSessionResponse)
    }
    fn register_ur_ls(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: RegisterUrLsRequest,
    ) -> Result<RegisterUrLsResponse, Error> {
        Ok(RegisterUrLsResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(GameSessionProtocol::new(GameSessionProtocolImpl))
}
