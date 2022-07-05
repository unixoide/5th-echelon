use quazal::{
    rmc::{Error, Protocol},
    ClientInfo, Context,
};
use slog::Logger;

use crate::protocols::privileges_service::privileges_protocol::{
    GetPrivilegesRequest, GetPrivilegesResponse, PrivilegesProtocol, PrivilegesProtocolTrait,
};

struct PrivilegesProtocolImpl;

impl<T> PrivilegesProtocolTrait<T> for PrivilegesProtocolImpl {
    fn get_privileges(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        _ci: &mut ClientInfo<T>,
        _request: GetPrivilegesRequest,
    ) -> Result<GetPrivilegesResponse, Error> {
        // TODO add all privileges
        Ok(GetPrivilegesResponse {
            privileges: Default::default(),
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(PrivilegesProtocol::new(PrivilegesProtocolImpl))
}
