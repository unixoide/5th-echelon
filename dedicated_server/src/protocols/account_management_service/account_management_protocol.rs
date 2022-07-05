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
enum AccountManagementProtocolMethod {
    CreateAccount = 1u32,
    DeleteAccount = 2u32,
    DisableAccount = 3u32,
    ChangePassword = 4u32,
    TestCapability = 5u32,
    GetName = 6u32,
    GetAccountData = 7u32,
    GetPrivateData = 8u32,
    GetPublicData = 9u32,
    GetMultiplePublicData = 10u32,
    UpdateAccountName = 11u32,
    UpdateAccountEmail = 12u32,
    UpdateCustomData = 13u32,
    FindByNameRegex = 14u32,
    UpdateAccountExpiryDate = 15u32,
    UpdateAccountEffectiveDate = 16u32,
    UpdateStatus = 17u32,
    GetStatus = 18u32,
    GetLastConnectionStats = 19u32,
    ResetPassword = 20u32,
    CreateAccountWithCustomData = 21u32,
    RetrieveAccount = 22u32,
    UpdateAccount = 23u32,
    ChangePasswordByGuest = 24u32,
    FindByNameLike = 25u32,
    CustomCreateAccount = 26u32,
    LookupOrCreateAccount = 27u32,
    CreateAccountEx = 28u32,
    DisconnectPrincipal = 29u32,
    DisconnectAllPrincipals = 30u32,
}
#[derive(Debug, FromStream)]
pub struct CreateAccountRequest {
    pub str_principal_name: String,
    pub str_key: String,
    pub ui_groups: u32,
    pub str_email: String,
}
#[derive(Debug, ToStream)]
pub struct CreateAccountResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct DeleteAccountRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct DeleteAccountResponse;
#[derive(Debug, FromStream)]
pub struct DisableAccountRequest {
    pub id_principal: u32,
    pub dt_until: quazal::rmc::types::DateTime,
    pub str_message: String,
}
#[derive(Debug, ToStream)]
pub struct DisableAccountResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct ChangePasswordRequest {
    pub str_new_key: String,
}
#[derive(Debug, ToStream)]
pub struct ChangePasswordResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct TestCapabilityRequest {
    pub ui_capability: u32,
}
#[derive(Debug, ToStream)]
pub struct TestCapabilityResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct GetNameRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct GetNameResponse {
    pub str_name: String,
}
#[derive(Debug, FromStream)]
pub struct GetAccountDataRequest;
#[derive(Debug, ToStream)]
pub struct GetAccountDataResponse {
    pub return_value: quazal::rmc::types::QResult,
    pub o_account_data: AccountData,
}
#[derive(Debug, FromStream)]
pub struct GetPrivateDataRequest;
#[derive(Debug, ToStream)]
pub struct GetPrivateDataResponse {
    pub return_value: bool,
    pub o_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, FromStream)]
pub struct GetPublicDataRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct GetPublicDataResponse {
    pub return_value: bool,
    pub o_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, FromStream)]
pub struct GetMultiplePublicDataRequest {
    pub lst_principals: Vec<u32>,
}
#[derive(Debug, ToStream)]
pub struct GetMultiplePublicDataResponse {
    pub return_value: bool,
    pub o_data: Vec<quazal::rmc::types::Any<quazal::rmc::types::Data, String>>,
}
#[derive(Debug, FromStream)]
pub struct UpdateAccountNameRequest {
    pub str_name: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateAccountNameResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct UpdateAccountEmailRequest {
    pub str_name: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateAccountEmailResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct UpdateCustomDataRequest {
    pub o_public_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
    pub o_private_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct UpdateCustomDataResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct FindByNameRegexRequest {
    pub ui_groups: u32,
    pub str_regex: String,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct FindByNameRegexResponse {
    pub plst_accounts: Vec<BasicAccountInfo>,
}
#[derive(Debug, FromStream)]
pub struct UpdateAccountExpiryDateRequest {
    pub id_principal: u32,
    pub dt_expiry: quazal::rmc::types::DateTime,
    pub str_expired_message: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateAccountExpiryDateResponse;
#[derive(Debug, FromStream)]
pub struct UpdateAccountEffectiveDateRequest {
    pub id_principal: u32,
    pub dt_effective_from: quazal::rmc::types::DateTime,
    pub str_not_effective_message: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateAccountEffectiveDateResponse;
#[derive(Debug, FromStream)]
pub struct UpdateStatusRequest {
    pub str_status: String,
}
#[derive(Debug, ToStream)]
pub struct UpdateStatusResponse;
#[derive(Debug, FromStream)]
pub struct GetStatusRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct GetStatusResponse {
    pub str_status: String,
}
#[derive(Debug, FromStream)]
pub struct GetLastConnectionStatsRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct GetLastConnectionStatsResponse {
    pub dt_last_session_login: quazal::rmc::types::DateTime,
    pub dt_last_session_logout: quazal::rmc::types::DateTime,
    pub dt_current_session_login: quazal::rmc::types::DateTime,
}
#[derive(Debug, FromStream)]
pub struct ResetPasswordRequest;
#[derive(Debug, ToStream)]
pub struct ResetPasswordResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct CreateAccountWithCustomDataRequest {
    pub str_principal_name: String,
    pub str_key: String,
    pub ui_groups: u32,
    pub str_email: String,
    pub o_public_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
    pub o_private_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct CreateAccountWithCustomDataResponse;
#[derive(Debug, FromStream)]
pub struct RetrieveAccountRequest;
#[derive(Debug, ToStream)]
pub struct RetrieveAccountResponse {
    pub o_account_data: AccountData,
    pub o_public_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
    pub o_private_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, FromStream)]
pub struct UpdateAccountRequest {
    pub str_key: String,
    pub str_email: String,
    pub o_public_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
    pub o_private_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct UpdateAccountResponse;
#[derive(Debug, FromStream)]
pub struct ChangePasswordByGuestRequest {
    pub str_principal_name: String,
    pub str_email: String,
    pub str_key: String,
}
#[derive(Debug, ToStream)]
pub struct ChangePasswordByGuestResponse;
#[derive(Debug, FromStream)]
pub struct FindByNameLikeRequest {
    pub ui_groups: u32,
    pub str_like: String,
    pub result_range: quazal::rmc::types::ResultRange,
}
#[derive(Debug, ToStream)]
pub struct FindByNameLikeResponse {
    pub plst_accounts: Vec<BasicAccountInfo>,
}
#[derive(Debug, FromStream)]
pub struct CustomCreateAccountRequest {
    pub str_principal_name: String,
    pub str_key: String,
    pub ui_groups: u32,
    pub str_email: String,
    pub o_auth_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct CustomCreateAccountResponse {
    pub pid: u32,
}
#[derive(Debug, FromStream)]
pub struct LookupOrCreateAccountRequest {
    pub str_principal_name: String,
    pub str_key: String,
    pub ui_groups: u32,
    pub str_email: String,
    pub o_auth_data: quazal::rmc::types::Any<quazal::rmc::types::Data, String>,
}
#[derive(Debug, ToStream)]
pub struct LookupOrCreateAccountResponse {
    pub pid: u32,
}
#[derive(Debug, FromStream)]
pub struct CreateAccountExRequest {
    pub principal_type: i8,
    pub str_principal_name: String,
    pub str_key: String,
    pub ui_groups: u32,
    pub str_email: String,
    pub context: u64,
}
#[derive(Debug, ToStream)]
pub struct CreateAccountExResponse {
    pub return_value: quazal::rmc::types::QResult,
}
#[derive(Debug, FromStream)]
pub struct DisconnectPrincipalRequest {
    pub id_principal: u32,
}
#[derive(Debug, ToStream)]
pub struct DisconnectPrincipalResponse {
    pub return_value: bool,
}
#[derive(Debug, FromStream)]
pub struct DisconnectAllPrincipalsRequest;
#[derive(Debug, ToStream)]
pub struct DisconnectAllPrincipalsResponse {
    pub return_value: bool,
}
pub struct AccountManagementProtocol<T: AccountManagementProtocolTrait<CI>, CI>(
    T,
    ::std::marker::PhantomData<CI>,
);
impl<T: AccountManagementProtocolTrait<CI>, CI> AccountManagementProtocol<T, CI> {
    pub fn new(implementation: T) -> Self {
        Self(implementation, ::std::marker::PhantomData::default())
    }
}
impl<T: AccountManagementProtocolTrait<CI>, CI> Protocol<CI> for AccountManagementProtocol<T, CI> {
    fn id(&self) -> u16 {
        25u16
    }
    fn name(&self) -> String {
        "AccountManagementProtocol".to_string()
    }
    fn num_methods(&self) -> u32 {
        30u32
    }
    fn handle(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: &Request,
    ) -> Result<Vec<u8>, Error> {
        let method = AccountManagementProtocolMethod::try_from(request.method_id).ok();
        match method {
            None => Err(Error::UnknownMethod),
            Some(AccountManagementProtocolMethod::CreateAccount) => {
                let req = CreateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.create_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::DeleteAccount) => {
                let req = DeleteAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.delete_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::DisableAccount) => {
                let req = DisableAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.disable_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::ChangePassword) => {
                let req = ChangePasswordRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.change_password(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::TestCapability) => {
                let req = TestCapabilityRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.test_capability(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetName) => {
                let req = GetNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_name(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetAccountData) => {
                let req = GetAccountDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_account_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetPrivateData) => {
                let req = GetPrivateDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_private_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetPublicData) => {
                let req = GetPublicDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_public_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetMultiplePublicData) => {
                let req = GetMultiplePublicDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_multiple_public_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateAccountName) => {
                let req = UpdateAccountNameRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account_name(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateAccountEmail) => {
                let req = UpdateAccountEmailRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account_email(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateCustomData) => {
                let req = UpdateCustomDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_custom_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::FindByNameRegex) => {
                let req = FindByNameRegexRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.find_by_name_regex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateAccountExpiryDate) => {
                let req = UpdateAccountExpiryDateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account_expiry_date(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateAccountEffectiveDate) => {
                let req = UpdateAccountEffectiveDateRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account_effective_date(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateStatus) => {
                let req = UpdateStatusRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_status(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetStatus) => {
                let req = GetStatusRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_status(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::GetLastConnectionStats) => {
                let req = GetLastConnectionStatsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.get_last_connection_stats(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::ResetPassword) => {
                let req = ResetPasswordRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.reset_password(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::CreateAccountWithCustomData) => {
                let req = CreateAccountWithCustomDataRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.create_account_with_custom_data(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::RetrieveAccount) => {
                let req = RetrieveAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.retrieve_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::UpdateAccount) => {
                let req = UpdateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.update_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::ChangePasswordByGuest) => {
                let req = ChangePasswordByGuestRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.change_password_by_guest(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::FindByNameLike) => {
                let req = FindByNameLikeRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.find_by_name_like(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::CustomCreateAccount) => {
                let req = CustomCreateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.custom_create_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::LookupOrCreateAccount) => {
                let req = LookupOrCreateAccountRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.lookup_or_create_account(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::CreateAccountEx) => {
                let req = CreateAccountExRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.create_account_ex(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::DisconnectPrincipal) => {
                let req = DisconnectPrincipalRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.disconnect_principal(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
            Some(AccountManagementProtocolMethod::DisconnectAllPrincipals) => {
                let req = DisconnectAllPrincipalsRequest::from_bytes(&request.parameters)?;
                debug!(logger, "Request: {:?}", req);
                let resp = self.0.disconnect_all_principals(logger, ctx, ci, req);
                debug!(logger, "Response: {:?}", resp);
                Ok(resp?.as_bytes())
            }
        }
    }
    fn method_name(&self, method_id: u32) -> Option<String> {
        AccountManagementProtocolMethod::try_from(method_id)
            .ok()
            .map(|e| format!("{:?}", e))
    }
}
#[allow(unused_variables)]
pub trait AccountManagementProtocolTrait<CI> {
    fn create_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(create_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn delete_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(delete_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn disable_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DisableAccountRequest,
    ) -> Result<DisableAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(disable_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn change_password(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ChangePasswordRequest,
    ) -> Result<ChangePasswordResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(change_password)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn test_capability(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: TestCapabilityRequest,
    ) -> Result<TestCapabilityResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(test_capability)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_name(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetNameRequest,
    ) -> Result<GetNameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_name)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_account_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetAccountDataRequest,
    ) -> Result<GetAccountDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_account_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_private_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPrivateDataRequest,
    ) -> Result<GetPrivateDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_private_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_public_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetPublicDataRequest,
    ) -> Result<GetPublicDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_public_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_multiple_public_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetMultiplePublicDataRequest,
    ) -> Result<GetMultiplePublicDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_multiple_public_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account_name(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountNameRequest,
    ) -> Result<UpdateAccountNameResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_account_name)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account_email(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountEmailRequest,
    ) -> Result<UpdateAccountEmailResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_account_email)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_custom_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateCustomDataRequest,
    ) -> Result<UpdateCustomDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_custom_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn find_by_name_regex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: FindByNameRegexRequest,
    ) -> Result<FindByNameRegexResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(find_by_name_regex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account_expiry_date(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountExpiryDateRequest,
    ) -> Result<UpdateAccountExpiryDateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_account_expiry_date)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account_effective_date(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountEffectiveDateRequest,
    ) -> Result<UpdateAccountEffectiveDateResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_account_effective_date)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_status(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateStatusRequest,
    ) -> Result<UpdateStatusResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_status)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_status(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetStatusRequest,
    ) -> Result<GetStatusResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_status)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn get_last_connection_stats(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: GetLastConnectionStatsRequest,
    ) -> Result<GetLastConnectionStatsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(get_last_connection_stats)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn reset_password(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ResetPasswordRequest,
    ) -> Result<ResetPasswordResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(reset_password)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn create_account_with_custom_data(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateAccountWithCustomDataRequest,
    ) -> Result<CreateAccountWithCustomDataResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(create_account_with_custom_data)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn retrieve_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: RetrieveAccountRequest,
    ) -> Result<RetrieveAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(retrieve_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn update_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: UpdateAccountRequest,
    ) -> Result<UpdateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(update_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn change_password_by_guest(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: ChangePasswordByGuestRequest,
    ) -> Result<ChangePasswordByGuestResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(change_password_by_guest)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn find_by_name_like(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: FindByNameLikeRequest,
    ) -> Result<FindByNameLikeResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(find_by_name_like)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn custom_create_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CustomCreateAccountRequest,
    ) -> Result<CustomCreateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(custom_create_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn lookup_or_create_account(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: LookupOrCreateAccountRequest,
    ) -> Result<LookupOrCreateAccountResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(lookup_or_create_account)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn create_account_ex(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: CreateAccountExRequest,
    ) -> Result<CreateAccountExResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(create_account_ex)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn disconnect_principal(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DisconnectPrincipalRequest,
    ) -> Result<DisconnectPrincipalResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(disconnect_principal)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
    fn disconnect_all_principals(
        &self,
        logger: &Logger,
        ctx: &Context,
        ci: &mut ClientInfo<CI>,
        request: DisconnectAllPrincipalsRequest,
    ) -> Result<DisconnectAllPrincipalsResponse, Error> {
        warn!(
            logger,
            "Method {}.{} not implemented",
            "AccountManagementProtocol",
            stringify!(disconnect_all_principals)
        );
        Err(quazal::rmc::Error::UnimplementedMethod)
    }
}
