use quazal::prudp::ClientRegistry;
use quazal::rmc::types::DateTime;
use quazal::rmc::types::QList;
use quazal::rmc::types::Variant;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use crate::login_required;
use crate::protocols::user_storage::types::ContentProperty;
use crate::protocols::user_storage::types::UserContent;
use crate::protocols::user_storage::types::UserContentKey;
use crate::protocols::user_storage::types::UserContentURL;
use crate::protocols::user_storage::user_storage_protocol::GetContentUrlRequest;
use crate::protocols::user_storage::user_storage_protocol::GetContentUrlResponse;
use crate::protocols::user_storage::user_storage_protocol::SearchContentsRequest;
use crate::protocols::user_storage::user_storage_protocol::SearchContentsResponse;
use crate::protocols::user_storage::user_storage_protocol::UserStorageProtocolServer;
use crate::protocols::user_storage::user_storage_protocol::UserStorageProtocolServerTrait;

struct UserStorageProtocolServerImpl;

impl<CI> UserStorageProtocolServerTrait<CI> for UserStorageProtocolServerImpl {
    fn search_contents(
        &self,
        _logger: &Logger,
        _ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchContentsRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<SearchContentsResponse, Error> {
        #![allow(clippy::unreadable_literal)]

        login_required(&*ci)?;
        if request.query.type_id == 0x8000_0002 {
            let search_results = QList(vec![UserContent {
                key: UserContentKey {
                    type_id: 0x8000_0002,
                    content_id: 1,
                },
                pid: 0x0000_045f,
                properties: QList(vec![
                    ContentProperty {
                        id: 6,
                        value: Variant::I64(0x274),
                    },
                    ContentProperty {
                        id: 4,
                        value: Variant::DateTime(DateTime(0x1f768edbd6)),
                    },
                    ContentProperty {
                        id: 5,
                        value: Variant::DateTime(DateTime(0x1f768f13f9)),
                    },
                    ContentProperty {
                        id: 7,
                        value: Variant::String("A6E32CFD0C2B2CFFE2D0C785830B7C49".to_string()),
                    },
                ]),
            }]);
            Ok(SearchContentsResponse { search_results })
        } else {
            Ok(SearchContentsResponse {
                search_results: QList::default(),
            })
        }
    }

    fn get_content_url(
        &self,
        _logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        _request: GetContentUrlRequest,
        _client_registry: &ClientRegistry<CI>,
        _socket: &std::net::UdpSocket,
    ) -> Result<GetContentUrlResponse, Error> {
        login_required(&*ci)?;
        let protocol = ctx
            .settings
            .get("content_protocol")
            .map_or("http://", String::as_str)
            .to_owned();
        let host = ctx
            .settings
            .get("storage_host")
            .expect("missing storage_host setting")
            .to_owned();
        let path = ctx
            .settings
            .get("storage_path")
            .expect("missing storage_path setting")
            .to_owned();

        Ok(GetContentUrlResponse {
            download_info: UserContentURL { protocol, host, path },
        })
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(UserStorageProtocolServer::new(UserStorageProtocolServerImpl))
}
