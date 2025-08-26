//! Implements the `LocalizationProtocolServer` for handling locale-related requests.

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

/// Implementation of the `LocalizationProtocolServerTrait` for handling localization requests.
struct LocalizationProtocolServerImpl;

impl<T> LocalizationProtocolServerTrait<T> for LocalizationProtocolServerImpl {
    /// Handles the `SetLocaleCode` request, setting the client's locale code.
    ///
    /// This function requires the client to be logged in.
    fn set_locale_code(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: SetLocaleCodeRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SetLocaleCodeResponse, Error> {
        // Ensure the client is logged in before setting the locale.
        login_required(&*ci)?;
        // Log the locale code being set.
        debug!(logger, "setting locale to {}", request.local_code);
        Ok(SetLocaleCodeResponse)
    }
}

/// Creates a new boxed `LocalizationProtocolServer` instance.
///
/// This function is typically used to register the localization protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LocalizationProtocolServer::new(LocalizationProtocolServerImpl))
}
