use std::convert::TryFrom;

use bencher_json::project::threshold::{JsonNewStatistic, JsonStatisticKind};

use crate::{
    cli::project::threshold::{CliStatisticCreate, CliStatisticKind},
    CliError,
};

#[derive(Debug, Clone, Copy)]
pub struct Statistic {
    pub test: JsonStatisticKind,
    pub min_sample_size: Option<u32>,
    pub max_sample_size: Option<u32>,
    pub window: Option<u32>,
    pub left_side: Option<f32>,
    pub right_side: Option<f32>,
}

impl TryFrom<CliStatisticCreate> for Statistic {
    type Error = CliError;

    fn try_from(create: CliStatisticCreate) -> Result<Self, Self::Error> {
        let CliStatisticCreate {
            test,
            min_sample_size,
            max_sample_size,
            window,
            left_side,
            right_side,
        } = create;
        Ok(Self {
            test: test.into(),
            min_sample_size,
            max_sample_size,
            window,
            // TODO validate these as reasonable percentages
            left_side,
            right_side,
        })
    }
}

impl From<CliStatisticKind> for JsonStatisticKind {
    fn from(kind: CliStatisticKind) -> Self {
        match kind {
            CliStatisticKind::Z => Self::Z,
            CliStatisticKind::T => Self::T,
        }
    }
}

impl From<Statistic> for JsonNewStatistic {
    fn from(statistic: Statistic) -> Self {
        let Statistic {
            test,
            min_sample_size,
            max_sample_size,
            window,
            left_side,
            right_side,
        } = statistic;
        Self {
            test,
            min_sample_size,
            max_sample_size,
            window,
            left_side: left_side.map(Into::into),
            right_side: right_side.map(Into::into),
        }
    }
}
