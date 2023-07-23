use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::localization_service::localization_protocol::LocalizationProtocol;
use crate::protocols::localization_service::localization_protocol::LocalizationProtocolTrait;
use crate::protocols::localization_service::localization_protocol::SetLocaleCodeRequest;
use crate::protocols::localization_service::localization_protocol::SetLocaleCodeResponse;

struct LocalizationProtocolImpl;

impl<T> LocalizationProtocolTrait<T> for LocalizationProtocolImpl {
    fn set_locale_code(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: SetLocaleCodeRequest,
    ) -> Result<SetLocaleCodeResponse, Error> {
        login_required(&*ci)?;
        debug!(logger, "setting locale to {}", request.local_code);
        Ok(SetLocaleCodeResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LocalizationProtocol::new(LocalizationProtocolImpl))
}
