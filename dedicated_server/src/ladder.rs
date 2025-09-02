use std::time::Duration;
use std::time::SystemTime;

use quazal::prudp::ClientRegistry;
use quazal::rmc::Protocol;
use quazal::Context;

use crate::login_required;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::GetUnixUtcRequest;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::GetUnixUtcResponse;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::LadderHelperProtocolServer;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::LadderHelperProtocolServerTrait;

/// Implementation of the `LadderHelperProtocolServerTrait` for handling ladder-related requests.
struct LadderHelperProtocolServerImpl;

impl<T> LadderHelperProtocolServerTrait<T> for LadderHelperProtocolServerImpl {
    /// Handles the `GetUnixUtc` request, returning the current Unix timestamp.
    ///
    /// This function requires the client to be logged in.
    fn get_unix_utc(
        &self,
        _logger: &slog::Logger,
        _ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: GetUnixUtcRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetUnixUtcResponse, quazal::rmc::Error> {
        login_required(&*ci)?;

        #[allow(clippy::cast_possible_truncation)]
        Ok(GetUnixUtcResponse {
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).as_ref().map(Duration::as_secs).unwrap_or_default() as u32,
        })
    }
}

/// Creates a new boxed `LadderHelperProtocolServer` instance.
///
/// This function is typically used to register the ladder helper protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LadderHelperProtocolServer::new(LadderHelperProtocolServerImpl))
}
