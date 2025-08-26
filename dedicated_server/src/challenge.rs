//! Implements the `ChallengeHelperProtocolServer` for handling challenge-related requests.

use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::ChallengeHelperProtocolServer;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::ChallengeHelperProtocolServerTrait;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::GenerateFriendChallengesRequest;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::GenerateFriendChallengesResponse;

/// Implementation of the `ChallengeHelperProtocolServerTrait`.
struct ChallengeHelperProtocolServerImpl;

impl<CI> ChallengeHelperProtocolServerTrait<CI> for ChallengeHelperProtocolServerImpl {
    /// Handles the `GenerateFriendChallenges` request.
    ///
    /// This function requires the client to be logged in. It currently returns an empty list of challenges.
    fn generate_friend_challenges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GenerateFriendChallengesRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GenerateFriendChallengesResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(GenerateFriendChallengesResponse { result: QList::default() })
    }
}

/// Creates a new boxed `ChallengeHelperProtocolServer` instance.
///
/// This function is typically used to register the challenge helper protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(ChallengeHelperProtocolServer::new(ChallengeHelperProtocolServerImpl))
}
