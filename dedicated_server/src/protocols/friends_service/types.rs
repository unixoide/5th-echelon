#[derive(Debug, ToStream, FromStream)]
pub struct FriendData {
    pub pid: u32,
    pub str_name: String,
    pub by_relationship: u8,
    pub ui_details: u32,
    pub str_status: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct RelationshipData {
    pub pid: u32,
    pub str_name: String,
    pub by_relationship: u8,
    pub ui_details: u32,
    pub by_status: u8,
}
