//! Implements the `GameSessionProtocolServer` for managing game sessions.

use std::sync::Arc;

use quazal::prudp::ClientRegistry;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use sc_bl_protocols::game_session_service::game_session_protocol::JoinSessionRequest;
use sc_bl_protocols::game_session_service::game_session_protocol::JoinSessionResponse;
use sc_bl_protocols::game_session_service::game_session_protocol::RemoveParticipantsRequest;
use sc_bl_protocols::game_session_service::game_session_protocol::RemoveParticipantsResponse;
use slog::Logger;

use crate::login_required;
use quazal::rmc::types::DateTime;

use crate::protocols::game_session_service::game_session_protocol::AbandonSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::AbandonSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::AddParticipantsRequest;
use crate::protocols::game_session_service::game_session_protocol::AddParticipantsResponse;
use crate::protocols::game_session_service::game_session_protocol::CreateSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::CreateSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::DeleteSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::DeleteSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocolServer;
use crate::protocols::game_session_service::game_session_protocol::GameSessionProtocolServerTrait;
use crate::protocols::game_session_service::game_session_protocol::AcceptInvitationRequest;
use crate::protocols::game_session_service::game_session_protocol::AcceptInvitationResponse;
use crate::protocols::game_session_service::game_session_protocol::CancelInvitationRequest;
use crate::protocols::game_session_service::game_session_protocol::CancelInvitationResponse;
use crate::protocols::game_session_service::game_session_protocol::DeclineInvitationRequest;
use crate::protocols::game_session_service::game_session_protocol::DeclineInvitationResponse;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationReceivedCountRequest;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationReceivedCountResponse;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationSentCountRequest;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationSentCountResponse;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationsReceivedRequest;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationsReceivedResponse;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationsSentRequest;
use crate::protocols::game_session_service::game_session_protocol::GetInvitationsSentResponse;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::LeaveSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsRequest;
use crate::protocols::game_session_service::game_session_protocol::RegisterUrLsResponse;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsRequest;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsResponse;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsWithParticipantsRequest;
use crate::protocols::game_session_service::game_session_protocol::ReportUnsuccessfulJoinSessionsRequest;
use crate::protocols::game_session_service::game_session_protocol::ReportUnsuccessfulJoinSessionsResponse;
use crate::protocols::game_session_service::game_session_protocol::SendInvitationRequest;
use crate::protocols::game_session_service::game_session_protocol::SendInvitationResponse;
use crate::protocols::game_session_service::game_session_protocol::SearchSessionsWithParticipantsResponse;
use crate::protocols::game_session_service::game_session_protocol::SplitSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::SplitSessionResponse;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionRequest;
use crate::protocols::game_session_service::game_session_protocol::UpdateSessionResponse;
use crate::protocols::game_session_service::types::GameSessionInvitationReceived;
use crate::protocols::game_session_service::types::GameSessionInvitationSent;
use crate::protocols::game_session_service::types::GameSessionKey;
use crate::protocols::game_session_service::types::GameSessionSearchResult;
use crate::protocols::game_session_service::types::GameSessionSearchWithParticipantsResult;
use crate::config::DebugConfig;
use quazal::rmc::basic::ToStream as _;
use crate::storage::Storage;

/// Quazal's notification event, hand-written rather than generated.
///
/// `sc_bl_protocols::protocol_foundation` does carry it, but that crate does not compile:
/// `types.rs` wants a `BufferTail` type quazal does not have, and
/// `remote_log_device_protocol.rs` is still `todo!()`. Not worth it for six fields.
///
/// Field order follows the DDL - it decides the bytes on the wire.
#[derive(Debug, quazal_macros::ToStream)]
pub struct NotificationEvent {
    pub pid_source: u32,
    pub ui_type: u32,
    pub ui_param_1: u32,
    pub ui_param_2: u32,
    pub str_param: String,
    pub ui_param_3: u32,
}

/// Quazal Rendez-Vous lists the notification protocol as 14, with `ProcessNotificationEvent`
/// as its method 1.
const NOTIFICATION_PROTOCOL_ID: u16 = 14;
const PROCESS_NOTIFICATION_EVENT: u32 = 1;

/// Implementation of the `GameSessionProtocolServerTrait` for handling game session operations.
struct GameSessionProtocolServerImpl {
    storage: Arc<Storage>,
    debug_config: Arc<DebugConfig>,
}

/// Property that tells the two rooms of a private match apart.
///
/// A host who opened a private match is in two sessions at once: the anteroom every client
/// opens on entering multiplayer, and the configured match room itself. Both name the same
/// host and live in the same session type - this property is the only thing distinguishing
/// them.
const PROPERTY_ROOM_KIND: u32 = 113;

/// Value of [`PROPERTY_ROOM_KIND`] for the anteroom every client opens for itself.
const ROOM_KIND_ANTEROOM: u32 = 1;

/// Picks the rooms a friend search should answer with.
///
/// The invited room is always in. Whether anything joins it depends on what kind of room it is:
///
///   * invited into a private match (`113 == 0`) - the host's current anteroom comes along, so
///     the client can fill both of its invitation slots
///   * invited into a lobby (`113 == 1`) - that room already *is* the anteroom, and the answer
///     is complete with it alone
///
/// At most one anteroom either way, because the client keeps exactly one slot for it. A host
/// can be left holding older anterooms it no longer advertises; sending those along means the
/// first one in the answer takes the slot, and the guest joins a room nobody is in. The
/// current one is the youngest, session ids being handed out in order - the same assumption
/// `find_host_sessions` already encodes in its `ORDER BY g.id DESC`.
fn rooms_for_friend_search<'a>(rooms: impl Iterator<Item = (u32, &'a str)>, invited_session_id: u32) -> Vec<u32> {
    let kinds: Vec<(u32, Option<u32>)> = rooms
        .map(|(id, attributes)| (id, attribute_value(attributes, PROPERTY_ROOM_KIND)))
        .collect();

    let invited_is_anteroom = kinds
        .iter()
        .any(|(id, kind)| *id == invited_session_id && *kind == Some(ROOM_KIND_ANTEROOM));

    let mut keep = vec![invited_session_id];
    if !invited_is_anteroom {
        let current_anteroom = kinds
            .iter()
            .filter(|(id, kind)| *kind == Some(ROOM_KIND_ANTEROOM) && *id != invited_session_id)
            .map(|(id, _)| *id)
            .max();
        keep.extend(current_anteroom);
    }
    keep
}

/// Reads one property out of an `"id => value;..."` attribute string.
///
/// Returns `None` if the property is absent or unparsable; callers treat both the same way.
fn attribute_value(attributes: &str, wanted_id: u32) -> Option<u32> {
    attributes.split(';').find_map(|part| {
        let (id, value) = part.split_once("=>")?;
        (id.trim().parse::<u32>().ok()? == wanted_id).then(|| value.trim().parse().ok())?
    })
}

/// Presents a private room as if its slots were public.
///
/// Attribute 3 counts public slots, attribute 4 private ones - a private match reserves all
/// eight as private and leaves 3 at zero. An invited client looking at that room sees no seat
/// it could take and backs out again, which is exactly what a public match with 3 => 4 does
/// not do. Only ever applied to the room a pending invitation points at.
pub(crate) fn advertise_private_slots_as_public(attributes: &str) -> String {
    let mut public_slots = 0u32;
    let mut private_slots = 0u32;
    for part in attributes.split(';') {
        let Some((id, value)) = part.split_once("=>") else { continue };
        let (Ok(id), Ok(value)) = (id.trim().parse::<u32>(), value.trim().parse::<u32>()) else { continue };
        match id {
            3 => public_slots = value,
            4 => private_slots = value,
            _ => {}
        }
    }
    if public_slots > 0 || private_slots == 0 {
        return attributes.to_string();
    }
    attributes
        .split(';')
        .map(|part| match part.split_once("=>").and_then(|(id, _)| id.trim().parse::<u32>().ok()) {
            Some(3) => format!("3 => {private_slots}"),
            Some(4) => "4 => 0".to_string(),
            _ => part.to_string(),
        })
        .collect::<Vec<_>>()
        .join(";")
}

/// Is this address unreachable from the outside?
fn is_unroutable(ip: std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local() || v4.is_unspecified(),
        std::net::IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified(),
    }
}

/// Replaces an unroutable address inside a `prudp:/address=<ip>;port=<n>;...` URL.
fn replace_url_address(url: &str, new_ip: std::net::IpAddr) -> String {
    url.split(';')
        .map(|part| {
            for prefix in ["prudp:/address=", "address="] {
                if let Some(rest) = part.strip_prefix(prefix) {
                    if rest.parse::<std::net::IpAddr>().map(is_unroutable).unwrap_or(false) {
                        return format!("{prefix}{new_ip}");
                    }
                    break;
                }
            }
            part.to_string()
        })
        .collect::<Vec<_>>()
        .join(";")
}

impl<CI> GameSessionProtocolServerTrait<CI> for GameSessionProtocolServerImpl {
    /// Handles the `CreateSession` request, creating a new game session.
    ///
    /// This function requires the client to be logged in. It extracts and stores
    /// game session attributes.
    fn create_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<CreateSessionResponse, Error> {
        info!(logger, "Client creates session: {:?}", request);
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;

        let attributes = request
            .game_session
            .attributes
            .0
            .into_iter()
            /*
            101 => map
            102 => game mode
            https://github.com/GitHubProUser67/MultiServer3/blob/dc189cfac27589356a52d2ad64c31c8a124c68f7/SpecializedServers/QuazalServer/RDVServices/DDL/Models/GameSessionService/GameSession.cs#L15
             */
            .map(|p| format!("{} => {}", p.id, p.value))
            .collect::<Vec<_>>()
            .join(";");
        let session_id = rmc_err!(
            self.storage.create_game_session(user_id, request.game_session.type_id, attributes),
            logger,
            "error creating game session"
        )?;
        Ok(CreateSessionResponse {
            game_session_key: GameSessionKey {
                type_id: request.game_session.type_id,
                session_id,
            },
        })
    }

    /// Handles the `UpdateSession` request, updating an existing game session.
    ///
    /// This function requires the client to be logged in. It updates game session attributes.
    fn update_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<UpdateSessionResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        info!(logger, "Client updates session: {:?}", request);
        let attributes = request
            .game_session_update
            .attributes
            .0
            .into_iter()
            /*
            101 => map
            102 => game mode
            https://github.com/GitHubProUser67/MultiServer3/blob/dc189cfac27589356a52d2ad64c31c8a124c68f7/SpecializedServers/QuazalServer/RDVServices/DDL/Models/GameSessionService/GameSession.cs#L15
             */
            .map(|p| format!("{} => {}", p.id, p.value))
            .collect::<Vec<_>>()
            .join(";");
        rmc_err!(
            self.storage.update_game_session(
                request.game_session_update.session_key.type_id,
                request.game_session_update.session_key.session_id,
                attributes,
            ),
            logger,
            "error updating game session"
        )?;
        Ok(UpdateSessionResponse)
    }

    /// Handles the `DeleteSession` request, deleting an existing game session.
    ///
    /// This function requires the client to be logged in.
    fn delete_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<DeleteSessionResponse, Error> {
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;
        if rmc_err!(
            self.storage
                .delete_game_session(user_id, request.game_session_key.type_id, request.game_session_key.session_id),
            logger,
            "error deleting session"
        )? != 1
        {
            warn!(logger, "Unexpected amount of sessions deleted");
        }
        Ok(DeleteSessionResponse)
    }

    /// Handles the `LeaveSession` request.
    ///
    /// This function requires the client to be logged in. It currently returns an empty response.
    fn leave_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: LeaveSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<LeaveSessionResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(LeaveSessionResponse)
    }

    /// Handles the `AddParticipants` request, adding participants to a game session.
    ///
    /// This function requires the client to be logged in.
    fn add_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<AddParticipantsResponse, Error> {
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;
        info!(logger, "Client adds participants: {:?}", request);
        rmc_err!(
            self.storage.add_participants(
                request.game_session_key.type_id,
                request.game_session_key.session_id,
                request.private_participant_ids.0.clone(),
                request.public_participant_ids.0.clone(),
            ),
            logger,
            "error adding participants"
        )?;

        // On the invitation route this call IS the join - the client never sends JoinSession
        // here (see split_session for the measurement). Retire the binding now, otherwise it
        // stays pending and misdirects the next split this player makes.
        let per_einladung = self
            .storage
            .consume_invite_for_session(user_id, request.game_session_key.type_id, request.game_session_key.session_id)
            .unwrap_or(false);
        if per_einladung {
            info!(
                logger,
                "User {user_id} joined session {} through an invitation (via AddParticipants)", request.game_session_key.session_id
            );
        }

        // Nudge the player who was just added, or they wait forever.
        //
        // Joining a PUBLIC match, the guest sends `JoinSession` and gets its answer. Joining a
        // PRIVATE one it deliberately sends nothing: `StateJoin::vf08` reads `[object+0x5A8]`,
        // sees the room is private, skips the join and sets `[session+0x42B]`. From there
        // `StateJoin::vf00` waits on `[session+0x42A]`, and the only thing that ever sets that
        // gate is `NetOnlineSessionServiceRdv::vf28`, on a notification with
        // `ui_type % 1000 == 3` whose `ui_param_1` carries the session id. Without it the game
        // sits at "creating game session..." indefinitely.
        //
        // Only nudge PRIVATE matches (room kind 0). Public rooms send `JoinSession` and need
        // no nudge, and a packet the client does not accept stalls the ordered PRUDP stream -
        // nudging during party setup breaks the party join that otherwise works.
        let ist_privater_raum = self
            .storage
            .game_session_attributes(request.game_session_key.type_id, request.game_session_key.session_id)
            .ok()
            .flatten()
            .is_some_and(|a| {
                a.split(';').any(|teil| {
                    let Some((id, wert)) = teil.split_once("=>") else { return false };
                    id.trim() == "113" && wert.trim() == "0"
                })
            });
        if self.debug_config.push_notifications && ist_privater_raum {
        for participant in request
            .private_participant_ids
            .0
            .iter()
            .chain(request.public_participant_ids.0.iter())
            .copied()
            // The caller is notified too, even though they added themselves.
            //
            // On the invitation route into a private match the guest is its own adder: it
            // splits out of its session and calls `AddParticipants` for itself. Skipping the
            // caller here means nobody gets notified at all on that route, and the guest's
            // state machine never moves. The party route is unaffected - there the host adds
            // the guest and is not in the list itself.
        {
            let event = NotificationEvent {
                pid_source: user_id,
                // 7003 = category 7, subtype 3. Both halves are required:
                //
                // `NotificationHandler::vf01` (0x0077DA30) returns early on
                // `ui_type / 1000 != 7` without calling any listener at all.
                // `NetOnlineSessionServiceRdv::vf28` (0x007BDFB0) then checks
                // `ui_type % 1000 == 3` and, on a matching `ui_param_1`, opens the
                // `[session+0x42A]` gate that `StateJoin::vf00` waits on in a private match.
                //
                // A bare 3 here dies in the first filter without a trace - no listener, no
                // error, nothing.
                ui_type: 7003,
                // The id of the player this is ABOUT - not the room id.
                //
                // `vf28` compares it against `[[service@slot_0x18]+0x40C]+8`:
                //   0x007BDFD3  mov esi,[eax+0x18]     service from table slot 0x18
                //   0x007BDFD7  mov ebx,[ecx+0xc]      ui_param_1
                //   0x007BDFFA  mov esi,[esi+0x40c]
                //   0x007BE004  cmp ebx,[esi+8]        mismatch leaves the gate shut
                //
                // Slot 0x18 holds the local player's object, not the room, which matches the
                // usual meaning of a Quazal `NotificationEvent`: `pid_source` is the sender,
                // `ui_param_1` the player concerned. Putting the room id here makes `vf28` run
                // through without ever writing the gate.
                ui_param_1: participant,
                // A session id as well, not a type_id.
                //
                // `NotificationHandler::vf01` compares it against `[[service+0x40C]+8]` - at
                // 0x0077DA77: `mov ecx,[esi+0x40c]` / `call 0x22b14e0` (returns `[this+8]`) /
                // `cmp eax,[ebp+8]`. `[+0x40C]` is the GameSession object and `+8` its session
                // id; on a mismatch the loop finds no service object and never calls `vf28`.
                ui_param_2: request.game_session_key.session_id,
                str_param: String::new(),
                ui_param_3: 0,
            };
            info!(logger, "Notifying {participant} about session {}: {event:?}", request.game_session_key.session_id);
            let payload = event.to_bytes();
            let reliable = self.debug_config.push_notifications_reliable;
            let mut notify = |target: &mut ClientInfo<CI>| {
                if let Err(e) = quazal::rmc::call_client(
                    logger,
                    _ctx,
                    _socket,
                    target,
                    NOTIFICATION_PROTOCOL_ID,
                    PROCESS_NOTIFICATION_EVENT,
                    payload.clone(),
                    reliable,
                ) {
                    error!(logger, "Notification to {participant} failed: {e}");
                }
            };

            // Serve the caller through their own `ci` rather than through the registry.
            //
            // `client_by_user_id` filters with `try_borrow`, and the caller's cell is borrowed
            // for the duration of their own request - they are never findable that way.
            // Swapping in `borrow_mut` is no way out either; it panics the service thread.
            if participant == user_id {
                // Invitation route only. Otherwise the HOST notifies itself as well when it
                // adds itself while opening the private match, opening its own `[0x42A]` gate
                // - a join gate on the host side is pointless at best.
                if per_einladung {
                    notify(&mut *ci);
                }
            } else if let Some(cell) = _client_registry.client_by_user_id(participant) {
                notify(&mut cell.borrow_mut());
            } else {
                info!(logger, "No connected client for {participant} - no notification sent");
            }
        }
        }

        Ok(AddParticipantsResponse)
    }

    /// Handles the `RemoveParticipants` request, removing participants from a game session.
    ///
    /// This function requires the client to be logged in.
    fn remove_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RemoveParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RemoveParticipantsResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        info!(logger, "Client removes participants: {:?}", request);
        rmc_err!(
            self.storage
                .remove_participants(request.game_session_key.type_id, request.game_session_key.session_id, request.participant_ids.0.clone(),),
            logger,
            "error removing participants"
        )?;
        Ok(RemoveParticipantsResponse)
    }

    /// Handles the `AbandonSession` request.
    ///
    /// This function requires the client to be logged in. It currently returns an empty response.
    fn abandon_session(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: AbandonSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<AbandonSessionResponse, Error> {
        // Ensure the client is logged in.
        login_required(&*ci)?;
        Ok(AbandonSessionResponse)
    }

    /// Handles the `RegisterUrLs` request, registering client URLs.
    ///
    /// This function requires the client to be logged in.
    fn register_urls(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RegisterUrLsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<RegisterUrLsResponse, Error> {
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;
        info!(logger, "Client registers urls: {:?}", request);

        // Matches run peer to peer, so the other player must be able to reach these
        // addresses. A client behind NAT only knows its own private address, which is
        // useless to the other side. Substitute something reachable:
        //   * client seen from the internet -> the address we actually observe
        //   * client on our own network     -> the server's configured public address,
        //     because a peer behind the same NAT as this server can never be observed
        //     under a public address
        let observed = ci.address().ip();
        let reachable = if is_unroutable(observed) {
            // The public address is configured per service. This protocol runs on the
            // secure service, whose settings carry `storage_host`; `secure_server_addr`
            // lives on the authentication service and is not visible here. Try both so
            // the lookup keeps working if the layout changes.
            ["storage_host", "secure_server_addr"]
                .iter()
                .find_map(|key| _ctx.settings.get(*key))
                .and_then(|s| s.rsplit_once(':').map_or(s.as_str(), |(host, _)| host).parse::<std::net::IpAddr>().ok())
                .filter(|ip| !is_unroutable(*ip))
        } else {
            Some(observed)
        };

        let urls: Vec<String> = request
            .station_urls
            .0
            .into_iter()
            .map(|su| {
                let url = su.to_string();
                match reachable {
                    Some(ip) => {
                        let rewritten = replace_url_address(&url, ip);
                        if rewritten != url {
                            info!(logger, "rewrote station url {} -> {}", url, rewritten);
                        }
                        rewritten
                    }
                    None => url,
                }
            })
            .collect();

        rmc_err!(self.storage.register_urls(user_id, urls), logger, "error adding participants")?;
        Ok(RegisterUrLsResponse)
    }

    /// Handles the `SearchSessionsWithParticipants` request, searching for game sessions.
    ///
    /// This function requires the client to be logged in.
    fn search_sessions_with_participants(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsWithParticipantsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SearchSessionsWithParticipantsResponse, Error> {
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;
        info!(logger, "Searches for sessions with {request:?}");

        // Resolved before the search, because the answer depends on it.
        let invited = rmc_err!(
            self.storage.find_pending_invited_session(user_id, request.game_session_type_id),
            logger,
            "error resolving invited room"
        )?;

        // The searching client is put into the room it was invited to before we answer.
        //
        // Why: on the invitation route the game hands this search an output list at
        // manager+0xA8 and then walks it to notify its listeners (0x0085B7C0). That list stays
        // empty in our setup, so nothing is notified and the join never starts. The one thing
        // our answer does differently from a working search is the participant list - it names
        // only the host, and the client that is asking does not appear in it. Adding the caller
        // first makes the room look the way it would if the join had already been accepted.
        if let Some(room) = invited.as_ref() {
            if !room.participants.iter().any(|p| p.user_id == user_id) {
                info!(logger, "Adding {user_id} to invited room {} before answering the search", room.session_id);
                rmc_err!(
                    self.storage.add_participants(room.session_type, room.session_id, vec![user_id], vec![]),
                    logger,
                    "error adding the invited player"
                )?;
            }
        }

        let mut sessions = self
            .storage
            .search_sessions_with_participants(request.game_session_type_id, request.participant_ids.0.as_slice())
            .map_err(|e| {
                error!(logger, "Error searching game sessions: {e}");
                Error::InternalError
            })?;

        // Answer with the invited room, and - if that is a match room - the host's anteroom.
        //
        // The client does not pick one at random - it sorts them. For every session in this
        // answer, `nsOnlinePresence::StateAcceptInviteFindSession::vf00` reads
        // `PROPERTY_ROOM_KIND` and files the session into one of two slots of its invitation
        // object, filling each slot only while it is still empty:
        //
        //     113 == 0  ->  match slot
        //     113 == 1  ->  anteroom slot
        //     absent    ->  neither slot
        //
        // `nsOnlinePresence::StateAcceptInvite::vf08` then picks, and the match slot wins: a
        // filled match slot sends the guest straight into the private session, an anteroom
        // slot alone only joins the host's lobby, and two empty slots make the client open a
        // party of its own instead of joining anything.
        //
        // Narrowing this answer to the invited room therefore withholds exactly the session
        // the client needs to fill its second slot, and no invitation can reach a match that
        // way. Sending too much is just as wrong: a second anteroom takes the one slot there
        // is, and the guest joins a room nobody is in. `rooms_for_friend_search` picks exactly
        // what the invitation calls for.
        if let Some(invited) = invited {
            if sessions.iter().any(|session| session.session_id == invited.session_id) {
                let keep = rooms_for_friend_search(
                    sessions.iter().map(|session| (session.session_id, session.attributes.as_str())),
                    invited.session_id,
                );
                sessions.retain(|session| keep.contains(&session.session_id));

                let rooms: Vec<String> = sessions
                    .iter()
                    .map(|session| {
                        format!(
                            "{} (113 => {:?})",
                            session.session_id,
                            attribute_value(&session.attributes, PROPERTY_ROOM_KIND)
                        )
                    })
                    .collect();
                info!(
                    logger,
                    "Friend search of {user_id}: invited room {} plus anteroom -> {}",
                    invited.session_id,
                    rooms.join(", ")
                );

                // Slot counts stay untouched. A private match carries `3 => 0; 4 => 8`: no
                // public seats, eight private ones - and a private seat is exactly what an
                // invited player takes. Rewriting that to `3 => 8; 4 => 0`, as
                // `advertise_private_slots_as_public` does, fakes public seats instead; the
                // server already adds the invited player to the private list above.
            }
        }

        info!(logger, "Found sessions: {sessions:#?}");

        Ok(SearchSessionsWithParticipantsResponse {
            search_results: sessions
                .into_iter()
                .map(|session| {
                    let host = session.participants.iter().find(|p| p.user_id == session.creator_id).unwrap();
                    GameSessionSearchWithParticipantsResult {
                        game_session_search_result: GameSessionSearchResult {
                            session_key: GameSessionKey {
                                type_id: session.session_type,
                                session_id: session.session_id,
                            },
                            host_pid: host.user_id,
                            host_urls: host.station_urls.clone().try_into().unwrap(),
                            attributes: session.attributes.as_str().parse().unwrap(),
                        },
                        participant_ids: session.participants.into_iter().map(|p| p.user_id).collect(),
                    }
                })
                .collect(),
        })
    }

    /// Handles the `SplitSession` request.
    ///
    /// This function requires the client to be logged in. It currently returns a placeholder response.
    fn split_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SplitSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SplitSessionResponse, Error> {
        // Ensure the client is logged in.
        let user_id = login_required(&*ci)?;
        let key = request.game_session_key;

        let migrated = rmc_err!(
            self.storage.split_game_session(user_id, key.type_id, key.session_id),
            logger,
            "error splitting game session"
        )?;
        Ok(SplitSessionResponse {
            game_session_key_migrated: match migrated {
                Some(session_id) => GameSessionKey {
                    type_id: key.type_id,
                    session_id,
                },
                None => key,
            },
        })
    }

    /// Records why a client failed to join a session.
    ///
    /// This is the only place where the client tells us the reason. The default stub answered
    /// with `UnimplementedMethod` and dropped the payload, which carries an error category and
    /// an error code per room - the one piece of diagnosis available when a join silently
    /// falls apart.
    fn report_unsuccessful_join_sessions(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ReportUnsuccessfulJoinSessionsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<ReportUnsuccessfulJoinSessionsResponse, Error> {
        let user_id = login_required(&*ci)?;
        for gescheitert in &request.unsuccessful_join_sessions.0 {
            warn!(
                logger,
                "Join failed: {user_id} did not get into session {} (type {}) - category {}, code {:#010x}",
                gescheitert.session_key.session_id,
                gescheitert.session_key.type_id,
                gescheitert.error_category,
                gescheitert.error_code
            );
        }
        Ok(ReportUnsuccessfulJoinSessionsResponse)
    }

    /// Handles the `SendInvitation` request, inviting players into a session.
    ///
    /// This is the entry point of a private lobby: the host creates a session that is not meant
    /// to be found by matchmaking and hands out invitations instead. One row is stored per
    /// recipient; the recipients pick them up with `GetInvitationsReceived`.
    ///
    /// This function requires the client to be logged in.
    fn send_invitation(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SendInvitationRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SendInvitationResponse, Error> {
        let user_id = login_required(&*ci)?;
        info!(logger, "Client invites into session: {:?}", request);
        let invitation = request.invitation;
        rmc_err!(
            self.storage.add_game_session_invites(
                invitation.session_key.type_id,
                invitation.session_key.session_id,
                user_id,
                &invitation.recipient_pids.0,
                &invitation.message,
            ),
            logger,
            "error storing session invitations"
        )?;
        Ok(SendInvitationResponse)
    }

    /// Handles the `GetInvitationReceivedCount` request.
    ///
    /// The client polls this before fetching the list itself, so it stays cheap.
    ///
    /// This function requires the client to be logged in.
    fn get_invitation_received_count(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationReceivedCountRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetInvitationReceivedCountResponse, Error> {
        let user_id = login_required(&*ci)?;
        let count = rmc_err!(
            self.storage.count_game_session_invites_received(user_id, request.game_session_type_id),
            logger,
            "error counting received invitations"
        )?;
        Ok(GetInvitationReceivedCountResponse { count })
    }

    /// Handles the `GetInvitationSentCount` request.
    ///
    /// This function requires the client to be logged in.
    fn get_invitation_sent_count(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationSentCountRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetInvitationSentCountResponse, Error> {
        let user_id = login_required(&*ci)?;
        let count = rmc_err!(
            self.storage.count_game_session_invites_sent(user_id, request.game_session_type_id),
            logger,
            "error counting sent invitations"
        )?;
        Ok(GetInvitationSentCountResponse { count })
    }

    /// Handles the `GetInvitationsReceived` request, listing pending invitations for this player.
    ///
    /// This function requires the client to be logged in.
    fn get_invitations_received(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationsReceivedRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetInvitationsReceivedResponse, Error> {
        let user_id = login_required(&*ci)?;
        let invites = rmc_err!(
            self.storage.list_game_session_invites_received(
                user_id,
                request.game_session_type_id,
                request.result_range.offset,
                request.result_range.size,
            ),
            logger,
            "error listing received invitations"
        )?;
        info!(logger, "{} invitation(s) pending for user {}", invites.len(), user_id);
        Ok(GetInvitationsReceivedResponse {
            invitations: invites
                .into_iter()
                .map(|invite| GameSessionInvitationReceived {
                    session_key: GameSessionKey {
                        type_id: invite.session_type,
                        session_id: invite.session_id,
                    },
                    sender_pid: invite.sender,
                    message: invite.message,
                    creation_time: DateTime(invite.created_at),
                })
                .collect::<Vec<_>>()
                .into(),
        })
    }

    /// Handles the `GetInvitationsSent` request, listing invitations this player handed out.
    ///
    /// This function requires the client to be logged in.
    fn get_invitations_sent(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetInvitationsSentRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetInvitationsSentResponse, Error> {
        let user_id = login_required(&*ci)?;
        let invites = rmc_err!(
            self.storage
                .list_game_session_invites_sent(user_id, request.game_session_type_id, request.result_range.offset, request.result_range.size,),
            logger,
            "error listing sent invitations"
        )?;
        Ok(GetInvitationsSentResponse {
            invitations: invites
                .into_iter()
                .map(|invite| GameSessionInvitationSent {
                    session_key: GameSessionKey {
                        type_id: invite.session_type,
                        session_id: invite.session_id,
                    },
                    recipient_pid: invite.receiver,
                    message: invite.message,
                    creation_time: DateTime(invite.created_at),
                })
                .collect::<Vec<_>>()
                .into(),
        })
    }

    /// Handles the `AcceptInvitation` request.
    ///
    /// Accepting consumes the invitation and puts the player into the session's participant
    /// list. It is deliberately added as a *private* participant: that is what an invited
    /// player is, and it keeps the distinction the client already makes in `AddParticipants`.
    ///
    /// The actual connection is then established peer to peer - just like on the matchmaking
    /// path, where `JoinSession` also does not have to do anything, because the client already
    /// holds the host's station URLs.
    ///
    /// This function requires the client to be logged in.
    fn accept_invitation(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AcceptInvitationRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<AcceptInvitationResponse, Error> {
        let user_id = login_required(&*ci)?;
        let invitation = request.game_session_invitation;
        info!(logger, "User {} accepts invitation: {:?}", user_id, invitation);

        rmc_err!(
            self.storage
                .add_participants(invitation.session_key.type_id, invitation.session_key.session_id, vec![user_id], vec![]),
            logger,
            "error joining session after accepting invitation"
        )?;
        rmc_err!(
            self.storage.delete_game_session_invite(
                invitation.session_key.type_id,
                invitation.session_key.session_id,
                invitation.sender_pid,
                user_id,
            ),
            logger,
            "error clearing accepted invitation"
        )?;
        Ok(AcceptInvitationResponse)
    }

    /// Handles the `DeclineInvitation` request, dropping the invitation without joining.
    ///
    /// This function requires the client to be logged in.
    fn decline_invitation(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeclineInvitationRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<DeclineInvitationResponse, Error> {
        let user_id = login_required(&*ci)?;
        let invitation = request.game_session_invitation;
        info!(logger, "User {} declines invitation: {:?}", user_id, invitation);
        rmc_err!(
            self.storage.delete_game_session_invite(
                invitation.session_key.type_id,
                invitation.session_key.session_id,
                invitation.sender_pid,
                user_id,
            ),
            logger,
            "error clearing declined invitation"
        )?;
        Ok(DeclineInvitationResponse)
    }

    /// Handles the `CancelInvitation` request, withdrawing an invitation as the sender.
    ///
    /// This function requires the client to be logged in.
    fn cancel_invitation(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CancelInvitationRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<CancelInvitationResponse, Error> {
        let user_id = login_required(&*ci)?;
        let invitation = request.game_session_invitation;
        info!(logger, "User {} cancels invitation: {:?}", user_id, invitation);
        rmc_err!(
            self.storage.delete_game_session_invite(
                invitation.session_key.type_id,
                invitation.session_key.session_id,
                user_id,
                invitation.recipient_pid,
            ),
            logger,
            "error cancelling invitation"
        )?;
        Ok(CancelInvitationResponse)
    }

    /// Handles the `SearchSessions` request, the plain session search without a participant filter.
    ///
    /// `SearchSessionsWithParticipants` above answers "which session are these players in";
    /// this one answers "which sessions of this type are open". The client's own sessions are
    /// filtered out - it has no use for finding itself.
    ///
    /// This function requires the client to be logged in.
    fn search_sessions(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SearchSessionsResponse, Error> {
        let user_id = login_required(&*ci)?;
        info!(logger, "Client searches for sessions: {:?}", request);

        let sessions = rmc_err!(
            self.storage.search_sessions(request.game_session_query.type_id, Some(user_id)),
            logger,
            "error searching game sessions"
        )?;
        info!(logger, "Found sessions: {sessions:#?}");

        Ok(SearchSessionsResponse {
            search_results: sessions
                .into_iter()
                .filter_map(|session| {
                    // A session whose creator is not among its own participants has not
                    // registered its host URLs yet; there is nothing a client could connect to,
                    // so it is left out rather than reported with an empty address.
                    let host = session.participants.iter().find(|p| p.user_id == session.creator_id)?;
                    Some(GameSessionSearchResult {
                        session_key: GameSessionKey {
                            type_id: session.session_type,
                            session_id: session.session_id,
                        },
                        host_pid: host.user_id,
                        host_urls: host.station_urls.clone().try_into().ok()?,
                        attributes: session.attributes.as_str().parse().ok()?,
                    })
                })
                .collect::<Vec<_>>()
                .into(),
        })
    }

    /// Handles the `JoinSession` request.
    ///
    /// This function currently returns an empty response.
    fn join_session(
        &self,
        logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: JoinSessionRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<JoinSessionResponse, Error> {
        let user_id = login_required(&*ci)?;
        // The response carries no fields - the client already has the host's addresses from
        // the search result and builds the connection itself. What this call is good for is
        // knowing that the join happened: only now may a pending invitation be retired, so
        // that a client repeating its search in the meantime still finds the room.
        let key = request.game_session_key;
        if rmc_err!(
            self.storage.consume_invite_for_session(user_id, key.type_id, key.session_id),
            logger,
            "error consuming invitation"
        )? {
            info!(logger, "User {user_id} joined session {} through an invitation", key.session_id);
        }
        Ok(JoinSessionResponse)
    }
}

/// Creates a new boxed `GameSessionProtocolServer` instance.
///
/// This function is typically used to register the game session protocol
/// with the server's protocol dispatcher.
pub fn new_protocol<T: 'static>(storage: Arc<Storage>, debug_config: Arc<DebugConfig>) -> Box<dyn Protocol<T>> {
    Box::new(GameSessionProtocolServer::new(GameSessionProtocolServerImpl { storage, debug_config }))
}

#[cfg(test)]
mod tests {
    use quazal::rmc::basic::ToStream as _;
    use quazal::rmc::Request;

    use super::attribute_value;
    use super::PROPERTY_ROOM_KIND;
    use super::rooms_for_friend_search;
    use super::NotificationEvent;
    use super::NOTIFICATION_PROTOCOL_ID;
    use super::PROCESS_NOTIFICATION_EVENT;

    /// Builds the notification exactly as `add_participants` does and takes it apart again.
    ///
    /// This cannot tell whether 14 is the right protocol id - only the client knows that - but
    /// it catches everything that can go wrong on our side: wrong field order, broken framing,
    /// a lost call id.
    #[test]
    fn notification_event_is_well_formed() {
        let event = NotificationEvent {
            pid_source: 1006,
            ui_type: 3,
            ui_param_1: 234,
            ui_param_2: 1,
            str_param: String::new(),
            ui_param_3: 0,
        };
        let payload = event.to_bytes();

        let request = Request {
            protocol_id: NOTIFICATION_PROTOCOL_ID,
            call_id: 0x8000_0001,
            method_id: PROCESS_NOTIFICATION_EVENT,
            parameters: payload.clone(),
        };
        let bytes = request.to_bytes();
        let decoded = Request::from_bytes(&bytes).expect("request must round-trip");

        assert_eq!(decoded.protocol_id, 14, "notification protocol");
        assert_eq!(decoded.method_id, 1, "ProcessNotificationEvent");
        assert_eq!(decoded.call_id, 0x8000_0001);
        assert_eq!(decoded.parameters, payload);

        // The first three fields decide the gate on the client side. They have to sit there
        // as u32 in this order, or the game reads nonsense.
        assert_eq!(&payload[0..4], &1006u32.to_le_bytes(), "pid_source first");
        assert_eq!(&payload[4..8], &3u32.to_le_bytes(), "then ui_type");
        assert_eq!(&payload[8..12], &234u32.to_le_bytes(), "then ui_param_1");
    }

    const MATCH_ROOM_ATTRIBUTES: &str = "113 => 0;3 => 0;4 => 8;102 => 7";
    const ANTEROOM_ATTRIBUTES: &str = "113 => 1;3 => 8;4 => 0";

    /// Invited into a private match: the match room and the anteroom, so the client can fill
    /// both of its invitation slots.
    #[test]
    fn match_invitation_answers_with_match_room_and_anteroom() {
        let rooms = [(471, ANTEROOM_ATTRIBUTES), (472, MATCH_ROOM_ATTRIBUTES)];

        let keep = rooms_for_friend_search(rooms.into_iter(), 472);

        assert!(keep.contains(&472), "the invited match room");
        assert!(keep.contains(&471), "the anteroom the client needs alongside it");
        assert_eq!(keep.len(), 2);
    }

    /// Invited into a lobby: that room already is the anteroom, so it goes alone.
    ///
    /// Sending a second anteroom would take the one slot the client has for it.
    #[test]
    fn lobby_invitation_answers_with_that_room_alone() {
        let rooms = [(477, ANTEROOM_ATTRIBUTES), (478, ANTEROOM_ATTRIBUTES)];

        let keep = rooms_for_friend_search(rooms.into_iter(), 478);

        assert_eq!(keep, vec![478]);
    }

    /// Anterooms the host stopped advertising must not displace the current one.
    ///
    /// A host can be left holding older anterooms; the current one is the youngest.
    #[test]
    fn stale_anterooms_do_not_displace_the_current_one() {
        let rooms = [
            (477, ANTEROOM_ATTRIBUTES),
            (478, ANTEROOM_ATTRIBUTES),
            (484, MATCH_ROOM_ATTRIBUTES),
        ];

        let keep = rooms_for_friend_search(rooms.into_iter(), 484);

        assert!(keep.contains(&484), "the invited match room");
        assert!(keep.contains(&478), "the youngest anteroom is the current one");
        assert!(!keep.contains(&477), "the stale anteroom stays out");
    }

    /// A host without an anteroom yields just the invited room, not an empty answer.
    #[test]
    fn match_invitation_without_anteroom_still_answers() {
        let rooms = [(484, MATCH_ROOM_ATTRIBUTES)];

        assert_eq!(rooms_for_friend_search(rooms.into_iter(), 484), vec![484]);
    }

    /// A missing or unparsable property reads as absent rather than as a value.
    #[test]
    fn attribute_value_reports_absence() {
        assert_eq!(attribute_value("113 => 0;3 => 8", PROPERTY_ROOM_KIND), Some(0));
        assert_eq!(attribute_value("3 => 8;4 => 0", PROPERTY_ROOM_KIND), None);
        assert_eq!(attribute_value("", PROPERTY_ROOM_KIND), None);
    }
}
