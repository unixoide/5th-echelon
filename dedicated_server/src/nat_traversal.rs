use quazal::prudp::packet::PacketType;
use quazal::prudp::packet::QPacket;
use quazal::prudp::packet::StreamType;
use quazal::prudp::packet::VPort;
use quazal::prudp::ClientRegistry;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::nat_traversal::nat_traversal_protocol::InitiateProbeRequest;
use crate::protocols::nat_traversal::nat_traversal_protocol::NatTraversalProtocolMethod;
use crate::protocols::nat_traversal::nat_traversal_protocol::NatTraversalProtocolServer;
use crate::protocols::nat_traversal::nat_traversal_protocol::NatTraversalProtocolServerTrait;
use crate::protocols::nat_traversal::nat_traversal_protocol::RequestProbeInitiationExtRequest;
use crate::protocols::nat_traversal::nat_traversal_protocol::RequestProbeInitiationExtResponse;
use crate::protocols::nat_traversal::nat_traversal_protocol::NAT_TRAVERSAL_PROTOCOL_ID;

struct NatTraversalProtocolServerImpl;

impl<T> NatTraversalProtocolServerTrait<T> for NatTraversalProtocolServerImpl {
    fn request_probe_initiation_ext(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<T>,
        request: RequestProbeInitiationExtRequest,
        client_registry: &ClientRegistry<T>,
        socket: &std::net::UdpSocket,
    ) -> Result<RequestProbeInitiationExtResponse, Error> {
        let _user_id = login_required(&*ci)?;
        info!(logger, "Probe initiation requested: {request:?}");

        for url in request.url_target_list.iter() {
            let Some(conn_id) = url.params.get("RVCID") else {
                warn!(logger, "{url} doesn't include RVCID");
                continue;
            };
            let Ok(conn_id) = conn_id.parse() else {
                warn!(logger, "{url} doesn't include valid RVCID");
                continue;
            };

            let Some(target) = client_registry.client_by_connection_id(conn_id) else {
                warn!(logger, "No client found for RVCID {conn_id:?}");
                continue;
            };

            let payload = Request {
                protocol_id: NAT_TRAVERSAL_PROTOCOL_ID,
                call_id: rand::random(),
                method_id: NatTraversalProtocolMethod::InitiateProbe as u32,
                parameters: InitiateProbeRequest {
                    url_station_to_probe: request.url_station_to_probe.clone(),
                }
                .to_bytes(),
            }
            .to_bytes();

            // let payload = Request {
            //     protocol_id: 14,
            //     call_id: rand::random(),
            //     method_id: 1,
            //     parameters: b"\x0c#\x1d\x00[\x1b\x00\x0015\x19\x00\xbd\xb3\x98\x02\x01\x00\x00\x01\x00\x00\x00".to_vec(),
            // }
            // .to_bytes();

            let addr = { *target.borrow().address() };
            info!(logger, "Sending probe to {url} ({addr})\n{payload:x?}");

            let qpacket = QPacket {
                source: VPort {
                    port: 1,
                    stream_type: StreamType::RVSec,
                },
                destination: VPort {
                    port: 15,
                    stream_type: StreamType::RVSec,
                },
                packet_type: PacketType::Data,
                payload,
                ..Default::default()
            };
            quazal::prudp::send_request(
                logger,
                ctx,
                &addr,
                socket,
                qpacket,
                &mut *target.borrow_mut(),
            )
            .unwrap();
        }
        Ok(RequestProbeInitiationExtResponse)
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(NatTraversalProtocolServer::new(
        NatTraversalProtocolServerImpl,
    ))
}
