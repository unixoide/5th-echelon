// Provides functionality for handling byte serialization and deserialization.
use std::collections::HashMap;
use std::net::SocketAddr;
use std::num::Wrapping;
use std::path::PathBuf;

use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use sodiumoxide::crypto::secretbox;
use toml::Value;

use crate::prudp::packet::StreamType;
use crate::Error;

// Custom serialization and deserialization for byte arrays.
mod bytes {
    use serde::de::Visitor;
    use serde::Deserializer;
    use serde::Serializer;

    // Deserializes a value into a byte vector.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl Visitor<'_> for BytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or bytes")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.as_bytes().to_vec())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into_bytes())
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.to_vec())
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }
        deserializer.deserialize_any(BytesVisitor)
    }

    // Serializes a byte slice.
    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(s) = String::from_utf8(value.to_vec()) {
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(value)
        }
    }
}

/// Represents the context for a service, including keys, addresses, and settings.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Context {
    /// Access key for the service.
    #[serde(with = "bytes")]
    pub access_key: Vec<u8>,
    /// Crypto key for the service.
    #[serde(with = "bytes")]
    pub crypto_key: Vec<u8>,
    /// Listening address for the service.
    pub listen: SocketAddr,
    // pub server_id: u32,
    /// Virtual port for the service.
    pub vport: u8,
    /// Address of the secure server.
    pub secure_server_addr: Option<SocketAddr>,
    /// Additional settings for the service.
    pub settings: HashMap<String, String>,
    /// Ticket key for the service.
    pub ticket_key: secretbox::Key,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            access_key: Vec::new(),
            crypto_key: b"CD&ML".to_vec(),
            listen: "0.0.0.0:9999".parse().unwrap(),
            // server_id: 1000,
            vport: 1,
            secure_server_addr: None,
            settings: HashMap::new(),
            ticket_key: secretbox::gen_key(),
        }
    }
}

impl Context {
    /// Creates a new context for Splinter Cell Blacklist.
    #[must_use]
    pub fn splinter_cell_blacklist() -> Context {
        Context {
            access_key: b"yl4NG7qZ".to_vec(),
            ..Default::default()
        }
    }

    /// Returns the key for a given stream type.
    #[must_use]
    pub fn key(&self, stype: StreamType) -> u32 {
        let sum = || {
            let key_sum = self.access_key.iter().fold(Wrapping(0u32), |acc, x| acc + Wrapping(u32::from(*x)));
            key_sum.0
        };

        #[allow(clippy::match_same_arms)]
        match stype {
            StreamType::DO => 0,
            StreamType::RV => 0,
            StreamType::RVSec => sum(),
            StreamType::SBMGMT => 0,
            StreamType::NAT => 0,
            StreamType::SessionDiscovery => 0,
            StreamType::NATEcho => 0,
            StreamType::Routing => 0,
        }
    }
}

/// Represents an item in the online configuration.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OnlineConfigItem {
    name: String,
    values: Vec<String>,
}

/// Represents the content of the online configuration.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OnlineConfigContent {
    /// Raw string content.
    Raw(String),
    /// Typed content as a vector of items.
    Typed(Vec<OnlineConfigItem>),
}

/// Represents the online configuration.
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineConfig {
    /// Listening address for the configuration service.
    pub listen: SocketAddr,
    content: OnlineConfigContent,
}

impl Default for OnlineConfig {
    fn default() -> Self {
        {
            Self {
                listen: "0.0.0.0:80".parse().unwrap(),
                content: OnlineConfigContent::Typed(vec![
                    OnlineConfigItem {
                        name: "SandboxUrl".into(),
                        values: vec!["prudp:/address=127.0.0.1;port=21126".into()],
                    },
                    OnlineConfigItem {
                        name: "SandboxUrlWS".into(),
                        values: vec!["127.0.0.1:21126".into()],
                    },
                    // OnlineConfigItem {
                    //     name: "punch_DetectUrls".into(),
                    //     values: vec!["b-prod-mm-detect01.ubisoft.com:11020".into(), "lb-prod-mm-detect02.ubisoft.com:11020".into()],
                    // },
                    // OnlineConfigItem {
                    //     name: "SandboxUrl".into(),
                    //     values: vec!["prudp:/address=mdc-mm-rdv66.ubisoft.com;port=21170".into()],
                    // },
                    // OnlineConfigItem {
                    //     name: "SandboxUrlWS".into(),
                    //     values: vec!["mdc-mm-rdv66.ubisoft.com:21170".into()],
                    // },
                    // OnlineConfigItem {
                    //     name: "uplay_DownloadServiceUrl".into(),
                    //     values: vec!["https://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url=".into()],
                    // },
                    // OnlineConfigItem {
                    //     name: "uplay_DynContentBaseUrl".into(),
                    //     values: vec!["http://static8.cdn.ubi.com/u/Uplay/".into()],
                    // },
                    // OnlineConfigItem {
                    //     name: "uplay_DynContentSecureBaseUrl".into(),
                    //     values: vec!["http://static8.cdn.ubi.com/".into()],
                    // },
                ]),
            }
        }
    }
}

impl OnlineConfig {
    /// Returns the content of the online configuration as a string.
    #[must_use]
    pub fn content(&self) -> String {
        match &self.content {
            OnlineConfigContent::Raw(s) => s.clone(),
            OnlineConfigContent::Typed(items) => serde_json::to_string(items).unwrap_or_default(),
        }
    }

    /// Sets the IP addresses in the online configuration.
    pub fn set_ips(&mut self, ip: std::net::IpAddr, public_ip: std::net::IpAddr) {
        self.listen.set_ip(ip);

        match &mut self.content {
            OnlineConfigContent::Raw(s) => {
                let Some(idx) = s.find(r#""Name":"SandboxUrl","#) else {
                    return;
                };
                let value = &s[idx..];
                let Some(start_idx) = value.find("address=").map(|i| i + "address=".len() + idx) else {
                    return;
                };
                let Some(end_idx) = value.find(";port").map(|i| i + idx) else {
                    return;
                };
                let rest = s.split_off(start_idx);
                s.push_str(&public_ip.to_string());
                s.push_str(&rest[end_idx - start_idx..]);

                // repeat the same for SandboxUrlWS
                let Some(idx) = s.find(r#""Name":"SandboxUrlWS","#) else {
                    return;
                };
                let value = &s[idx..];
                let Some(start_idx) = value.find("\"Values\":[\"").map(|i| i + "\"Values\":[\"".len() + idx) else {
                    return;
                };
                let Some(end_idx) = value.find("\"]").map(|i| i + idx) else {
                    return;
                };
                let rest = s.split_off(start_idx);
                s.push_str(&public_ip.to_string());
                s.push_str(&rest[end_idx - start_idx..]);
            }
            OnlineConfigContent::Typed(online_config_items) => {
                for item in online_config_items.iter_mut() {
                    if item.name == "SandboxUrl" {
                        for value in &mut item.values {
                            let Some(start_idx) = value.find("address=").map(|i| i + "address=".len()) else {
                                continue;
                            };
                            let Some(end_idx) = value.find(";port") else {
                                continue;
                            };
                            let rest = value.split_off(start_idx);
                            value.push_str(&public_ip.to_string());
                            value.push_str(&rest[end_idx - start_idx..]);
                        }
                    }
                    if item.name == "SandboxUrlWS" {
                        for value in &mut item.values {
                            if let Ok(mut addr) = value.parse::<SocketAddr>() {
                                addr.set_ip(public_ip);
                                *value = addr.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Represents a content server.
#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServer {
    /// Listening address for the content server.
    pub listen: SocketAddr,
    /// A map of file names to their paths.
    pub files: HashMap<String, PathBuf>,
}

impl Default for ContentServer {
    fn default() -> Self {
        Self {
            listen: "127.0.0.1:8000".parse().unwrap(),
            files: HashMap::from([("/mp_balancing.ini".into(), "./data/mp_balancing.ini".into())]),
        }
    }
}

/// Represents the different types of services.
#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Service {
    /// Authentication service.
    Authentication(Context),
    /// Secure service.
    Secure(Context),
    /// Configuration service.
    Config(OnlineConfig),
    /// Content service.
    Content(ContentServer),
}

impl Default for Service {
    fn default() -> Self {
        Self::Authentication(Context::default())
    }
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Tag {
            Authentication,
            Secure,
            Config,
            Content,
        }

        let v = Value::deserialize(deserializer)?;
        let t = v.get("type");
        match t.map(|v| Tag::deserialize(v.to_owned())).transpose().map_err(de::Error::custom)? {
            Some(Tag::Authentication) => {
                let inner = Context::deserialize(v).map_err(de::Error::custom)?;
                Ok(Service::Authentication(inner))
            }
            Some(Tag::Secure) => {
                let inner = Context::deserialize(v).map_err(de::Error::custom)?;
                Ok(Service::Secure(inner))
            }
            Some(Tag::Config) => {
                let inner = OnlineConfig::deserialize(v).map_err(de::Error::custom)?;
                Ok(Service::Config(inner))
            }
            Some(Tag::Content) => {
                let inner = ContentServer::deserialize(v).map_err(de::Error::custom)?;
                Ok(Service::Content(inner))
            }
            None => {
                let inner = Context::deserialize(v).map_err(de::Error::custom)?;
                if inner.secure_server_addr.is_some() {
                    Ok(Service::Authentication(inner))
                } else {
                    Ok(Service::Secure(inner))
                }
            }
        }
    }
}

/// Represents the overall configuration.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    /// A set of enabled services.
    pub services: std::collections::HashSet<String>,
    /// A map of service names to their configurations.
    pub service: std::collections::HashMap<String, Service>,
}

impl Config {
    /// Loads the configuration from a file.
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let w: Config = toml::from_str(&data)?;
        Ok(w)
    }

    /// Converts the configuration into a vector of services.
    pub fn into_services(self) -> Result<Vec<(String, Service)>, Error> {
        let Config { mut services, service } = self;
        let service_contexts = service
            .into_iter()
            .filter_map(|(ref name, ctx)| if services.remove(name) { Some((name.to_string(), ctx)) } else { None })
            .collect();
        if services.is_empty() {
            Ok(service_contexts)
        } else {
            Err(Error::ServiceNotFound(services.into_iter().collect::<Vec<_>>().join("/")))
        }
    }

    /// Saves the configuration to a file.
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
