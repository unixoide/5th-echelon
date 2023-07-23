use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::ChallengeHelperProtocol;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::ChallengeHelperProtocolTrait;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::GenerateFriendChallengesRequest;
use crate::protocols::challenge_helper_service::challenge_helper_protocol::GenerateFriendChallengesResponse;

struct ChallengeHelperProtocolImpl;

impl<CI> ChallengeHelperProtocolTrait<CI> for ChallengeHelperProtocolImpl {
    fn generate_friend_challenges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GenerateFriendChallengesRequest,
    ) -> Result<GenerateFriendChallengesResponse, Error> {
        login_required(&*ci)?;
        Ok(GenerateFriendChallengesResponse {
            result: QList::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(ChallengeHelperProtocol::new(ChallengeHelperProtocolImpl))
}
