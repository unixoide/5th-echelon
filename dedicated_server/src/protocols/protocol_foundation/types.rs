#[derive(Debug, ToStream, FromStream)]
pub struct MessageRecipient {
    pub id_recipient: u32,
    pub ui_recipient_type: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct NotificationEvent {
    pub pid_source: u32,
    pub ui_type: u32,
    pub ui_param_1: u32,
    pub ui_param_2: u32,
    pub str_param: String,
    pub ui_param_3: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct Data;
#[derive(Debug, ToStream, FromStream)]
pub struct DynamicData {
    pub buf_tail: quazal::rmc::types::BufferTail,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UserMessage {
    pub ui_id: u32,
    pub id_recipient: u32,
    pub ui_recipient_type: u32,
    pub ui_parent_id: u32,
    pub pid_sender: u32,
    pub receptiontime: quazal::rmc::types::DateTime,
    pub ui_life_time: u32,
    pub ui_flags: u32,
    pub str_subject: String,
    pub str_sender: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct ResultRange {
    pub ui_offset: u32,
    pub ui_size: u32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct Property {
    pub id: u32,
    pub value: i32,
}
#[derive(Debug, ToStream, FromStream)]
pub struct PropertyVariant {
    pub id: u32,
    pub value: quazal::rmc::types::Variant,
}
