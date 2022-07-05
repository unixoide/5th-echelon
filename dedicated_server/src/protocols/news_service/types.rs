#[derive(Debug, ToStream, FromStream)]
pub struct NewsChannel {
    pub id: u32,
    pub owner_pid: u32,
    pub name: String,
    pub description: String,
    pub creation_time: quazal::rmc::types::DateTime,
    pub expiration_time: quazal::rmc::types::DateTime,
    pub typ: String,
    pub locale: String,
    pub subscribable: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct NewsHeader {
    pub id: u32,
    pub recipient_id: u32,
    pub recipient_type: u32,
    pub publisher_pid: u32,
    pub publisher_name: String,
    pub publication_time: quazal::rmc::types::DateTime,
    pub display_time: quazal::rmc::types::DateTime,
    pub expiration_time: quazal::rmc::types::DateTime,
    pub title: String,
    pub link: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct NewsMessage {
    pub body: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct NewsRecipient {
    pub recipient_id: u32,
    pub recipient_type: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct NewsFeedLink {
    pub id: u32,
    pub news_channel_id: u32,
    pub url: String,
    pub description: String,
    pub feed_updated_parse_time: quazal::rmc::types::DateTime,
    pub last_updated_time: quazal::rmc::types::DateTime,
    pub next_refresh_time: quazal::rmc::types::DateTime,
    pub refresh_elapsed_time_milliseconds: u32,
    pub refresh_period_seconds: u32,
    pub refresh_enabled: bool,
    pub refresh_method: u32,
    pub newest_message_time: quazal::rmc::types::DateTime,
    pub message_added: u32,
}
