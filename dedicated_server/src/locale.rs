use quazal::prudp::ClientRegistry;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::localization_service::localization_protocol::LocalizationProtocolServer;
use crate::protocols::localization_service::localization_protocol::LocalizationProtocolServerTrait;
use crate::protocols::localization_service::localization_protocol::SetLocaleCodeRequest;
use crate::protocols::localization_service::localization_protocol::SetLocaleCodeResponse;

struct LocalizationProtocolServerImpl;

impl<T> LocalizationProtocolServerTrait<T> for LocalizationProtocolServerImpl {
    fn set_locale_code(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: SetLocaleCodeRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SetLocaleCodeResponse, Error> {
        login_required(&*ci)?;
        debug!(logger, "setting locale to {}", request.local_code);
        Ok(SetLocaleCodeResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LocalizationProtocolServer::new(
        LocalizationProtocolServerImpl,
    ))
}
