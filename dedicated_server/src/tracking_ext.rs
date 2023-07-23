use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::trackingextension::tracking_extension_protocol::GetTrackingUserGroupRequest;
use crate::protocols::trackingextension::tracking_extension_protocol::GetTrackingUserGroupResponse;
use crate::protocols::trackingextension::tracking_extension_protocol::GetTrackingUserGroupTagsRequest;
use crate::protocols::trackingextension::tracking_extension_protocol::GetTrackingUserGroupTagsResponse;
use crate::protocols::trackingextension::tracking_extension_protocol::TrackingExtensionProtocol;
use crate::protocols::trackingextension::tracking_extension_protocol::TrackingExtensionProtocolTrait;

struct TrackingExtensionProtocolImpl;

impl<T> TrackingExtensionProtocolTrait<T> for TrackingExtensionProtocolImpl {
    fn get_tracking_user_group(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetTrackingUserGroupRequest,
    ) -> Result<GetTrackingUserGroupResponse, Error> {
        login_required(&*ci)?;
        Ok(GetTrackingUserGroupResponse { usergroup: 0 })
    }

    fn get_tracking_user_group_tags(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<T>,
        _request: GetTrackingUserGroupTagsRequest,
    ) -> Result<GetTrackingUserGroupTagsResponse, Error> {
        login_required(&*ci)?;
        Ok(GetTrackingUserGroupTagsResponse {
            tags: vec![
                "GAME_START\0".to_string(),
                "ADVCLIENT_STOP\0".to_string(),
                "LEVEL_START\0".to_string(),
                "LEVEL_STOP\0".to_string(),
                "TX_SPEND\0".to_string(),
                "LOBBY_ENTER\0".to_string(),
                "LOBBY_EXITHOST\0".to_string(),
                "LOBBY_EXITCLIENT\0".to_string(),
                "AWARD_UNLOCK\0".to_string(),
                "GAME_LOC\0".to_string(),
                "PC_SPECS\0".to_string(),
                "UPLAY_PASS\0".to_string(),
                "MENU_PASS\0".to_string(),
                "UPLAY_ACCOUNT\0".to_string(),
                "UPLAY_ACCOUNT_MENU\0".to_string(),
            ],
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(TrackingExtensionProtocol::new(
        TrackingExtensionProtocolImpl,
    ))
}
