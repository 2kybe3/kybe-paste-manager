use std::{collections::HashMap, sync::Arc};

pub mod pastebin_com;

pub struct PasteBins {
    services: HashMap<String, Arc<dyn PasteBin>>,
}

impl PasteBins {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: PasteBin + PasteBinMeta + 'static>(&mut self, service: Arc<T>) {
        self.services.insert(T::ID.to_string(), service);
    }

    pub fn get(&self, id: &str) -> Option<&Arc<dyn PasteBin>> {
        self.services.get(id)
    }

    #[allow(unused)]
    pub fn all(&self) -> impl Iterator<Item = &Arc<dyn PasteBin>> {
        self.services.values()
    }
}

pub trait PasteBinMeta {
    const ID: &'static str;
    const DISPLAY_NAME: &'static str;
    const DOMAIN: &'static str;
}

#[async_trait::async_trait]
pub trait PasteBin: Send + Sync {
    async fn upload(&self, content: &str) -> anyhow::Result<String>;
}
