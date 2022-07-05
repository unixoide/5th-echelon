use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::{
    HasAcceptedLatestTosRequest, HasAcceptedLatestTosResponse, LookupPrincipalIdsRequest,
    LookupPrincipalIdsResponse, LookupUbiAccountIDsByPidsRequest,
    LookupUbiAccountIDsByPidsResponse, UbiAccountManagementProtocol,
    UbiAccountManagementProtocolTrait,
};
use quazal::rmc::Protocol;

struct UbiAccountManagementProtocolImpl;

impl<T> UbiAccountManagementProtocolTrait<T> for UbiAccountManagementProtocolImpl {
    fn lookup_principal_ids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        _ci: &mut quazal::ClientInfo<T>,
        request: LookupPrincipalIdsRequest,
    ) -> Result<LookupPrincipalIdsResponse, quazal::rmc::Error> {
        if request.ubi_account_ids.is_empty() {
            return Ok(LookupPrincipalIdsResponse {
                pids: Default::default(),
            });
        }
        warn!(
            logger,
            "Lookup requested for {} ubi ids!",
            request.ubi_account_ids.len()
        );
        // TODO lookup/generate IDs
        Ok(LookupPrincipalIdsResponse {
            pids: request
                .ubi_account_ids
                .into_iter()
                .enumerate()
                .map(|(i, uid)| (uid, 6000 + i as u32))
                .collect(),
        })
    }

    fn lookup_ubi_account_i_ds_by_pids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        _ci: &mut quazal::ClientInfo<T>,
        request: LookupUbiAccountIDsByPidsRequest,
    ) -> Result<LookupUbiAccountIDsByPidsResponse, quazal::rmc::Error> {
        if request.pids.is_empty() {
            return Ok(LookupUbiAccountIDsByPidsResponse {
                ubiaccount_i_ds: Default::default(),
            });
        }
        if request.pids.len() > 1 {
            warn!(logger, "Lookup requested for {} pids!", request.pids.len());
        }
        // TODO lookup/generate IDs
        Ok(LookupUbiAccountIDsByPidsResponse {
            ubiaccount_i_ds: [(
                *request.pids.first().unwrap(),
                "254afdac-ab95-43fb-b2c9-bdd3e75c0e56".to_owned(),
            )]
            .iter()
            .cloned()
            .collect(),
        })
    }

    fn has_accepted_latest_tos(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        _ci: &mut quazal::ClientInfo<T>,
        _request: HasAcceptedLatestTosRequest,
    ) -> Result<HasAcceptedLatestTosResponse, quazal::rmc::Error> {
        Ok(HasAcceptedLatestTosResponse {
            has_accepted: true,
            failed_reasons: vec![],
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(UbiAccountManagementProtocol::new(
        UbiAccountManagementProtocolImpl,
    ))
}
