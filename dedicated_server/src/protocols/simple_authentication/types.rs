#[derive(Debug, ToStream, FromStream)]
pub struct RVConnectionData {
    pub url_regular_protocols: quazal::rmc::types::StationURL,
    pub lst_special_protocols: Vec<u8>,
    pub url_special_protocols: quazal::rmc::types::StationURL,
}
#[derive(Debug, ToStream, FromStream)]
pub struct LoginData {
    pub principal_type: i8,
    pub user_name: String,
    pub context: u64,
    pub similar_connection: u32,
}
