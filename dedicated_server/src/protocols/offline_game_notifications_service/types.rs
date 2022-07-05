#[derive(Debug, ToStream, FromStream)]
pub struct TimedNotification {
    pub timestamp: quazal::rmc::types::DateTime,
    pub notification: NotificationEvent,
}
