#[derive(Debug, ToStream, FromStream)]
pub struct AdminContent {
    pub tags: quazal::rmc::types::QList<WeightedTag>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct BannedUser {
    pub pid: u32,
    pub reason: String,
    pub date_banned: quazal::rmc::types::DateTime,
    pub expiration: quazal::rmc::types::DateTime,
}
