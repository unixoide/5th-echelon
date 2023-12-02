use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;
use serde::Serialize;
use tracing::info;

use crate::show_msgbox;
use crate::show_msgbox_ok_cancel;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn default_password() -> String {
    String::from("password1234")
}
fn default_username() -> String {
    String::from("sam_the_fisher")
}
fn default_account_id() -> String {
    String::from("00000000-0000-4000-0000-000000000000")
}
fn default_pattern() -> String {
    String::from("Save{slot}.sav")
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Hook {
    Printer,
    LeaveState,
    NextState,
    NetResultBase,
    Goal,
    SetStep,
    Thread,
    ChangeState,
    NetCore,
    NetResultSession,
    NetResultLobby,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub user: User,
    pub save: Save,
    #[serde(default)]
    pub forward_calls: Vec<String>,
    #[serde(default)]
    pub forward_all_calls: bool,
    #[serde(default)]
    pub internal_command_line: String,
    #[serde(default)]
    pub enable_hooks: Vec<Hook>,
    #[serde(default)]
    pub enable_all_hooks: bool,
    pub config_server: Option<String>,
    pub api_server: url::Url,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    #[serde(default)]
    pub cd_keys: Vec<String>,
    #[serde(default = "default_account_id")]
    pub account_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SaveDir {
    InstallLocation,
    #[default]
    Roaming,
    Custom(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Save {
    #[serde(default)]
    pub save_dir: SaveDir,
    #[serde(default = "default_pattern")]
    pub pattern: String,
    // #[serde(default)]
    // pub saves: Vec<SaveGame>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SaveGame {
    pub slot_id: usize,
    pub name: String,
}

const DEFAULT_CONFIG: &str = r#"
# Where to find the config server
ConfigServer = "127.0.0.1"
# Where to find the api server (typically the same as the config server)
ApiServer = "http://127.0.0.1:50051"

[User]
# Username for the community server
Username = "sam_the_fisher"
# Password for the community server
Password = "password1234"

[Save]
SaveDir = "Roaming"
"#;

pub fn get() -> Option<&'static Config> {
    CONFIG.get()
}

pub fn get_or_load(path: impl AsRef<Path>) -> anyhow::Result<&'static Config> {
    if let Some(cfg) = get() {
        return Ok(cfg);
    }
    let path = path.as_ref();
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(ref err)
            if err.kind() == std::io::ErrorKind::NotFound
                && show_msgbox_ok_cancel(
                    "Configuration not found, generate and exit?",
                    "Configuration not found",
                ) =>
        {
            fs::write(path, DEFAULT_CONFIG)?;
            show_msgbox(
                &format!("Config file placed at {}", path.to_str().unwrap()),
                "Done",
            );
            std::process::exit(0);
        }
        Err(err) => return Err(err.into()),
    };
    let mut cfg: Config = toml::from_str(&content)?;
    if cfg.user.cd_keys.is_empty() {
        info!("Passing startup to original dll");
        cfg.forward_calls.push("UPLAY_Startup".into());
        cfg.forward_calls.push("UPLAY_Quit".into());
        //        cfg.forward_calls.push("UPLAY_USER_GetCdKeys".into());
        cfg.user.cd_keys.push("ABCD-EFGH-IJKL-MNOP".into());
    }
    // if let Some(ref api_server) = cfg.api_server {
    crate::api::URL
        .set(cfg.api_server.clone())
        .map_err(|cfg| anyhow::anyhow!("Couldn't store api url {:?}", cfg))?;
    // }
    CONFIG
        .set(cfg)
        .map_err(|cfg| anyhow::anyhow!("Couldn't store config {:?}", cfg))?;

    get().ok_or_else(|| anyhow::anyhow!("Config not loaded"))
}

pub fn get_config_path(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().join("uplay.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_default_config() {
        let cfg: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
        println!("{}", toml::to_string_pretty(&cfg).unwrap());
    }
}
