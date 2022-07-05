use crate::protocols::secure_connection_service::secure_connection_protocol::{
    RegisterExRequest, RegisterExResponse, RegisterRequest, RegisterResponse,
    SecureConnectionProtocol, SecureConnectionProtocolTrait,
};
use quazal::rmc::{types::QResult, Protocol};

struct SecureConnectionProtocolImpl;

impl<T> SecureConnectionProtocolTrait<T> for SecureConnectionProtocolImpl {
    fn register(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: RegisterRequest,
    ) -> Result<RegisterResponse, quazal::rmc::Error> {
        Ok(RegisterResponse {
            return_value: QResult::Ok,
            pid_connection_id: 1234,
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
        Ok(RegisterExResponse {
            return_value: QResult::Ok,
            pid_connection_id: 1234,
            url_public: format!(
                "prudp:/address={};port={};sid={}",
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
