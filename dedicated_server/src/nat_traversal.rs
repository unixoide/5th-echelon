use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::nat_traversal::nat_traversal_protocol::NatTraversalProtocol;
use crate::protocols::nat_traversal::nat_traversal_protocol::NatTraversalProtocolTrait;
use crate::protocols::nat_traversal::nat_traversal_protocol::RequestProbeInitiationExtRequest;
use crate::protocols::nat_traversal::nat_traversal_protocol::RequestProbeInitiationExtResponse;

struct NatTraversalProtocolImpl;

impl<T> NatTraversalProtocolTrait<T> for NatTraversalProtocolImpl {
    fn request_probe_initiation_ext(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: RequestProbeInitiationExtRequest,
    ) -> Result<RequestProbeInitiationExtResponse, Error> {
        let _user_id = login_required(&*ci)?;
        warn!(
            logger,
            "Method {}.{}({:?}) not implemented",
            "NatTraversalProtocol",
            stringify!(request_probe_initiation_ext),
            request,
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(NatTraversalProtocol::new(NatTraversalProtocolImpl))
}
