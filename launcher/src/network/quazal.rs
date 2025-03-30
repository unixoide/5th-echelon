use std::time::Duration;

use quazal::prudp::packet::QPacket;
use quazal::Context;
use server_api::misc::misc_client::MiscClient;
use server_api::misc::TestP2pRequest;
use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use tokio::net::UdpSocket;
use tonic::transport::Channel;

use super::Error;
use crate::config::QUAZAL_DEFAULT_LOCAL_PORT;
use crate::config::QUAZAL_DEFAULT_PORT;

pub async fn test_quazal_login(server: &str, username: &str, password: &str) -> Result<(), Error> {
    let ctx = quazal::Context::splinter_cell_blacklist();

    let Ok(res) = tokio::time::timeout(Duration::from_secs(5), async {
        let socket = quazal_setup(server).await?;
        let mut quazal = Quazal {
            ctx,
            socket,
            session_id: rand::random(),
            signature: 0,
            sequence: 0,
        };
        quazal.syn().await?;
        quazal.connect().await?;
        quazal.login(username, password).await
    })
    .await
    else {
        return Err(Error::TimedOut);
    };

    res
}

struct Quazal {
    ctx: Context,
    socket: UdpSocket,
    session_id: u8,
    signature: u32,
    sequence: u16,
}

async fn quazal_setup(server: &str) -> std::io::Result<UdpSocket> {
    let socket = tokio::net::UdpSocket::bind(format!("0.0.0.0:{QUAZAL_DEFAULT_LOCAL_PORT}")).await?;
    socket.connect(format!("{server}:{QUAZAL_DEFAULT_PORT}")).await?;
    Ok(socket)
}

impl Quazal {
    async fn send(&mut self, mut packet: QPacket) -> Result<(), Error> {
        packet.session_id = self.session_id;
        packet.signature = self.signature;
        packet.sequence = self.sequence;

        self.sequence += 1;

        self.socket.send(&packet.to_bytes(&self.ctx)).await?;
        Ok(())
    }
    async fn send_ack(&mut self, mut packet: QPacket) -> Result<(), Error> {
        packet.session_id = self.session_id;
        packet.signature = self.signature;
        packet.sequence = 0;

        self.socket.send(&packet.to_bytes(&self.ctx)).await?;
        Ok(())
    }

    async fn syn(&mut self) -> Result<u32, Error> {
        use quazal::prudp::packet::PacketFlag;
        use quazal::prudp::packet::PacketType;
        use quazal::prudp::packet::QPacket;
        use quazal::prudp::packet::StreamType;
        use quazal::prudp::packet::VPort;

        let syn_pkt = QPacket {
            source: VPort {
                port: 15,
                stream_type: StreamType::RVSec,
            },
            destination: VPort {
                port: 1,
                stream_type: StreamType::RVSec,
            },
            packet_type: PacketType::Syn,
            flags: PacketFlag::NeedAck.into(),
            conn_signature: Some(0),
            ..Default::default()
        };
        self.send(syn_pkt).await?;

        let mut buf = vec![0u8; 4096];

        let n = self.socket.recv(&mut buf).await?;
        let (syn_ack_pkt, _size) =
            QPacket::from_bytes(&self.ctx, &buf[..n]).map_err(|e| std::io::Error::other(e.to_string()))?;

        if syn_ack_pkt.packet_type != PacketType::Syn || !syn_ack_pkt.flags.contains(PacketFlag::Ack) {
            return Err(Error::IO(std::io::Error::other("invalid syn ack")));
        }

        if syn_ack_pkt.conn_signature.is_none() {
            return Err(Error::IO(std::io::Error::other("missing connection signature")));
        }

        self.signature = syn_ack_pkt.conn_signature.unwrap();
        Ok(self.signature)
    }

    async fn connect(&mut self) -> Result<(), Error> {
        use quazal::prudp::packet::PacketFlag;
        use quazal::prudp::packet::PacketType;
        use quazal::prudp::packet::QPacket;
        use quazal::prudp::packet::StreamType;
        use quazal::prudp::packet::VPort;

        let connect_pkt = QPacket {
            source: VPort {
                port: 15,
                stream_type: StreamType::RVSec,
            },
            destination: VPort {
                port: 1,
                stream_type: StreamType::RVSec,
            },
            packet_type: PacketType::Connect,
            flags: PacketFlag::NeedAck.into(),
            conn_signature: Some(rand::random()),
            ..Default::default()
        };
        self.send(connect_pkt).await?;

        let mut buf = vec![0u8; 4096];

        let n = self.socket.recv(&mut buf).await?;
        let (conn_ack_pkt, _size) = QPacket::from_bytes(&self.ctx, &buf[..n])?;

        if conn_ack_pkt.packet_type != PacketType::Connect || !conn_ack_pkt.flags.contains(PacketFlag::Ack) {
            return Err(Error::IO(std::io::Error::other("invalid connect ack")));
        }
        Ok(())
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {
        use quazal::prudp::packet::PacketFlag;
        use quazal::prudp::packet::PacketType;
        use quazal::prudp::packet::QPacket;
        use quazal::prudp::packet::StreamType;
        use quazal::prudp::packet::VPort;
        use quazal::rmc::basic::ToStream as _;
        use quazal::rmc::types::Any;
        use quazal::rmc::Packet;
        use quazal::rmc::Request;
        use sc_bl_protocols::authentication_foundation::ticket_granting_protocol::LoginExRequest;
        use sc_bl_protocols::authentication_foundation::ticket_granting_protocol::TicketGrantingProtocolMethod;
        use sc_bl_protocols::authentication_foundation::ticket_granting_protocol::TICKET_GRANTING_PROTOCOL_ID;
        use sc_bl_protocols::ubi_authentication::types::UbiAuthenticationLoginCustomData;

        let parameters = LoginExRequest {
            str_user_name: username.to_string(),
            o_extra_data: Any::new(
                "UbiAuthenticationLoginCustomData".to_string(),
                UbiAuthenticationLoginCustomData {
                    data: quazal::rmc::types::Data,
                    user_name: username.to_string(),
                    online_key: "AAAA-BBBB-CCCC".to_string(),
                    password: password.to_string(),
                }
                .to_bytes(),
            ),
        }
        .to_bytes();

        let login_rmc_pkt = Packet::Request(Request {
            protocol_id: TICKET_GRANTING_PROTOCOL_ID,
            call_id: 10,
            method_id: TicketGrantingProtocolMethod::LoginEx as u32,
            parameters,
        });

        let login_prudp_pkt = QPacket {
            source: VPort {
                port: 15,
                stream_type: StreamType::RVSec,
            },
            destination: VPort {
                port: 1,
                stream_type: StreamType::RVSec,
            },
            packet_type: PacketType::Data,
            flags: PacketFlag::NeedAck | PacketFlag::Reliable,
            fragment_id: Some(0),
            payload: login_rmc_pkt.to_bytes(),
            ..Default::default()
        };
        self.send(login_prudp_pkt).await?;

        let mut buf = vec![0u8; 4096];

        let n = self.socket.recv(&mut buf).await?;
        let (data_ack_pkt, _size) = QPacket::from_bytes(&self.ctx, &buf[..n])?;

        if data_ack_pkt.packet_type != PacketType::Data || !data_ack_pkt.flags.contains(PacketFlag::Ack) {
            return Err(Error::IO(std::io::Error::other("invalid data ack")));
        }

        let mut buf = vec![0u8; 4096];

        let n = self.socket.recv(&mut buf).await?;
        let (login_resp, _size) = QPacket::from_bytes(&self.ctx, &buf[..n])?;

        if login_resp.packet_type != PacketType::Data || !login_resp.flags.contains(PacketFlag::HasSize) {
            return Err(Error::IO(std::io::Error::other("invalid response")));
        }

        // ack the response
        self.send_ack(QPacket {
            source: VPort {
                port: 15,
                stream_type: StreamType::RVSec,
            },
            destination: VPort {
                port: 1,
                stream_type: StreamType::RVSec,
            },
            packet_type: PacketType::Data,
            flags: PacketFlag::Ack.into(),
            fragment_id: Some(0),
            ..Default::default()
        })
        .await?;

        // disconnect
        self.send(QPacket {
            source: VPort {
                port: 15,
                stream_type: StreamType::RVSec,
            },
            destination: VPort {
                port: 1,
                stream_type: StreamType::RVSec,
            },
            packet_type: PacketType::Disconnect,
            fragment_id: Some(0),
            ..Default::default()
        })
        .await?;

        let n = self.socket.recv(&mut buf).await?;
        let (disco_ack_pkt, _size) = QPacket::from_bytes(&self.ctx, &buf[..n])?;
        if disco_ack_pkt.packet_type != PacketType::Disconnect || !disco_ack_pkt.flags.contains(PacketFlag::Ack) {
            return Err(Error::IO(std::io::Error::other("invalid disco ack")));
        }

        let resp = Packet::from_bytes(&login_resp.payload)?;
        if let Packet::Response(resp) = resp {
            resp.result
                .map_err(|e| Error::Rmc(quazal::rmc::Error::from_error_code(e.error_code).unwrap()))?;
        } else {
            return Err(Error::IO(std::io::Error::other("invalid rmc response")));
        }

        Ok(())
    }
}

pub async fn test_p2p(api_url: String, username: &str, password: &str) -> Result<(), Error> {
    let Ok(mut client) = UsersClient::connect(api_url.clone()).await else {
        return Err(Error::ConnectionFailed);
    };

    let resp = match client
        .login(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .await
    {
        Ok(resp) => resp,
        Err(status) => {
            if matches!(status.code(), tonic::Code::Unauthenticated) {
                return Err(Error::InvalidPassword);
            } else {
                return Err(Error::SendingRequestFailed);
            }
        }
    };

    let resp = resp.into_inner();
    if !resp.error.is_empty() {
        return Err(Error::ServerFailure(resp.error));
    }
    let token: tonic::metadata::MetadataValue<tonic::metadata::Ascii> = resp.token.parse().unwrap();
    let Ok(channel) = Channel::from_shared(api_url).unwrap().connect().await else {
        return Err(Error::ConnectionFailed);
    };
    let mut client = MiscClient::with_interceptor(channel, move |mut req: tonic::Request<_>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let udp_client_handle = tokio::spawn(async {
        let socket = UdpSocket::bind("0.0.0.0:13000").await?;
        let mut buf = vec![0u8; 4096];
        let (n, addr) = socket.recv_from(&mut buf).await?;
        let buf = &buf[..n];
        let Some(challenge) = buf.strip_prefix(b"P2P Test - ") else {
            return Err(Error::IO(std::io::Error::other("invalid challenge")));
        };
        socket.send_to(challenge, addr).await?;
        Ok::<_, Error>(challenge.to_vec())
    });

    let rpc_client_handle = tokio::spawn(async move {
        let challenge: [u8; 32] = rand::random();
        let resp = client
            .test_p2p(TestP2pRequest {
                challenge: challenge.to_vec(),
            })
            .await?;
        Ok::<_, Error>(resp.into_inner().challenge)
    });

    match tokio::try_join!(udp_client_handle, rpc_client_handle) {
        Ok((Err(udp_err), Ok(_))) => {
            return Err(Error::P2P(Box::new(udp_err)));
        }
        Ok((Ok(_), Err(rpc_err))) => {
            return Err(Error::P2P(Box::new(rpc_err)));
        }
        Ok((Err(_udp_err), Err(rpc_err))) => {
            return Err(Error::P2P(Box::new(rpc_err)));
        }
        Ok((Ok(udp_challenge), Ok(rpc_challenge))) => {
            if udp_challenge != rpc_challenge {
                return Err(Error::ChallengeMismatch);
            }
        }
        Err(e) => {
            return Err(Error::IO(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())));
        }
    }

    Ok(())
}
