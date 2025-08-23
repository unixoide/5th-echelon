use quazal::prudp::ClientRegistry;
use quazal::rmc::basic::ToStream;
use quazal::rmc::types::Variant;
use quazal::rmc::Protocol;
use quazal::Context;

use crate::login_required;

#[allow(clippy::module_name_repetitions)]
pub struct OverlordCoreProtocol;

impl<T> Protocol<T> for OverlordCoreProtocol {
    fn id(&self) -> u16 {
        5003
    }

    fn name(&self) -> String {
        "OverlordCoreProtocol".into()
    }

    fn num_methods(&self) -> u32 {
        1
    }

    fn handle(
        &self,
        _logger: &slog::Logger,
        _ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: &quazal::rmc::Request,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> std::result::Result<Vec<u8>, quazal::rmc::Error> {
        #[allow(clippy::enum_glob_use)]
        use Variant::*;

        login_required(&*ci)?;
        if request.method_id != 1 {
            return Err(quazal::rmc::Error::UnknownMethod);
        }

        let cfg: Vec<(std::string::String, Variant)> = vec![
            ("VER_SERVER_STAGE".to_owned(), I64(10)),
            ("2593515025".to_owned(), I64(1)),
            ("2626349757".to_owned(), I64(1)),
            ("NC_CONNECTION_TICKET_TIMEOUT".to_owned(), F64(10.0)),
            ("NC_CONNECTION_HANDSHAKING_TIMEOUT".to_owned(), F64(5.0)),
            ("314486871".to_owned(), I64(1)),
            ("3759634546".to_owned(), I64(1)),
            ("NC_MAIN_PORT_RANGE".to_owned(), I64(32)),
            ("2449304206".to_owned(), I64(1)),
            ("OVERLORD_VERSION".to_owned(), String("0.8.0.0".into())),
            ("2685611256".to_owned(), I64(1)),
            ("3376331533".to_owned(), I64(1)),
            ("VER_SERVER_CODE".to_owned(), I64(3007)),
            ("FILESERVICE_UNLOCKS_UPLOAD_ENABLED".to_owned(), I64(1)),
            ("2739184075".to_owned(), I64(1)),
            ("2942435614".to_owned(), I64(1)),
            ("SN_FRIENDCHALLENGES_MAX_READ_INTERVAL".to_owned(), F64(900.0)),
            ("NC_CONNECTION_JOIN_TIMEOUT".to_owned(), F64(15.0)),
            ("1027449109".to_owned(), I64(1)),
            ("4175756708".to_owned(), I64(1)),
            ("NC_CONNECTION_INACTIVITY_THRESHOLD".to_owned(), F64(8.0)),
            ("FILESERVICE_ADMIN_RDVID".to_owned(), I64(1119)),
            ("SN_WEEKLYCHALLENGES_ENABLE".to_owned(), I64(1)),
            ("1597953054".to_owned(), I64(1)),
            ("2156388390".to_owned(), I64(1)),
            ("3785106560".to_owned(), I64(1)),
            ("STATS_WRITE_INTERVAL".to_owned(), F64(1.0)),
            ("3835530207".to_owned(), I64(1)),
            ("1492891464".to_owned(), I64(1)),
            ("SN_DAILYCHALLENGES_ENABLE".to_owned(), I64(1)),
            ("2505766166".to_owned(), I64(1)),
            ("11866509".to_owned(), I64(1)),
            ("SN_FRIENDCHALLENGES_ENABLE".to_owned(), I64(1)),
            ("FILESERVICE_UNLOCKS_UPLOAD_INTERVAL".to_owned(), F64(3600.0)),
            ("2524360986".to_owned(), I64(1)),
            ("721797971".to_owned(), I64(1)),
            ("1525666223".to_owned(), I64(1)),
            ("COMMUNITYEVENT_DOUBLECASH".to_owned(), I64(0)),
            ("UPLAY_MAX_RANK_LIMITED_MODE".to_owned(), I64(5)),
            ("SN_GONEDARKCHALLENGES_ENABLE".to_owned(), I64(1)),
            ("_OSDK_VERSION".to_owned(), String("1.4.16.32918".into())),
            ("NC_MAIN_PORT".to_owned(), I64(13000)),
            ("COMMUNITYEVENT_DOUBLEXP".to_owned(), I64(0)),
            ("3804368594".to_owned(), I64(1)),
            ("NC_CONNECTION_CLOSING_TIMEOUT".to_owned(), F64(2.0)),
            ("NC_CONNECTION_ESTABLISHED_TIMEOUT".to_owned(), F64(10.0)),
        ];

        /* Alternative with hashmap:
         let cfg: std::collections::HashMap<std::string::String, Variant> = [
            ...
        ].iter()
        .cloned()
        .collect();
        */
        Ok(cfg.to_bytes())
    }

    fn method_name(&self, method_id: u32) -> Option<String> {
        if method_id == 1 {
            Some("fetch_config".into())
        } else {
            None
        }
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(OverlordCoreProtocol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method1() {
        let logger = slog::Logger::root(slog::Discard, slog::o!());
        let ctx = quazal::Context::default();
        let mut ci = quazal::ClientInfo::<()>::new("127.0.0.1:2".parse().unwrap());
        ci.user_id = Some(1);
        let prot = OverlordCoreProtocol;
        let request = quazal::rmc::Request {
            protocol_id: Protocol::<()>::id(&prot),
            call_id: 1,
            method_id: 1,
            parameters: vec![],
        };
        let resp = prot
            .handle(
                &logger,
                &ctx,
                &mut ci,
                &request,
                &ClientRegistry::default(),
                &std::net::UdpSocket::bind("127.0.0.1:12345").unwrap(),
            )
            .unwrap();

        let expected = include_bytes!("../../testdata/overlord_core_config.bin");
        assert_eq!(expected.len(), resp.len());
        assert_eq!(
            expected.as_slice(),
            resp.as_slice(),
            "{}",
            diff::slice(expected.as_ref(), resp.as_slice())
                .into_iter()
                .map(|diff| match diff {
                    diff::Result::Left(l) => format!("-{l:02x}"),
                    diff::Result::Both(l, _) => format!(" {l:02x}"),
                    diff::Result::Right(r) => format!("+{r:02x}"),
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
