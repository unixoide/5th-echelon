#![allow(clippy::wildcard_imports, clippy::module_name_repetitions)]

use std::convert::TryFrom;

use num_enum::TryFromPrimitive;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;

use super::types::*;
use crate::protocols::game_session_service::types::GameSessionQuery;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum GameSessionExProtocolMethod {
    SearchSessions = 1u32,
}
#[derive(Debug, FromStream)]
pub struct SearchSessionsRequest {
    pub game_session_query: GameSessionQuery,
}
#[derive(Debug, ToStream)]
pub struct SearchSessionsResponse {
    pub search_results: quazal::rmc::types::QList<GameSessionSearchResultEx>,
}
pub struct GameSessionExProtocol<T: GameSessionExProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: GameSessionExProtocolTrait<CI>, CI> GameSessionExProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: GameSessionExProtocolTrait<CI>, CI> Protocol<CI> for GameSessionExProtocol<T, CI> {
    fn id(&self) -> u16 {
        123u16
    }
    fn name(&self) -> String {
        "GameSessionExProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        1u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = GameSessionExProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(GameSessionExProtocolMethod::SearchSessions) => {
                let req = SearchSessionsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.search_sessions(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        GameSessionExProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait GameSessionExProtocolTrait<CI> {
    fn search_sessions(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SearchSessionsRequest,
    ) -> Result<SearchSessionsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "GameSessionExProtocol",
            stringify!(search_sessions)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
