use bencher_json::{NonEmpty, ResourceId, Slug};
use clap::{Parser, Subcommand};

use crate::cli::CliBackend;

#[derive(Subcommand, Debug)]
pub enum CliTestbed {
    /// List testbeds
    #[clap(alias = "ls")]
    List(CliTestbedList),
    /// Create a testbed
    #[clap(alias = "add")]
    Create(CliTestbedCreate),
    /// View a testbed
    View(CliTestbedView),
}

#[derive(Parser, Debug)]
pub struct CliTestbedList {
    /// Project slug or UUID
    #[clap(long)]
    pub project: ResourceId,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliTestbedCreate {
    /// Project slug or UUID
    #[clap(long)]
    pub project: ResourceId,

    /// Testbed name
    pub name: NonEmpty,

    /// Testbed slug
    #[clap(long)]
    pub slug: Option<Slug>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Parser, Debug)]
pub struct CliTestbedView {
    /// Project slug or UUID
    #[clap(long)]
    pub project: ResourceId,

    /// Testbed slug or UUID
    pub testbed: ResourceId,

    #[clap(flatten)]
    pub backend: CliBackend,
}
