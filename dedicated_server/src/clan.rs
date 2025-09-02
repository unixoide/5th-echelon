//! Implements the `ClanHelperProtocolServer` for handling clan-related requests.

use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QList;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::clan_helper_service::clan_helper_protocol::ClanHelperProtocolServer;
use crate::protocols::clan_helper_service::clan_helper_protocol::ClanHelperProtocolServerTrait;
use crate::protocols::clan_helper_service::clan_helper_protocol::GenerateClanChallengesRequest;
use crate::protocols::clan_helper_service::clan_helper_protocol::GenerateClanChallengesResponse;
use crate::protocols::clan_helper_service::clan_helper_protocol::GetClanInfoByPidRequest;
use crate::protocols::clan_helper_service::clan_helper_protocol::GetClanInfoByPidResponse;
use crate::protocols::clan_helper_service::clan_helper_protocol::GetMemberListByClidRequest;
use crate::protocols::clan_helper_service::clan_helper_protocol::GetMemberListByClidResponse;
use crate::protocols::clan_helper_service::types::ClanInfo;

/// Implementation of the `ClanHelperProtocolServerTrait` for handling clan-related operations.
struct ClanHelperProtocolServerImpl;

impl<CI> ClanHelperProtocolServerTrait<CI> for ClanHelperProtocolServerImpl {
    /// Handles the `GetClanInfoByPid` request.
    ///
    /// This function requires the client to be logged in. It returns hardcoded clan information.
    fn get_clan_info_by_pid(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GetClanInfoByPidRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetClanInfoByPidResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(GetClanInfoByPidResponse {
            clan_info: ClanInfo {
                clid: u32::MAX,
                tag: String::from("TEST"),
                title: String::from("FOO"),
                motto: String::from("BAR"),
            },
        })
    }

    /// Handles the `GenerateClanChallenges` request.
    ///
    /// This function requires the client to be logged in. It currently returns an empty list of challenges.
    fn generate_clan_challenges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GenerateClanChallengesRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GenerateClanChallengesResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(GenerateClanChallengesResponse { result: QList::default() })
    }

    /// Handles the `GetMemberListByClid` request.
    ///
    /// This function requires the client to be logged in. It returns a hardcoded list of members.
    fn get_member_list_by_clid(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GetMemberListByClidRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetMemberListByClidResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(GetMemberListByClidResponse { members: vec![1002].into() })
    }
}

/// Creates a new boxed `ClanHelperProtocolServer` instance.
///
/// This function is typically used to register the clan helper protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(ClanHelperProtocolServer::new(ClanHelperProtocolServerImpl))
}
