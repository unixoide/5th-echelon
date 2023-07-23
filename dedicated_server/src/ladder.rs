use std::time::Duration;
use std::time::SystemTime;

use quazal::rmc::Protocol;
use quazal::Context;

use crate::login_required;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::GetUnixUtcRequest;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::GetUnixUtcResponse;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::LadderHelperProtocol;
use crate::protocols::ladder_helper_service::ladder_helper_protocol::LadderHelperProtocolTrait;

struct LadderHelperProtocolImpl;

impl<T> LadderHelperProtocolTrait<T> for LadderHelperProtocolImpl {
    fn get_unix_utc(
        &self,
        _logger: &slog::Logger,
        _ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: GetUnixUtcRequest,
    ) -> Result<GetUnixUtcResponse, quazal::rmc::Error> {
        login_required(&*ci)?;

        #[allow(clippy::cast_possible_truncation)]
        Ok(GetUnixUtcResponse {
            time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .as_ref()
                .map(Duration::as_secs)
                .unwrap_or_default() as u32,
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(LadderHelperProtocol::new(LadderHelperProtocolImpl))
}
