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
use crate::protocols::challenge_helper_service::types::FriendChallenge;
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum ClanHelperProtocolMethod {
    SetClanInfo = 1u32,
    AddPidToClid = 2u32,
    RemoveMemberByPid = 3u32,
    DisbandEntireClid = 4u32,
    GetClanInfoByPid = 5u32,
    GetClanInfoByClid = 6u32,
    GetMemberListByPid = 7u32,
    GetMemberListByClid = 8u32,
    GenerateClanChallenges = 9u32,
}
#[derive(Debug, FromStream)]
pub struct SetClanInfoRequest {
    pub new_info: ClanInfo,
}
#[derive(Debug, ToStream)]
pub struct SetClanInfoResponse;
#[derive(Debug, FromStream)]
pub struct AddPidToClidRequest {
    pub target_pid: u32,
    pub clid: u32,
}
#[derive(Debug, ToStream)]
pub struct AddPidToClidResponse;
#[derive(Debug, FromStream)]
pub struct RemoveMemberByPidRequest {
    pub target_pid: u32,
}
#[derive(Debug, ToStream)]
pub struct RemoveMemberByPidResponse;
#[derive(Debug, FromStream)]
pub struct DisbandEntireClidRequest {
    pub target_clid: u32,
}
#[derive(Debug, ToStream)]
pub struct DisbandEntireClidResponse;
#[derive(Debug, FromStream)]
pub struct GetClanInfoByPidRequest {
    pub target_pid: u32,
}
#[derive(Debug, ToStream)]
pub struct GetClanInfoByPidResponse {
    pub clan_info: ClanInfo,
}
#[derive(Debug, FromStream)]
pub struct GetClanInfoByClidRequest {
    pub target_clid: u32,
}
#[derive(Debug, ToStream)]
pub struct GetClanInfoByClidResponse {
    pub clan_info: ClanInfo,
}
#[derive(Debug, FromStream)]
pub struct GetMemberListByPidRequest {
    pub target_pid: u32,
}
#[derive(Debug, ToStream)]
pub struct GetMemberListByPidResponse {
    pub members: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, FromStream)]
pub struct GetMemberListByClidRequest {
    pub target_clid: u32,
}
#[derive(Debug, ToStream)]
pub struct GetMemberListByClidResponse {
    pub members: quazal::rmc::types::QList<u32>,
}
#[derive(Debug, FromStream)]
pub struct GenerateClanChallengesRequest {
    pub target_pid: u32,
}
#[derive(Debug, ToStream)]
pub struct GenerateClanChallengesResponse {
    pub result: quazal::rmc::types::QList<FriendChallenge>,
}
pub struct ClanHelperProtocol<T: ClanHelperProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: ClanHelperProtocolTrait<CI>, CI> ClanHelperProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData)
    }
}
impl<T: ClanHelperProtocolTrait<CI>, CI> Protocol<CI> for ClanHelperProtocol<T, CI> {
    fn id(&self) -> u16 {
        106u16
    }
    fn name(&self) -> String {
        "ClanHelperProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        9u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = ClanHelperProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(ClanHelperProtocolMethod::SetClanInfo) => {
                let req = SetClanInfoRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.set_clan_info(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::AddPidToClid) => {
                let req = AddPidToClidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.add_pid_to_clid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::RemoveMemberByPid) => {
                let req = RemoveMemberByPidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.remove_member_by_pid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::DisbandEntireClid) => {
                let req = DisbandEntireClidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.disband_entire_clid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::GetClanInfoByPid) => {
                let req = GetClanInfoByPidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_clan_info_by_pid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::GetClanInfoByClid) => {
                let req = GetClanInfoByClidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_clan_info_by_clid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::GetMemberListByPid) => {
                let req = GetMemberListByPidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_member_list_by_pid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::GetMemberListByClid) => {
                let req = GetMemberListByClidRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_member_list_by_clid(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(ClanHelperProtocolMethod::GenerateClanChallenges) => {
                let req = GenerateClanChallengesRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.generate_clan_challenges(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        ClanHelperProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait ClanHelperProtocolTrait<CI> {
    fn set_clan_info(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: SetClanInfoRequest,
    ) -> Result<SetClanInfoResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(set_clan_info)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn add_pid_to_clid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: AddPidToClidRequest,
    ) -> Result<AddPidToClidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(add_pid_to_clid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn remove_member_by_pid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RemoveMemberByPidRequest,
    ) -> Result<RemoveMemberByPidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(remove_member_by_pid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn disband_entire_clid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DisbandEntireClidRequest,
    ) -> Result<DisbandEntireClidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(disband_entire_clid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_clan_info_by_pid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetClanInfoByPidRequest,
    ) -> Result<GetClanInfoByPidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(get_clan_info_by_pid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_clan_info_by_clid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetClanInfoByClidRequest,
    ) -> Result<GetClanInfoByClidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(get_clan_info_by_clid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_member_list_by_pid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetMemberListByPidRequest,
    ) -> Result<GetMemberListByPidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(get_member_list_by_pid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_member_list_by_clid(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetMemberListByClidRequest,
    ) -> Result<GetMemberListByClidResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(get_member_list_by_clid)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn generate_clan_challenges(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GenerateClanChallengesRequest,
    ) -> Result<GenerateClanChallengesResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "ClanHelperProtocol",
            stringify!(generate_clan_challenges)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
