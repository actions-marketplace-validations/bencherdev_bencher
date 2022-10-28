use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::{system::config::JsonUpdateConfig, JsonConfig};

use crate::{
    bencher::{backend::Backend, sub::SubCmd, wide::Wide},
    cli::system::server::CliConfigUpdate,
    CliError,
};

use super::CONFIG_PATH;

#[derive(Debug, Clone)]
pub struct Update {
    pub config: JsonConfig,
    pub delay: Option<u64>,
    pub backend: Backend,
}

impl TryFrom<CliConfigUpdate> for Update {
    type Error = CliError;

    fn try_from(update: CliConfigUpdate) -> Result<Self, Self::Error> {
        let CliConfigUpdate {
            config,
            delay,
            backend,
        } = update;
        Ok(Self {
            config: serde_json::from_str(&config)?,
            delay,
            backend: backend.try_into()?,
        })
    }
}

impl From<Update> for JsonUpdateConfig {
    fn from(update: Update) -> Self {
        let Update {
            config,
            delay,
            backend: _,
        } = update;
        Self { config, delay }
    }
}

#[async_trait]
impl SubCmd for Update {
    async fn exec(&self, _wide: &Wide) -> Result<(), CliError> {
        let update_config: JsonUpdateConfig = self.clone().into();
        self.backend.put(CONFIG_PATH, &update_config).await?;
        Ok(())
    }
}