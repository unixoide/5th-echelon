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

mod bytes {
    use serde::de::Visitor;
    use serde::Deserializer;
    use serde::Serializer;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("stirng or bytes")
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct Context {
    #[serde(with = "bytes")]
    pub access_key: Vec<u8>,
    #[serde(with = "bytes")]
    pub crypto_key: Vec<u8>,
    pub listen: SocketAddr,
    // pub server_id: u32,
    pub vport: u8,
    pub secure_server_addr: Option<SocketAddr>,
    pub settings: HashMap<String, String>,
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
    #[must_use]
    pub fn splinter_cell_blacklist() -> Context {
        Context {
            access_key: b"yl4NG7qZ".to_vec(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn key(&self, stype: StreamType) -> u32 {
        let sum = || {
            let key_sum = self
                .access_key
                .iter()
                .fold(Wrapping(0u32), |acc, x| acc + Wrapping(u32::from(*x)));
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OnlineConfigItem {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OnlineConfigContent {
    Raw(String),
    Typed(Vec<OnlineConfigItem>),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineConfig {
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
                        values: vec!["prudp:/address=127.0.0.1;port=21170".into()],
                    },
                    OnlineConfigItem {
                        name: "SandboxUrlWS".into(),
                        values: vec!["127.0.0.1:21170".into()],
                    },
                    // OnlineConfigItem { name: "punch_DetectUrls".into(), values: vec!["b-prod-mm-detect01.ubisoft.com:11020".into(),"lb-prod-mm-detect02.ubisoft.com:11020".into()]},
                    // OnlineConfigItem { name: "SandboxUrl".into(), values: vec!["prudp:/address=mdc-mm-rdv66.ubisoft.com;port=21170".into()]},
                    // OnlineConfigItem { name: "SandboxUrlWS".into(), values: vec!["mdc-mm-rdv66.ubisoft.com:21170".into()]},
                    // OnlineConfigItem { name: "uplay_DownloadServiceUrl".into(), values: vec!["https://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url=".into()]},
                    // OnlineConfigItem { name: "uplay_DynContentBaseUrl".into(), values: vec!["http://static8.cdn.ubi.com/u/Uplay/".into()]},
                    // OnlineConfigItem { name: "uplay_DynContentSecureBaseUrl".into(), values: vec!["http://static8.cdn.ubi.com/".into()]},
                ]),
            }
        }
    }
}

impl OnlineConfig {
    #[must_use]
    pub fn content(&self) -> String {
        match &self.content {
            OnlineConfigContent::Raw(s) => s.clone(),
            OnlineConfigContent::Typed(items) => serde_json::to_string(items).unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServer {
    pub listen: SocketAddr,
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

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Service {
    Authentication(Context),
    Secure(Context),
    Config(OnlineConfig),
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
        match t
            .map(|v| Tag::deserialize(v.to_owned()))
            .transpose()
            .map_err(de::Error::custom)?
        {
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

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub services: std::collections::HashSet<String>,
    pub service: std::collections::HashMap<String, Service>,
}

impl Config {
    pub fn load_from_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let w: Config = toml::from_str(&data)?;
        Ok(w)
    }

    pub fn into_services(self) -> Result<Vec<(String, Service)>, Error> {
        let Config {
            mut services,
            service,
        } = self;
        let service_contexts = service
            .into_iter()
            .filter_map(|(ref name, ctx)| {
                if services.remove(name) {
                    Some((name.to_string(), ctx))
                } else {
                    None
                }
            })
            .collect();
        if services.is_empty() {
            Ok(service_contexts)
        } else {
            Err(Error::ServiceNotFound(
                services.into_iter().collect::<Vec<_>>().join("/"),
            ))
        }
    }

    pub fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
