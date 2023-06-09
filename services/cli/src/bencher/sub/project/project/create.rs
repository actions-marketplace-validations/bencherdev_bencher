use std::convert::TryFrom;

use async_trait::async_trait;
use bencher_json::{project::JsonVisibility, JsonNewProject, NonEmpty, ResourceId, Slug, Url};

use crate::{
    bencher::{backend::Backend, sub::SubCmd},
    cli::project::{CliProjectCreate, CliProjectVisibility},
    CliError,
};

#[derive(Debug, Clone)]
pub struct Create {
    pub org: ResourceId,
    pub name: NonEmpty,
    pub slug: Option<Slug>,
    pub url: Option<Url>,
    pub visibility: Option<Visibility>,
    pub backend: Backend,
}

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    Public,
    #[cfg(feature = "plus")]
    Private,
}

impl TryFrom<CliProjectCreate> for Create {
    type Error = CliError;

    fn try_from(create: CliProjectCreate) -> Result<Self, Self::Error> {
        let CliProjectCreate {
            org,
            name,
            slug,
            url,
            visibility,
            backend,
        } = create;
        Ok(Self {
            org,
            name,
            slug,
            url,
            visibility: visibility.map(Into::into),
            backend: backend.try_into()?,
        })
    }
}

impl From<CliProjectVisibility> for Visibility {
    fn from(visibility: CliProjectVisibility) -> Self {
        match visibility {
            CliProjectVisibility::Public => Self::Public,
            #[cfg(feature = "plus")]
            CliProjectVisibility::Private => Self::Private,
        }
    }
}

impl From<Create> for JsonNewProject {
    fn from(create: Create) -> Self {
        let Create {
            name,
            slug,
            url,
            visibility,
            ..
        } = create;
        Self {
            name,
            slug,
            url,
            visibility: visibility.map(Into::into),
        }
    }
}

impl From<Visibility> for JsonVisibility {
    fn from(visibility: Visibility) -> Self {
        match visibility {
            Visibility::Public => Self::Public,
            #[cfg(feature = "plus")]
            Visibility::Private => Self::Private,
        }
    }
}

#[async_trait]
impl SubCmd for Create {
    async fn exec(&self) -> Result<(), CliError> {
        let project: JsonNewProject = self.clone().into();
        self.backend
            .post(
                &format!("/v0/organizations/{}/projects", self.org),
                &project,
            )
            .await?;
        Ok(())
    }
}
