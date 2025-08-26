//! Implements the `PrivilegesProtocolServer` for handling user privilege requests.

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

/// Implementation of the `PrivilegesProtocolServerTrait` for managing user privileges.
struct PrivilegesProtocolServerImpl;

impl<T> PrivilegesProtocolServerTrait<T> for PrivilegesProtocolServerImpl {
    /// Handles the `GetPrivileges` request, returning a map of user privileges.
    ///
    /// This function requires the client to be logged in. Currently, it returns a hardcoded
    /// "PlayOnline" privilege. The commented-out section shows how additional privileges
    /// could be dynamically generated.
    fn get_privileges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetPrivilegesRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetPrivilegesResponse, Error> {
        // Ensure the client is logged in before retrieving privileges.
        login_required(&*ci)?;
        let privileges = HashMap::from([(
            1,
            Privilege {
                id: 1,
                description: "PlayOnline".into(),
            },
        )]);
        // for id in 2000..2045 {
        //     privileges.insert(
        //         id,
        //         Privilege {
        //             id,
        //             description: format!("Priv {id}"),
        //         },
        //     );
        // }
        Ok(GetPrivilegesResponse { privileges })
    }
}

/// Creates a new boxed `PrivilegesProtocolServer` instance.
///
/// This function is typically used to register the privileges protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PrivilegesProtocolServer::new(PrivilegesProtocolServerImpl))
}
