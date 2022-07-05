#[derive(Debug, ToStream, FromStream)]
pub struct AccountData {
    pub pid: u32,
    pub str_name: String,
    pub ui_groups: u32,
    pub str_email: String,
    pub dt_creation_date: quazal::rmc::types::DateTime,
    pub dt_effective_date: quazal::rmc::types::DateTime,
    pub str_not_effective_msg: String,
    pub dt_expiry_date: quazal::rmc::types::DateTime,
    pub str_expired_msg: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BasicAccountInfo {
    pub pid_owner: u32,
    pub str_name: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PublicData;
#[derive(Debug, ToStream, FromStream)]
pub struct PrivateData;
