#[derive(Debug, ToStream, FromStream)]
pub struct Privilege {
    pub id: u32,
    pub description: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PrivilegeEx {
    pub id: u32,
    pub description: String,
    pub duration: i32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PrivilegeGroup {
    pub description: String,
    pub privileges: quazal::rmc::types::QList<Privilege>,
}
