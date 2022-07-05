#[derive(Debug, ToStream, FromStream)]
pub struct UplayActionPlatform {
    pub platform_code: String,
    pub completed: bool,
    pub specific_key: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplayRewardPlatform {
    pub platform_code: String,
    pub purchased: bool,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplayAction {
    pub code: String,
    pub name: String,
    pub description: String,
    pub value: i32,
    pub game_code: String,
    pub platforms: quazal::rmc::types::QList<UplayActionPlatform>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplayReward {
    pub code: String,
    pub name: String,
    pub description: String,
    pub value: i32,
    pub reward_type_name: String,
    pub game_code: String,
    pub platforms: quazal::rmc::types::QList<UplayRewardPlatform>,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplaySectionContentLocalized {
    pub key: String,
    pub culture: String,
    pub text: String,
    pub url: String,
    pub duration: i32,
    pub size: String,
    pub width: String,
    pub height: String,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplaySectionContent {
    pub key: String,
    pub name: String,
    pub order: i16,
    pub type_name: String,
    pub localized_info: UplaySectionContentLocalized,
}
#[derive(Debug, ToStream, FromStream)]
pub struct UplaySection {
    pub key: String,
    pub name: String,
    pub type_name: String,
    pub menu_type_name: String,
    pub content_list: quazal::rmc::types::QList<UplaySectionContent>,
    pub game_code: String,
    pub platform_code: String,
}
