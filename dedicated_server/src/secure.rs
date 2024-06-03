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

struct SecureConnectionProtocolServerImpl;

impl<T> SecureConnectionProtocolServerTrait<T> for SecureConnectionProtocolServerImpl {
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
            url_public: format!(
                "prudp:/address={};port={};sid={};type=2",
                ci.address().ip(),
                ci.address().port(),
                14
            )
            .parse()
            .map_err(|_| quazal::rmc::Error::InternalError)?,
        })
    }

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
            url_public: format!(
                "prudp:/address={};port={};sid={};type=3",
                ci.address().ip(),
                ci.address().port(),
                15
            )
            .parse()
            .map_err(|_| quazal::rmc::Error::InternalError)?,
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(SecureConnectionProtocolServer::new(
        SecureConnectionProtocolServerImpl,
    ))
}
