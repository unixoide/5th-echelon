#![allow(
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

use std::convert::TryFrom;

use num_enum::TryFromPrimitive;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use super::types::*;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum GameSessionProtocolMethod {
    CreateSession = 1u32,
    UpdateSession = 2u32,
    DeleteSession = 3u32,
    MigrateSession = 4u32,
    LeaveSession = 5u32,
    GetSession = 6u32,
    SearchSessions = 7u32,
    AddParticipants = 8u32,
    RemoveParticipants = 9u32,
    GetParticipantCount = 10u32,
    GetParticipants = 11u32,
    SendInvitation = 12u32,
    GetInvitationReceivedCount = 13u32,
    GetInvitationsReceived = 14u32,
    GetInvitationSentCount = 15u32,
    GetInvitationsSent = 16u32,
    AcceptInvitation = 17u32,
    DeclineInvitation = 18u32,
    CancelInvitation = 19u32,
    SendTextMessage = 20u32,
    RegisterUrLs = 21u32,
    JoinSession = 22u32,
    AbandonSession = 23u32,
    SearchSessionsWithParticipants = 24u32,
    GetSessions = 25u32,
    GetParticipantsUrLs = 26u32,
    MigrateSessionHost = 27u32,
    SplitSession = 28u32,
    SearchSocialSessions = 29u32,
    ReportUnsuccessfulJoinSessions = 30u32,
}
#[derive(Debug, FromStream)]
pub struct CreateSessionRequest {
    pub game_session: GameSession,
}
#[derive(Debug, ToStream)]
pub struct CreateSessionResponse {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, FromStream)]
pub struct UpdateSessionRequest {
    pub game_session_update: GameSessionUpdate,
}
#[derive(Debug, ToStream)]
pub struct UpdateSessionResponse;
#[derive(Debug, FromStream)]
pub struct DeleteSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct DeleteSessionResponse;
#[derive(Debug, FromStream)]
pub struct MigrateSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct MigrateSessionResponse {
    pub game_session_key_migrated: GameSessionKey,
}
#[derive(Debug, FromStream)]
pub struct LeaveSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct LeaveSessionResponse;
#[derive(Debug, FromStream)]
pub struct GetSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct GetSessionResponse {
    pub search_result: GameSessionSearchResult,
}
#[derive(Debug, FromStream)]
pub struct SearchSessionsRequest {
    pub game_session_query: GameSessionQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchSessionsResponse {
    pub search_results: quazal::rmc::types::QList<GameSessionSearchResult>,
}
#[derive(Debug, FromStream)]
pub struct AddParticipantsRequest {
    pub game_session_key: GameSessionKey,
    pub public_participant_i_ds: quazal::rmc::types::QList<u32>,
    pub private_participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct AddParticipantsResponse;
#[derive(Debug, FromStream)]
pub struct RemoveParticipantsRequest {
    pub game_session_key: GameSessionKey,
    pub participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct RemoveParticipantsResponse;
#[derive(Debug, FromStream)]
pub struct GetParticipantCountRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct GetParticipantCountResponse {
    pub count: u32,
}
#[derive(Debug, FromStream)]
pub struct GetParticipantsRequest {
    pub game_session_key: GameSessionKey,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetParticipantsResponse {
    pub participants: quazal::rmc::types::QList<GameSessionParticipant>,
}
#[derive(Debug, FromStream)]
pub struct SendInvitationRequest {
    pub invitation: GameSessionInvitation,
}
#[derive(Debug, ToStream)]
pub struct SendInvitationResponse;
#[derive(Debug, FromStream)]
pub struct GetInvitationReceivedCountRequest {
    pub game_session_type_id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetInvitationReceivedCountResponse {
    pub count: u32,
}
#[derive(Debug, FromStream)]
pub struct GetInvitationsReceivedRequest {
    pub game_session_type_id: u32,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetInvitationsReceivedResponse {
    pub invitations: quazal::rmc::types::QList<GameSessionInvitationReceived>,
}
#[derive(Debug, FromStream)]
pub struct GetInvitationSentCountRequest {
    pub game_session_type_id: u32,
}
#[derive(Debug, ToStream)]
pub struct GetInvitationSentCountResponse {
    pub count: u32,
}
#[derive(Debug, FromStream)]
pub struct GetInvitationsSentRequest {
    pub game_session_type_id: u32,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct GetInvitationsSentResponse {
    pub invitations: quazal::rmc::types::QList<GameSessionInvitationSent>,
}
#[derive(Debug, FromStream)]
pub struct AcceptInvitationRequest {
    pub game_session_invitation: GameSessionInvitationReceived,
}
#[derive(Debug, ToStream)]
pub struct AcceptInvitationResponse;
#[derive(Debug, FromStream)]
pub struct DeclineInvitationRequest {
    pub game_session_invitation: GameSessionInvitationReceived,
}
#[derive(Debug, ToStream)]
pub struct DeclineInvitationResponse;
#[derive(Debug, FromStream)]
pub struct CancelInvitationRequest {
    pub game_session_invitation: GameSessionInvitationSent,
}
#[derive(Debug, ToStream)]
pub struct CancelInvitationResponse;
#[derive(Debug, FromStream)]
pub struct SendTextMessageRequest {
    pub game_session_message: GameSessionMessage,
}
#[derive(Debug, ToStream)]
pub struct SendTextMessageResponse;
#[derive(Debug, FromStream)]
pub struct RegisterUrLsRequest {
    pub station_ur_ls: quazal::rmc::types::QList<quazal::rmc::types::StationURL>,
}
#[derive(Debug, ToStream)]
pub struct RegisterUrLsResponse;
#[derive(Debug, FromStream)]
pub struct JoinSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct JoinSessionResponse;
#[derive(Debug, FromStream)]
pub struct AbandonSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct AbandonSessionResponse;
#[derive(Debug, FromStream)]
pub struct SearchSessionsWithParticipantsRequest {
    pub game_session_type_id: u32,
    pub participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct SearchSessionsWithParticipantsResponse {
    pub search_results: quazal::rmc::types::QList<GameSessionSearchWithParticipantsResult>,
}
#[derive(Debug, FromStream)]
pub struct GetSessionsRequest {
    pub game_session_keys: quazal::rmc::types::QList<GameSessionKey>,
}
#[derive(Debug, ToStream)]
pub struct GetSessionsResponse {
    pub search_results: quazal::rmc::types::QList<GameSessionSearchResult>,
}
#[derive(Debug, FromStream)]
pub struct GetParticipantsUrLsRequest {
    pub game_session_key: GameSessionKey,
    pub participant_i_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GetParticipantsUrLsResponse {
    pub participants: quazal::rmc::types::QList<GameSessionParticipant>,
}
#[derive(Debug, FromStream)]
pub struct MigrateSessionHostRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct MigrateSessionHostResponse;
#[derive(Debug, FromStream)]
pub struct SplitSessionRequest {
    pub game_session_key: GameSessionKey,
}
#[derive(Debug, ToStream)]
pub struct SplitSessionResponse {
    pub game_session_key_migrated: GameSessionKey,
}
#[derive(Debug, FromStream)]
pub struct SearchSocialSessionsRequest {
    pub game_session_social_query: GameSessionSocialQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchSocialSessionsResponse {
    pub search_results: quazal::rmc::types::QList<GameSessionSearchWithParticipantsResult>,
}
#[derive(Debug, FromStream)]
pub struct ReportUnsuccessfulJoinSessionsRequest {
    pub unsuccessful_join_sessions: quazal::rmc::types::QList<GameSessionUnsuccessfulJoinSession>,
}
#[derive(Debug, ToStream)]
pub struct ReportUnsuccessfulJoinSessionsResponse;
pub struct GameSessionProtocol<T: GameSessionProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: GameSessionProtocolTrait<CI>, CI> GameSessionProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: GameSessionProtocolTrait<CI>, CI> Protocol<CI> for GameSessionProtocol<T, CI> {
    fn id(&self) -> u16 {
        42u16
    }
    fn name(&self) -> String {
        "GameSessionProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        30u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = GameSessionProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(GameSessionProtocolMethod::CreateSession) => {
                let req = CreateSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.create_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::UpdateSession) => {
                let req = UpdateSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::DeleteSession) => {
                let req = DeleteSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.delete_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::MigrateSession) => {
                let req = MigrateSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.migrate_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::LeaveSession) => {
                let req = LeaveSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.leave_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetSession) => {
                let req = GetSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SearchSessions) => {
                let req = SearchSessionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_sessions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::AddParticipants) => {
                let req = AddParticipantsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_participants(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::RemoveParticipants) => {
                let req = RemoveParticipantsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.remove_participants(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetParticipantCount) => {
                let req = GetParticipantCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_participant_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetParticipants) => {
                let req = GetParticipantsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_participants(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SendInvitation) => {
                let req = SendInvitationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_invitation(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetInvitationReceivedCount) => {
                let req = GetInvitationReceivedCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_invitation_received_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetInvitationsReceived) => {
                let req = GetInvitationsReceivedRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_invitations_received(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetInvitationSentCount) => {
                let req = GetInvitationSentCountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_invitation_sent_count(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetInvitationsSent) => {
                let req = GetInvitationsSentRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_invitations_sent(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::AcceptInvitation) => {
                let req = AcceptInvitationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.accept_invitation(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::DeclineInvitation) => {
                let req = DeclineInvitationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.decline_invitation(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::CancelInvitation) => {
                let req = CancelInvitationRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.cancel_invitation(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SendTextMessage) => {
                let req = SendTextMessageRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.send_text_message(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::RegisterUrLs) => {
                let req = RegisterUrLsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.register_ur_ls(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::JoinSession) => {
                let req = JoinSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.join_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::AbandonSession) => {
                let req = AbandonSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.abandon_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SearchSessionsWithParticipants) => {
                let req = SearchSessionsWithParticipantsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .search_sessions_with_participants(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetSessions) => {
                let req = GetSessionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_sessions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::GetParticipantsUrLs) => {
                let req = GetParticipantsUrLsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_participants_ur_ls(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::MigrateSessionHost) => {
                let req = MigrateSessionHostRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.migrate_session_host(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SplitSession) => {
                let req = SplitSessionRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.split_session(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::SearchSocialSessions) => {
                let req = SearchSocialSessionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_social_sessions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(GameSessionProtocolMethod::ReportUnsuccessfulJoinSessions) => {
                let req = ReportUnsuccessfulJoinSessionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .report_unsuccessful_join_sessions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        GameSessionProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait GameSessionProtocolTrait<CI> {
    fn create_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateSessionRequest,
    ) -> Result<CreateSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(create_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateSessionRequest,
    ) -> Result<UpdateSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(update_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn delete_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteSessionRequest,
    ) -> Result<DeleteSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(delete_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn migrate_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: MigrateSessionRequest,
    ) -> Result<MigrateSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(migrate_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn leave_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LeaveSessionRequest,
    ) -> Result<LeaveSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(leave_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSessionRequest,
    ) -> Result<GetSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_sessions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsRequest,
    ) -> Result<SearchSessionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(search_sessions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn add_participants(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddParticipantsRequest,
    ) -> Result<AddParticipantsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(add_participants)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn remove_participants(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RemoveParticipantsRequest,
    ) -> Result<RemoveParticipantsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(remove_participants)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_participant_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetParticipantCountRequest,
    ) -> Result<GetParticipantCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_participant_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_participants(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetParticipantsRequest,
    ) -> Result<GetParticipantsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_participants)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn send_invitation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendInvitationRequest,
    ) -> Result<SendInvitationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(send_invitation)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_invitation_received_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationReceivedCountRequest,
    ) -> Result<GetInvitationReceivedCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_invitation_received_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_invitations_received(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationsReceivedRequest,
    ) -> Result<GetInvitationsReceivedResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_invitations_received)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_invitation_sent_count(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationSentCountRequest,
    ) -> Result<GetInvitationSentCountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_invitation_sent_count)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_invitations_sent(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationsSentRequest,
    ) -> Result<GetInvitationsSentResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_invitations_sent)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn accept_invitation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AcceptInvitationRequest,
    ) -> Result<AcceptInvitationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(accept_invitation)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn decline_invitation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeclineInvitationRequest,
    ) -> Result<DeclineInvitationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(decline_invitation)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn cancel_invitation(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CancelInvitationRequest,
    ) -> Result<CancelInvitationResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(cancel_invitation)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn send_text_message(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendTextMessageRequest,
    ) -> Result<SendTextMessageResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(send_text_message)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn register_ur_ls(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterUrLsRequest,
    ) -> Result<RegisterUrLsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(register_ur_ls)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn join_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: JoinSessionRequest,
    ) -> Result<JoinSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(join_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn abandon_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AbandonSessionRequest,
    ) -> Result<AbandonSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(abandon_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_sessions_with_participants(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsWithParticipantsRequest,
    ) -> Result<SearchSessionsWithParticipantsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(search_sessions_with_participants)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_sessions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetSessionsRequest,
    ) -> Result<GetSessionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_sessions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_participants_ur_ls(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetParticipantsUrLsRequest,
    ) -> Result<GetParticipantsUrLsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(get_participants_ur_ls)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn migrate_session_host(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: MigrateSessionHostRequest,
    ) -> Result<MigrateSessionHostResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(migrate_session_host)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn split_session(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SplitSessionRequest,
    ) -> Result<SplitSessionResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(split_session)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn search_social_sessions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSocialSessionsRequest,
    ) -> Result<SearchSocialSessionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(search_social_sessions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn report_unsuccessful_join_sessions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReportUnsuccessfulJoinSessionsRequest,
    ) -> Result<ReportUnsuccessfulJoinSessionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionProtocol",
            stringify!(report_unsuccessful_join_sessions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
