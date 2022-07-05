#[derive(Debug, ToStream, FromStream)]
pub struct ClanInfo {
    pub clid: u32,
    pub tag: String,
    pub title: String,
    pub motto: String,
}
