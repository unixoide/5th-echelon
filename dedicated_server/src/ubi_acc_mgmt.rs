use std::collections::HashMap;
use std::sync::Arc;

use quazal::prudp::ClientRegistry;
use quazal::rmc::Protocol;

use crate::login_required;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::HasAcceptedLatestTosRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::HasAcceptedLatestTosResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupPrincipalIdsRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupPrincipalIdsResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupUbiAccountIDsByPidsRequest;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::LookupUbiAccountIDsByPidsResponse;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::UbiAccountManagementProtocolServer;
use crate::protocols::ubi_account_management_service::ubi_account_management_protocol::UbiAccountManagementProtocolServerTrait;
use crate::storage::Storage;

struct UbiAccountManagementProtocolServerImpl {
    storage: Arc<Storage>,
}

impl<T> UbiAccountManagementProtocolServerTrait<T> for UbiAccountManagementProtocolServerImpl {
    fn lookup_principal_ids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LookupPrincipalIdsRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LookupPrincipalIdsResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        if request.ubi_account_ids.is_empty() {
            return Ok(LookupPrincipalIdsResponse { pids: HashMap::default() });
        }
        let ubi_len = request.ubi_account_ids.len();
        let pids: HashMap<_, _> = request
            .ubi_account_ids
            .iter()
            .filter_map(|ubi_id| {
                self.storage
                    .find_user_id_by_ubi_id(ubi_id)
                    .map_err(|e| error!(logger, "storage lookup failed"; "error" => ?e))
                    .ok()
                    .flatten()
                    .map(|uid| (ubi_id.clone(), uid))
            })
            .collect();
        info!(
            logger,
            "Lookup requested for {} ubi ids ({:?}). Found {} ({:?})",
            ubi_len,
            request.ubi_account_ids,
            pids.len(),
            pids,
        );

        Ok(LookupPrincipalIdsResponse { pids })
    }

    fn lookup_ubi_account_ids_by_pids(
        &self,
        logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        request: LookupUbiAccountIDsByPidsRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LookupUbiAccountIDsByPidsResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        if request.pids.is_empty() {
            return Ok(LookupUbiAccountIDsByPidsResponse {
                ubiaccount_ids: HashMap::default(),
            });
        }
        let pid_len = request.pids.len();
        let ubiaccount_ids: HashMap<_, _> = request
            .pids
            .iter()
            .filter_map(|&uid| {
                self.storage
                    .find_ubi_id_by_user_id(uid)
                    .map_err(|e| error!(logger, "storage lookup failed"; "error" => ?e))
                    .ok()
                    .flatten()
                    .map(|ubi_id| (uid, ubi_id))
            })
            .collect();
        info!(logger, "Lookup requested for {} pids. Found {}", pid_len, ubiaccount_ids.len(),);
        info!(
            logger,
            "Lookup requested for {} ({:?}) pids. Found {} ({:?})",
            pid_len,
            request.pids,
            ubiaccount_ids.len(),
            ubiaccount_ids,
        );
        Ok(LookupUbiAccountIDsByPidsResponse { ubiaccount_ids })
    }

    fn has_accepted_latest_tos(
        &self,
        _logger: &slog::Logger,
        _ctx: &quazal::Context,
        ci: &mut quazal::ClientInfo<T>,
        _request: HasAcceptedLatestTosRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<HasAcceptedLatestTosResponse, quazal::rmc::Error> {
        login_required(&*ci)?;
        Ok(HasAcceptedLatestTosResponse {
            has_accepted: true,
            failed_reasons: vec![],
        })
    }
}

pub fn new_protocol<T: 'static>(storage: Arc<Storage>) -> Box<dyn Protocol<T>> {
    Box::new(UbiAccountManagementProtocolServer::new(UbiAccountManagementProtocolServerImpl { storage }))
}
