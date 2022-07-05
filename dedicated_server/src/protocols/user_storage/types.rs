#[derive(Debug, ToStream, FromStream)]
pub struct ContentProperty {
    pub id: u32,
    pub value: quazal::rmc::types::Variant,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserContentKey {
    pub type_id: u32,
    pub content_id: u64,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserContent {
    pub key: UserContentKey,
    pub pid: u32,
    pub properties: quazal::rmc::types::QList<ContentProperty>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserStorageQuery {
    pub type_id: u32,
    pub query_id: u32,
    pub result_range: quazal::rmc::types::ResultRange,
    pub parameters: quazal::rmc::types::QList<ContentProperty>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserSlotCount {
    pub pid: u32,
    pub type_id: u32,
    pub used_slots: u32,
    pub total_slots: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserContentURL {
    pub protocol: String,
    pub host: String,
    pub path: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct WeightedTag {
    pub id: u32,
    pub number_of_occurences: u32,
}
