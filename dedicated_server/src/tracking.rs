use quazal::prudp::ClientRegistry;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::tracking_service::tracking_protocol_3::GetConfigurationRequest;
use crate::protocols::tracking_service::tracking_protocol_3::GetConfigurationResponse;
use crate::protocols::tracking_service::tracking_protocol_3::SendTagsRequest;
use crate::protocols::tracking_service::tracking_protocol_3::SendTagsResponse;
use crate::protocols::tracking_service::tracking_protocol_3::TrackingProtocol3Server;
use crate::protocols::tracking_service::tracking_protocol_3::TrackingProtocol3ServerTrait;

struct TrackingProtocol3ServerImpl;

impl<T> TrackingProtocol3ServerTrait<T> for TrackingProtocol3ServerImpl {
    fn get_configuration(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetConfigurationRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetConfigurationResponse, Error> {
        login_required(&*ci)?;
        Ok(GetConfigurationResponse {
            #[cfg(not(feature = "tracking"))]
            tags: Vec::new(),
            #[cfg(feature = "tracking")]
            tags: vec![
                "ADVCLIENT_STOP".to_string(),
                "ADVCTIOBJECTIVE_EVENT".to_string(),
                "ADVFRONTLINEOBJ_EVENT".to_string(),
                "ADVHACKING_EVENT".to_string(),
                "ADVRESPAWN_EVENT".to_string(),
                "ADVROUND_FINISH".to_string(),
                "ADVROUND_START".to_string(),
                "ADVSPAWNLOCATION_EVENT".to_string(),
                "ADVSPAWNVIS".to_string(),
                "ADVXP_GAINED".to_string(),
                "BLACKBOX_STOP".to_string(),
                "CINEMATIC_STOP".to_string(),
                "GADGET_EXPLOSION".to_string(),
                "GADGED_USED".to_string(),
                "GAME_START".to_string(),
                "GAME_STOP".to_string(),
                "LEVEL_END".to_string(),
                "LEVEL_START".to_string(),
                "LINKAPP_VIEW".to_string(),
                "LOBBY_ENTER".to_string(),
                "LOBBY_EXITHOST".to_string(),
                "LOBBY_EXITCLIENT".to_string(),
                "PLAYERBULLET_EVENT".to_string(),
                "PLAYER_DEATH".to_string(),
                "PLAYER_DETECT".to_string(),
                "PLAYER_HOSTAGE".to_string(),
                "PLAYER_KILL".to_string(),
                "PLAYER_LOADOUT".to_string(),
                "PLAYER_MARK".to_string(),
                "PLAYER_POS".to_string(),
                "PLAYER_REVIVE".to_string(),
                "TX_SPEND".to_string(),
                "UPLAY_START".to_string(),
                "UPLAY_STOP".to_string(),
                "WAVE_STOP".to_string(),
                "ADV_BUGREPORT".to_string(),
                "ADVCTIFLAG_POS".to_string(),
                "UPLAY_PASS".to_string(),
                "MENU_PASS".to_string(),
                "UPLAY_ACCOUNT".to_string(),
                "UPLAY_ACCOUNT_MENU".to_string(),
                "STOREACTION".to_string(),
                "SHADOWNET".to_string(),
                "GAME_LOC".to_string(),
                "PC_SPECS".to_string(),
                "LOBBY_COMPLETE".to_string(),
                "FPSCLIENT_START".to_string(),
                "FPSCLIENT_STOP".to_string(),
                "LEVEL_STOP".to_string(),
                "OBJECTIVE_START".to_string(),
                "OBJECTIVE_STOP".to_string(),
                "UPLAY_BROWSE".to_string(),
                "AWARD_UNLOCK".to_string(),
                "GAME_SAVE".to_string(),
                "INSTALL_START".to_string(),
                "INSTALL_STOP".to_string(),
                "MENU_ENTER".to_string(),
                "MENU_EXIT".to_string(),
                "MENU_OPTIONCHANGE".to_string(),
                "MM_RES".to_string(),
                "PLAYER_SAVED".to_string(),
                "UNINSTALL_START".to_string(),
                "UNINSTALL_STOP".to_string(),
                "VIDEO_START".to_string(),
                "VIDEO_STOP".to_string(),
                "BLACKBOX_END".to_string(),
                "PLAYER_DOWN".to_string(),
                "COMBO_END".to_string(),
            ],
        })
    }

    fn send_tags(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: SendTagsRequest,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SendTagsResponse, Error> {
        login_required(&*ci)?;
        Ok(SendTagsResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(TrackingProtocol3Server::new(TrackingProtocol3ServerImpl))
}
