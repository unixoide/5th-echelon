use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use quazal::ContentServer;
use quazal::Context;
use quazal::OnlineConfig;
use quazal::Service;
use serde::Deserialize;
use serde::Serialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, Serialize, Default, Clone, Copy)]
pub struct DebugConfig {
    pub mark_all_as_online: bool,
    pub force_joins: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[allow(clippy::struct_field_repetitions)]
    #[serde(flatten)]
    pub quazal_config: quazal::Config,
    pub api_server: SocketAddr,
    pub debug: DebugConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parsing error: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Serializing error: {0}")]
    Serialize(#[from] toml::ser::Error),
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
        online_cfg
            .listen
            .set_ip(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

        let server_ip = std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let mut content_srv = ContentServer::default();
        content_srv.listen.set_ip(online_cfg.listen.ip());

        let mut ctx = Context::splinter_cell_blacklist();
        ctx.listen.set_ip(online_cfg.listen.ip());
        ctx.listen.set_port(21170);
        let mut secure_ctx = ctx.clone();

        let mut content_server_addr = content_srv.listen;
        content_server_addr.set_ip(server_ip);
        secure_ctx.settings.insert(
            String::from("storage_host"),
            content_server_addr.to_string(),
        );
        if let Some(path) = content_srv.files.keys().next() {
            secure_ctx
                .settings
                .insert(String::from("storage_path"), path.clone());
        }
        secure_ctx.listen.set_port(ctx.listen.port() + 1);

        let mut secure_server_addr = secure_ctx.listen;
        secure_server_addr.set_ip(server_ip);
        ctx.secure_server_addr = Some(secure_server_addr);

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
            api_server: "0.0.0.0:50051".parse()?,
            quazal_config,
            debug: DebugConfig::default(),
        };
        if let Err(e) = cfg.save_to_file(path) {
            error!(logger, "Couldn't save service file"; "error" => %e);
        }
        Ok(cfg)
    }
}
