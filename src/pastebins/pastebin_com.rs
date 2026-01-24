use anyhow::{Context, anyhow};

use crate::pastebins::{PasteBin, PasteBinMeta};

const URL: &str = "https://pastebin.com/api/api_post.php";

/// https://pastebin.com/
/// Register and visit https://pastebin.com/doc_api to get your `api_key`
pub struct PastebinCom {
    api_key: String,
}

impl PastebinCom {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
}

impl PasteBinMeta for PastebinCom {
    const ID: &'static str = "pastebin";
    const DISPLAY_NAME: &'static str = "Pastebin";
    const DOMAIN: &'static str = "pastebin.com";
}

#[async_trait::async_trait]
impl PasteBin for PastebinCom {
    async fn upload(&self, content: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post(URL)
            .form(&[
                ("api_dev_key", self.api_key.as_str()),
                ("api_option", "paste"),
                ("api_paste_code", content),
            ])
            .send()
            .await
            .context("failed to send request to pastebin")?
            .text()
            .await
            .context("failed to read pastebin response")?;

        if response.starts_with("Bad API request") {
            return Err(anyhow!("pastebin error: {}", response));
        }

        Ok(response)
    }
}
