#![cfg(feature = "plus")]

use bencher_json::ResourceId;
use clap::Parser;

use crate::cli::CliBackend;

#[derive(Parser, Debug)]
pub struct CliOrganizationUsage {
    /// Organization slug or UUID
    pub organization: ResourceId,

    /// Start time
    #[clap(long)]
    pub start: i64,

    /// End time
    #[clap(long)]
    pub end: i64,

    #[clap(flatten)]
    pub backend: CliBackend,
}
