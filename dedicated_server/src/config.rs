use std::collections::HashMap;
use std::net::SocketAddr;

use quazal::ContentServer;
use quazal::Context;
use quazal::OnlineConfig;
use quazal::Service;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(flatten)]
    pub quazal_config: quazal::Config,
    pub api_server: SocketAddr,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parsing error: {0}")]
    DeserializeError(#[from] toml::de::Error),
    #[error("Serializing error: {0}")]
    SerializeError(#[from] toml::ser::Error),
}

impl Config {
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let data = std::fs::read_to_string(path)?;
        let w: Config = toml::from_str(&data)?;
        Ok(w)
    }

    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Error> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn load_from_file_or_default<P: AsRef<std::path::Path>>(
        logger: &slog::Logger,
        path: P,
    ) -> eyre::Result<Self> {
        let e = match Self::load_from_file(path.as_ref()) {
            Ok(cfg) => return Ok(cfg),
            Err(e) => e,
        };

        if !matches!(e, Error::IO(_)) {
            return Err(e.into());
        }

        error!(logger, "Couldn't load service file, generating default"; "error" => %e);
        let mut online_cfg = OnlineConfig::default();
        online_cfg.listen.set_ip("127.0.0.1".parse().unwrap());
        online_cfg.content = online_cfg
            .content
            .replace("mdc-mm-rdv66.ubisoft.com", "127.0.0.1");

        let mut content_srv = ContentServer::default();
        content_srv.listen.set_ip(online_cfg.listen.ip());

        let mut ctx = Context::splinter_cell_blacklist();
        ctx.listen.set_ip(online_cfg.listen.ip());
        ctx.listen.set_port(21170);
        let mut secure_ctx = ctx.clone();
        secure_ctx
            .settings
            .insert(String::from("storage_host"), content_srv.listen.to_string());
        if let Some(path) = content_srv.files.keys().next() {
            secure_ctx
                .settings
                .insert(String::from("storage_path"), path.clone());
        }
        secure_ctx.listen.set_port(ctx.listen.port() + 1);
        ctx.secure_server_addr = Some(secure_ctx.listen);

        let quazal_config = quazal::Config {
            services: ["onlineconfig", "content", "sc_bl_secure", "sc_bl_auth"]
                .into_iter()
                .map(String::from)
                .collect(),
            service: HashMap::from([
                ("sc_bl_auth".to_string(), Service::Authentication(ctx)),
                ("sc_bl_secure".to_string(), Service::Secure(secure_ctx)),
                ("onlineconfig".to_string(), Service::Config(online_cfg)),
                ("content".to_string(), Service::Content(content_srv)),
            ]),
        };

        let cfg = Config {
            api_server: "127.0.0.1:50051".parse()?,
            quazal_config,
        };
        if let Err(e) = cfg.save_to_file(path) {
            error!(logger, "Couldn't save service file"; "error" => %e);
        }
        Ok(cfg)
    }
}
