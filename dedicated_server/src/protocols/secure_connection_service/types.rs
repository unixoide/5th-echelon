#[derive(Debug, ToStream, FromStream)]
pub struct ConnectionData {
    pub station_url: quazal::rmc::types::StationURL,
    pub connection_id: u32,
}
