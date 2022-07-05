use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::localization_service::localization_protocol::{
    LocalizationProtocol, LocalizationProtocolTrait, SetLocaleCodeRequest, SetLocaleCodeResponse,
};

struct LocalizationProtocolImpl;

impl<T> LocalizationProtocolTrait<T> for LocalizationProtocolImpl {
    fn set_locale_code(
        &self,
        logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<T>,
        request: SetLocaleCodeRequest,
    ) -> Result<SetLocaleCodeResponse, Error> {
        debug!(logger, "setting locale to {}", request.local_code);
        Ok(SetLocaleCodeResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LocalizationProtocol::new(LocalizationProtocolImpl))
}
