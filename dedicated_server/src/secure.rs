use quazal::rmc::types::QResult;
use quazal::rmc::Protocol;

use crate::login_required;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterExRequest;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterExResponse;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterRequest;
use crate::protocols::secure_connection_service::secure_connection_protocol::RegisterResponse;
use crate::protocols::secure_connection_service::secure_connection_protocol::SecureConnectionProtocol;
use crate::protocols::secure_connection_service::secure_connection_protocol::SecureConnectionProtocolTrait;

struct SecureConnectionProtocolImpl;

impl<T> SecureConnectionProtocolTrait<T> for SecureConnectionProtocolImpl {
    fn register(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: RegisterRequest,
    ) -> Result<RegisterResponse, quazal::rmc::Error> {
        let user_id = login_required(&*ci)?;
        Ok(RegisterResponse {
            return_value: QResult::Ok,
            pid_connection_id: user_id, // probably wrong
            url_public: format!(
                "prudp:/address={};port={};sid={};type=2",
                ci.address().ip(),
                ci.address().port(),
                14
            )
            .into(),
        })
    }

    fn register_ex(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: RegisterExRequest,
    ) -> Result<RegisterExResponse, quazal::rmc::Error> {
        let user_id = login_required(&*ci)?;
        Ok(RegisterExResponse {
            return_value: QResult::Ok,
            pid_connection_id: user_id, // probably wrong
            url_public: format!(
                "prudp:/address={};port={};sid={};type=3",
                ci.address().ip(),
                ci.address().port(),
                15
            )
            .into(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(SecureConnectionProtocol::new(SecureConnectionProtocolImpl))
}
