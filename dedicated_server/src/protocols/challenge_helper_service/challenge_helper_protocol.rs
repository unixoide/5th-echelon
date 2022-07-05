#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]
use super::types::*;
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
enum ChallengeHelperProtocolMethod {
    GenerateMyFriendChallenges = 1u32,
    GenerateFriendChallenges = 2u32,
    GetOnlineChallenges = 3u32,
}
#[derive(Debug, FromStream)]
pub struct GenerateMyFriendChallengesRequest {
    pub friend_pi_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GenerateMyFriendChallengesResponse {
    pub result: quazal::rmc::types::QList<FriendChallenge>,
}
#[derive(Debug, FromStream)]
pub struct GenerateFriendChallengesRequest {
    pub target_pid: u32,
    pub friend_pi_ds: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, ToStream)]
pub struct GenerateFriendChallengesResponse {
    pub result: quazal::rmc::types::QList<FriendChallenge>,
}
#[derive(Debug, FromStream)]
pub struct GetOnlineChallengesRequest;
#[derive(Debug, ToStream)]
pub struct GetOnlineChallengesResponse {
    pub online_challenges: quazal::rmc::types::QList<OnlineChallenge>,
}
pub struct ChallengeHelperProtocol<T: ChallengeHelperProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: ChallengeHelperProtocolTrait<CI>, CI> ChallengeHelperProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: ChallengeHelperProtocolTrait<CI>, CI> Protocol<CI> for ChallengeHelperProtocol<T, CI> {
    fn id(&self) -> u16 {
        105u16
    }
    fn name(&self) -> String {
        "ChallengeHelperProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        3u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = ChallengeHelperProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(ChallengeHelperProtocolMethod::GenerateMyFriendChallenges) => {
                let req = GenerateMyFriendChallengesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.generate_my_friend_challenges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ChallengeHelperProtocolMethod::GenerateFriendChallenges) => {
                let req = GenerateFriendChallengesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.generate_friend_challenges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ChallengeHelperProtocolMethod::GetOnlineChallenges) => {
                let req = GetOnlineChallengesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_online_challenges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        ChallengeHelperProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait ChallengeHelperProtocolTrait<CI> {
    fn generate_my_friend_challenges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GenerateMyFriendChallengesRequest,
    ) -> Result<GenerateMyFriendChallengesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ChallengeHelperProtocol",
            stringify!(generate_my_friend_challenges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn generate_friend_challenges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GenerateFriendChallengesRequest,
    ) -> Result<GenerateFriendChallengesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ChallengeHelperProtocol",
            stringify!(generate_friend_challenges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_online_challenges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetOnlineChallengesRequest,
    ) -> Result<GetOnlineChallengesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ChallengeHelperProtocol",
            stringify!(get_online_challenges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
