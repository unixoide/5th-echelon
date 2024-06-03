use std::collections::HashMap;

use quazal::prudp::ClientRegistry;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use sc_bl_protocols::privileges_service::types::Privilege;
use slog::Logger;

use crate::login_required;
use crate::protocols::privileges_service::privileges_protocol::GetPrivilegesRequest;
use crate::protocols::privileges_service::privileges_protocol::GetPrivilegesResponse;
use crate::protocols::privileges_service::privileges_protocol::PrivilegesProtocolServer;
use crate::protocols::privileges_service::privileges_protocol::PrivilegesProtocolServerTrait;

struct PrivilegesProtocolServerImpl;

impl<T> PrivilegesProtocolServerTrait<T> for PrivilegesProtocolServerImpl {
    fn get_privileges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetPrivilegesRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetPrivilegesResponse, Error> {
        login_required(&*ci)?;
        // TODO add all privileges
        Ok(GetPrivilegesResponse {
            privileges: HashMap::from([(
                1,
                Privilege {
                    id: 1,
                    description: "PlayOnline".into(),
                },
            )]),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PrivilegesProtocolServer::new(PrivilegesProtocolServerImpl))
}
