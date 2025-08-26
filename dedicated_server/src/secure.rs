use quazal::prudp::ClientRegistry;
use quazal::rmc::types::QResult;
use quazal::rmc::Protocol;

use crate::login_required;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterExRequest;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterExResponse;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterRequest;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterResponse;
use crate::protocols::secure_connection_service::secure_connection_protocol::SecureConnectionProtocolServer;
use crate::protocols::secure_connection_service::secure_connection_protocol::SecureConnectionProtocolServerTrait;

/// Implementation of the `SecureConnectionProtocolServerTrait` for handling secure connection requests.
struct SecureConnectionProtocolServerImpl;

impl<T> SecureConnectionProtocolServerTrait<T> for SecureConnectionProtocolServerImpl {
    /// Handles the `Register` request, establishing a secure connection.
    ///
    /// This function requires the client to be logged in. It returns the connection ID
    /// and the public URL of the client.
    fn register(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: RegisterRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RegisterResponse, quazal::rmc::Error> {
        let _user_id = login_required(&*ci)?;
        info!(logger, "Client registers with {:?}", request);
        Ok(RegisterResponse {
            return_value: QResult::Ok,
            pid_connection_id: ci.connection_id.unwrap().into(), // should be set at this point
            url_public: format!("prudp:/address={};port={};sid={};type=2", ci.address().ip(), ci.address().port(), 14)
                .parse()
                .map_err(|_| quazal::rmc::Error::InternalError)?,
        })
    }

    /// Handles the `RegisterEx` request, establishing an extended secure connection.
    ///
    /// This function requires the client to be logged in. It returns the connection ID
    /// and the public URL of the client.
    fn register_ex(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: RegisterExRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RegisterExResponse, quazal::rmc::Error> {
        let _user_id = login_required(&*ci)?;
        info!(logger, "Client registers with {:?}", request);
        Ok(RegisterExResponse {
            return_value: QResult::Ok,
            pid_connection_id: ci.connection_id.unwrap().into(), // should be set at this point
            url_public: format!("prudp:/address={};port={};sid={};type=3", ci.address().ip(), ci.address().port(), 15)
                .parse()
                .map_err(|_| quazal::rmc::Error::InternalError)?,
        })
    }
}

/// Creates a new boxed `SecureConnectionProtocolServer` instance.
///
/// This function is typically used to register the secure connection protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(SecureConnectionProtocolServer::new(SecureConnectionProtocolServerImpl))
}
