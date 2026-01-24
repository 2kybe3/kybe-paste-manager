use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};

use crate::config::types::Config;

pub mod types;

const CONFIG_FILE: &str = "config.toml";

impl Config {
    pub async fn load() -> anyhow::Result<(Config, PathBuf)> {
        let mut file = directories::ProjectDirs::from("xyz", "2kybe3", "kybe-paste-manager")
            .ok_or(anyhow!(
                "no valid home directory path could be retrieved from the operating system."
            ))?
            .config_dir()
            .to_path_buf();

        file.push(CONFIG_FILE);

        Ok((Self::load_from_path(file.clone()).await?, file))
    }

    async fn load_from_path(path: PathBuf) -> anyhow::Result<Config> {
        if !path.exists() {
            Ok(Self::create_config(&path, &path).await?)
        } else {
            if !path.is_file() {
                bail!("path exists but is not a file");
            }
            let config_str = tokio::fs::read_to_string(&path)
                .await
                .context("failed to read config file")?;
            Ok(toml::from_str(&config_str)
                .context("failed to parse config (invalid config file)")?)
        }
    }

    async fn create_config(path: &Path, file: &Path) -> anyhow::Result<Config> {
        if let Some(folder_path) = path.to_path_buf().parent() {
            tokio::fs::create_dir_all(folder_path).await?;
        }
        let config = Config::default();
        tokio::fs::write(file, toml::to_string(&config)?)
            .await
            .context("failed to write default config")?;
        Ok(config)
    }
}
