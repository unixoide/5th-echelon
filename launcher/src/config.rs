use std::borrow::Cow;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use tracing::error;

use crate::games::GameVersion;

pub const RPC_DEFAULT_PORT: u16 = 50051;
pub const QUAZAL_DEFAULT_PORT: u16 = 21126;
pub const QUAZAL_DEFAULT_LOCAL_PORT: u16 = 3128;
pub const P2P_DEFAULT_PORT: u16 = 13000;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum UIVersion {
    Old,
    New,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    pub name: String,
    pub server: String,
    pub api_server_url: Option<url::Url>,
    #[serde(flatten)]
    pub user: hooks_config::User,
    pub adapter: Option<String>,
}

impl Profile {
    pub fn api_server_url(&self) -> Cow<url::Url> {
        self.api_server_url.as_ref().map(Cow::Borrowed).unwrap_or_else(|| {
            Cow::Owned(
                format!(
                    "http://{}:{}",
                    if self.server.is_empty() {
                        "localhost"
                    } else {
                        &self.server
                    },
                    RPC_DEFAULT_PORT
                )
                .parse()
                .unwrap(),
            )
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub profiles: Vec<Profile>,
    pub default_profile: String,
    #[serde(flatten)]
    pub hook_config: hooks_config::Config,
    pub default_game: GameVersion,
    pub ui_version: Option<UIVersion>,
}

impl Config {
    pub fn load(target_dir: &Path) -> ConfigMut {
        let config_path = hooks_config::get_config_path(target_dir);
        fs::read_to_string(&config_path)
            .ok()
            .and_then(|s| {
                toml::from_str(&s)
                    .inspect_err(|e| error!("Failed to parse config: {e}"))
                    .map(|cfg: Config| ConfigMut {
                        inner: cfg,
                        filepath: config_path.clone(),
                    })
                    .ok()
            })
            .unwrap_or_else(|| {
                let cfg = hooks_config::default();
                let cfg = ConfigMut {
                    inner: Config {
                        hook_config: cfg,
                        profiles: vec![
                            Profile {
                                name: "Test Account 1".into(),
                                server: "127.0.0.1".into(),
                                user: hooks_config::User {
                                    username: "sam_the_fisher".into(),
                                    password: "password1234".into(),
                                    cd_keys: vec![],
                                    account_id: "sam_the_fisher".into(),
                                },
                                api_server_url: None,
                                adapter: None,
                            },
                            Profile {
                                name: "Test Account 2".into(),
                                server: "127.0.0.1".into(),
                                user: hooks_config::User {
                                    username: "AAAABBBB".into(),
                                    password: "CCCCDDDD".into(),
                                    cd_keys: vec![],
                                    account_id: "AAAABBBB".into(),
                                },
                                api_server_url: None,
                                adapter: None,
                            },
                        ],
                        default_profile: String::from("Test Account 1"),
                        default_game: GameVersion::SplinterCellBlacklistDx9,
                        ui_version: None,
                    },
                    filepath: config_path.clone(),
                };
                if let Err(e) = cfg.save() {
                    error!("Failed to save config: {e}");
                }
                cfg
            })
    }
}

#[derive(Debug)]
pub struct ConfigMut {
    inner: Config,
    filepath: PathBuf,
}

pub enum UpdateResult<T> {
    Saved(T),
    Unchanged(T),
    Error(T),
}

impl<T> UpdateResult<T> {
    pub fn is_err(&self) -> bool {
        matches!(self, UpdateResult::Error(_))
    }
}

impl ConfigMut {
    fn save(&self) -> anyhow::Result<()> {
        let s = toml::to_string_pretty(&self.inner)?;
        fs::write(&self.filepath, s)?;
        Ok(())
    }

    pub fn update<T>(&mut self, f: impl FnOnce(&mut Config) -> T) -> UpdateResult<T> {
        let backup = self.inner.clone();
        let res = f(&mut self.inner);
        if backup != self.inner {
            if let Err(e) = self.save() {
                error!("Failed to save config: {e}");
                return UpdateResult::Error(res);
            }
            return UpdateResult::Unchanged(res);
        }
        UpdateResult::Saved(res)
    }

    pub fn into_inner(self) -> Config {
        self.inner
    }
}

impl Deref for ConfigMut {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
