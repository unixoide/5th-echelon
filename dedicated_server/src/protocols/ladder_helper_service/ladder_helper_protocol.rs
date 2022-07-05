use num_enum::TryFromPrimitive;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::Error;
use quazal::rmc::Protocol;
use quazal::rmc::Request;
use quazal::ClientInfo;
use quazal::Context;
use slog::Logger;
use std::convert::TryFrom;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum LadderHelperProtocolMethod {
    GetUnixUtc = 1u32,
    AreLaddersAvailableInCountry = 2u32,
    CheckLadderIsRunning = 3u32,
    ClearLadderLeaderboard = 4u32,
}
#[derive(Debug, FromStream)]
pub struct GetUnixUtcRequest;
#[derive(Debug, ToStream)]
pub struct GetUnixUtcResponse {
    pub time: u32,
}
#[derive(Debug, FromStream)]
pub struct AreLaddersAvailableInCountryRequest;
#[derive(Debug, ToStream)]
pub struct AreLaddersAvailableInCountryResponse {
    pub allowed: bool,
}
#[derive(Debug, FromStream)]
pub struct CheckLadderIsRunningRequest {
    pub start_time: u32,
    pub end_time: u32,
}
#[derive(Debug, ToStream)]
pub struct CheckLadderIsRunningResponse {
    pub running: bool,
}
#[derive(Debug, FromStream)]
pub struct ClearLadderLeaderboardRequest {
    pub stat_set: i32,
}
#[derive(Debug, ToStream)]
pub struct ClearLadderLeaderboardResponse {
    pub success: bool,
}
pub struct LadderHelperProtocol<T: LadderHelperProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: LadderHelperProtocolTrait<CI>, CI> LadderHelperProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: LadderHelperProtocolTrait<CI>, CI> Protocol<CI> for LadderHelperProtocol<T, CI> {
    fn id(&self) -> u16 {
        107u16
    }
    fn name(&self) -> String {
        "LadderHelperProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        4u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = LadderHelperProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(LadderHelperProtocolMethod::GetUnixUtc) => {
                let req = GetUnixUtcRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_unix_utc(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(LadderHelperProtocolMethod::AreLaddersAvailableInCountry) => {
                let req = AreLaddersAvailableInCountryRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self
                    .0
                    .are_ladders_available_in_country(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(LadderHelperProtocolMethod::CheckLadderIsRunning) => {
                let req = CheckLadderIsRunningRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.check_ladder_is_running(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(LadderHelperProtocolMethod::ClearLadderLeaderboard) => {
                let req = ClearLadderLeaderboardRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.clear_ladder_leaderboard(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        LadderHelperProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait LadderHelperProtocolTrait<CI> {
    fn get_unix_utc(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetUnixUtcRequest,
    ) -> Result<GetUnixUtcResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LadderHelperProtocol",
            stringify!(get_unix_utc)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn are_ladders_available_in_country(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AreLaddersAvailableInCountryRequest,
    ) -> Result<AreLaddersAvailableInCountryResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LadderHelperProtocol",
            stringify!(are_ladders_available_in_country)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn check_ladder_is_running(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CheckLadderIsRunningRequest,
    ) -> Result<CheckLadderIsRunningResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LadderHelperProtocol",
            stringify!(check_ladder_is_running)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn clear_ladder_leaderboard(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ClearLadderLeaderboardRequest,
    ) -> Result<ClearLadderLeaderboardResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "LadderHelperProtocol",
            stringify!(clear_ladder_leaderboard)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
