use std::collections::HashMap;
use std::sync::Arc;

use quazal::rmc::Protocol;

use crate::login_required;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::HasAcceptedLatestTosRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::HasAcceptedLatestTosResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupPrincipalIdsRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupPrincipalIdsResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupUbiAccountIDsByPidsRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupUbiAccountIDsByPidsResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::UbiAccountManagementProtocol;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::UbiAccountManagementProtocolTrait;
use crate::storage::Storage;

struct UbiAccountManagementProtocolImpl {
    storage: Arc<Storage>,
}

impl<T> UbiAccountManagementProtocolTrait<T> for UbiAccountManagementProtocolImpl {
    fn lookup_principal_ids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LookupPrincipalIdsRequest,
    ) -> Result<LookupPrincipalIdsResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        if request.ubi_account_ids.is_empty() {
            return Ok(LookupPrincipalIdsResponse {
                pids: HashMap::default(),
            });
        }
        let ubi_len = request.ubi_account_ids.len();
        let pids: HashMap<_, _> = request
            .ubi_account_ids
            .into_iter()
            .filter_map(|ubi_id| {
                self.storage
                    .find_user_id_by_ubi_id(&ubi_id)
                    .map_err(|e| error!(logger, "storage lookup failed"; "error" => ?e))
                    .ok()
                    .flatten()
                    .map(|uid| (ubi_id, uid))
            })
            .collect();
        info!(
            logger,
            "Lookup requested for {} ubi ids. Found {}",
            ubi_len,
            pids.len(),
        );

        Ok(LookupPrincipalIdsResponse { pids })
    }

    fn lookup_ubi_account_i_ds_by_pids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LookupUbiAccountIDsByPidsRequest,
    ) -> Result<LookupUbiAccountIDsByPidsResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        if request.pids.is_empty() {
            return Ok(LookupUbiAccountIDsByPidsResponse {
                ubiaccount_i_ds: HashMap::default(),
            });
        }
        let pid_len = request.pids.len();
        let ubiaccount_ids: HashMap<_, _> = request
            .pids
            .into_iter()
            .filter_map(|uid| {
                self.storage
                    .find_ubi_id_by_user_id(uid)
                    .map_err(|e| error!(logger, "storage lookup failed"; "error" => ?e))
                    .ok()
                    .flatten()
                    .map(|ubi_id| (uid, ubi_id))
            })
            .collect();
        info!(
            logger,
            "Lookup requested for {} pids. Found {}",
            pid_len,
            ubiaccount_ids.len(),
        );
        Ok(LookupUbiAccountIDsByPidsResponse {
            ubiaccount_i_ds: ubiaccount_ids,
        })
    }

    fn has_accepted_latest_tos(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: HasAcceptedLatestTosRequest,
    ) -> Result<HasAcceptedLatestTosResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        Ok(HasAcceptedLatestTosResponse {
            has_accepted: true,
            failed_reasons: vec![],
        })
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(UbiAccountManagementProtocol::new(
        UbiAccountManagementProtocolImpl { storage },
    ))
}
