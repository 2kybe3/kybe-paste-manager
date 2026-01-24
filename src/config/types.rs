use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "pastebin_com")]
    pub pastebin_com: PastebinComConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PastebinComConfig {
    pub enable: bool,
    pub key: Option<String>,
}

impl Default for PastebinComConfig {
    fn default() -> Self {
        Self {
            enable: false,
            key: Some("".into()),
        }
    }
}
