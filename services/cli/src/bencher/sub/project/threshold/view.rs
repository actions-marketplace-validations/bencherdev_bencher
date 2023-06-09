use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::ResourceId;
use uuid::Uuid;

use crate::{
    bencher::{backend::Backend, sub::SubCmd},
    cli::project::threshold::CliThresholdView,
    CliError,
};

#[derive(Debug)]
pub struct View {
    pub project: ResourceId,
    pub threshold: Uuid,
    pub backend: Backend,
}

impl TryFrom<CliThresholdView> for View {
    type Error = CliError;

    fn try_from(view: CliThresholdView) -> Result<Self, Self::Error> {
        let CliThresholdView {
            project,
            threshold,
            backend,
        } = view;
        Ok(Self {
            project,
            threshold,
            backend: backend.try_into()?,
        })
    }
}

#[async_trait]
impl SubCmd for View {
    async fn exec(&self) -> Result<(), CliError> {
        self.backend
            .get(&format!(
                "/v0/projects/{}/thresholds/{}",
                self.project, self.threshold
            ))
            .await?;
        Ok(())
    }
}
