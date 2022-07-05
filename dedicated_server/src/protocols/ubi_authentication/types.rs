#[derive(Debug, ToStream, FromStream)]
pub struct UbiAuthenticationLoginCustomData {
    pub user_name: String,
    pub online_key: String,
    pub password: String,
}
