#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config file not loaded")]
    ConfigNotLoaded,
}
