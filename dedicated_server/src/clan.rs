use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::clan_helper_service::{
    clan_helper_protocol::{
        ClanHelperProtocol, ClanHelperProtocolTrait, GenerateClanChallengesRequest,
        GenerateClanChallengesResponse, GetClanInfoByPidRequest, GetClanInfoByPidResponse,
        GetMemberListByClidRequest, GetMemberListByClidResponse,
    },
    types::ClanInfo,
};

struct ClanHelperProtocolImpl;

impl<CI> ClanHelperProtocolTrait<CI> for ClanHelperProtocolImpl {
    fn get_clan_info_by_pid(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GetClanInfoByPidRequest,
    ) -> Result<GetClanInfoByPidResponse, Error> {
        Ok(GetClanInfoByPidResponse {
            clan_info: ClanInfo {
                clid: u32::MAX,
                tag: "".to_owned(),
                title: "".to_owned(),
                motto: "".to_owned(),
            },
        })
    }

    fn generate_clan_challenges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GenerateClanChallengesRequest,
    ) -> Result<GenerateClanChallengesResponse, Error> {
        Ok(GenerateClanChallengesResponse {
            result: Default::default(),
        })
    }

    fn get_member_list_by_clid(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<CI>,
        _request: GetMemberListByClidRequest,
    ) -> Result<GetMemberListByClidResponse, Error> {
        Ok(GetMemberListByClidResponse {
            members: Default::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(ClanHelperProtocol::new(ClanHelperProtocolImpl))
}
