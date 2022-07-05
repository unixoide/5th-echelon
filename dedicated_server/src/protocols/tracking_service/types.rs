#[derive(Debug, ToStream, FromStream)]
pub struct TrackingInformation {
    pub ipn: u32,
    pub user_id: String,
    pub machine_id: String,
    pub visitor_id: String,
    pub uts_version: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct TrackingTag {
    pub tracking_id: u32,
    pub tag: String,
    pub attributes: String,
    pub delta_time: u32,
    pub new_user_id: String,
}
