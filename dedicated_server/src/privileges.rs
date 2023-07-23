use std::collections::HashMap;

use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::privileges_service::privileges_protocol::GetPrivilegesRequest;
use crate::protocols::privileges_service::privileges_protocol::GetPrivilegesResponse;
use crate::protocols::privileges_service::privileges_protocol::PrivilegesProtocol;
use crate::protocols::privileges_service::privileges_protocol::PrivilegesProtocolTrait;

struct PrivilegesProtocolImpl;

impl<T> PrivilegesProtocolTrait<T> for PrivilegesProtocolImpl {
    fn get_privileges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetPrivilegesRequest,
    ) -> Result<GetPrivilegesResponse, Error> {
        login_required(&*ci)?;
        // TODO add all privileges
        Ok(GetPrivilegesResponse {
            privileges: HashMap::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PrivilegesProtocol::new(PrivilegesProtocolImpl))
}
