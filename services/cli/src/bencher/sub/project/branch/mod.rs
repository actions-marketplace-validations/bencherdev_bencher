use async_trait::async_trait;

use crate::{
    bencher::{sub::SubCmd, wide::Wide},
    cli::project::branch::CliBranch,
    CliError,
};

mod create;
mod list;
mod view;

#[derive(Debug)]
pub enum Branch {
    List(list::List),
    Create(create::Create),
    View(view::View),
}

impl TryFrom<CliBranch> for Branch {
    type Error = CliError;

    fn try_from(branch: CliBranch) -> Result<Self, Self::Error> {
        Ok(match branch {
            CliBranch::List(list) => Self::List(list.try_into()?),
            CliBranch::Create(create) => Self::Create(create.try_into()?),
            CliBranch::View(view) => Self::View(view.try_into()?),
        })
    }
}

#[async_trait]
impl SubCmd for Branch {
    async fn exec(&self, wide: &Wide) -> Result<(), CliError> {
        match self {
            Self::List(list) => list.exec(wide).await,
            Self::Create(create) => create.exec(wide).await,
            Self::View(view) => view.exec(wide).await,
        }
    }
}