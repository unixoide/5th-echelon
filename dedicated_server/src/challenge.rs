use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::challenge_helper_service::challenge_helper_protocol::{
    ChallengeHelperProtocol, ChallengeHelperProtocolTrait, GenerateFriendChallengesRequest,
    GenerateFriendChallengesResponse,
};

struct ChallengeHelperProtocolImpl;

impl<CI> ChallengeHelperProtocolTrait<CI> for ChallengeHelperProtocolImpl {
    fn generate_friend_challenges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GenerateFriendChallengesRequest,
    ) -> Result<GenerateFriendChallengesResponse, Error> {
        Ok(GenerateFriendChallengesResponse {
            result: Default::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(ChallengeHelperProtocol::new(ChallengeHelperProtocolImpl))
}
