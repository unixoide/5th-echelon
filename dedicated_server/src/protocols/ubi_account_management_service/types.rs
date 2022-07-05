#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]
#[derive(Debug, ToStream, FromStream)]
pub struct ExternalAccount {
    pub account_type: u32,
    pub id: String,
    pub username: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UbiAccountStatus {
    pub basic_status: u32,
    pub missing_required_informations: bool,
    pub recovering_password: bool,
    pub pending_deactivation: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UbiAccount {
    pub ubi_account_id: String,
    pub username: String,
    pub password: String,
    pub status: UbiAccountStatus,
    pub email: String,
    pub date_of_birth: quazal::rmc::types::DateTime,
    pub gender: u32,
    pub country_code: String,
    pub opt_in: bool,
    pub third_party_opt_in: bool,
    pub first_name: String,
    pub last_name: String,
    pub preferred_language: String,
    pub external_accounts: Vec<ExternalAccount>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct TOS {
    pub locale_code: String,
    pub content: String,
    pub storing_info_question: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct TOSEx {
    pub locale_code: String,
    pub contents: Vec<String>,
    pub storing_info_question: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ValidationFailureReason {
    pub validation_id: u32,
    pub description: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UsernameValidation {
    pub suggestions: Vec<String>,
    pub reasons: Vec<ValidationFailureReason>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct Country {
    pub code: String,
    pub name: String,
}
