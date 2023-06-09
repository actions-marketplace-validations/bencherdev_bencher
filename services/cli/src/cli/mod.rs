use bencher_json::{Jwt, Url};
use clap::{ArgGroup, Args, Parser, Subcommand};

pub mod docs;
pub mod mock;
pub mod organization;
pub mod project;
pub mod system;
pub mod user;

#[cfg(feature = "docs")]
use docs::CliDocs;
use mock::CliMock;
use organization::{member::CliMember, CliOrganization};
use project::{
    alert::CliAlert, benchmark::CliBenchmark, branch::CliBranch, metric_kind::CliMetricKind,
    perf::CliPerf, report::CliReport, result::CliResult, run::CliRun, testbed::CliTestbed,
    threshold::CliThreshold, CliProject,
};
use system::{auth::CliAuth, server::CliServer};
use user::{token::CliToken, CliUser};

/// Bencher CLI
#[derive(Parser, Debug)]
#[clap(name = "bencher", author, version, about, long_about = None)]
pub struct CliBencher {
    /// Bencher subcommands
    #[clap(subcommand)]
    pub sub: CliSub,
}

#[derive(Subcommand, Debug)]
pub enum CliSub {
    /// Server authentication & authorization
    #[clap(subcommand)]
    Auth(CliAuth),

    /// Manage organization
    #[clap(subcommand, alias = "org")]
    Organization(CliOrganization),
    /// Manage organization members
    #[clap(subcommand)]
    Member(CliMember),

    /// Manage projects
    #[clap(subcommand)]
    Project(CliProject),
    /// Manage reports
    #[clap(subcommand)]
    Report(CliReport),
    /// View results
    #[clap(subcommand)]
    Result(CliResult),
    /// Manage branches
    #[clap(subcommand)]
    Branch(CliBranch),
    /// Manage testbeds
    #[clap(subcommand)]
    Testbed(CliTestbed),
    /// Manage thresholds
    #[clap(subcommand)]
    Threshold(CliThreshold),
    /// Manage metric kinds
    #[clap(subcommand)]
    MetricKind(CliMetricKind),
    /// Run benchmarks
    Run(CliRun),
    /// View benchmarks
    #[clap(subcommand)]
    Benchmark(CliBenchmark),
    /// View alerts
    #[clap(subcommand)]
    Alert(CliAlert),
    /// Query benchmark data
    Perf(CliPerf),

    /// View user
    #[clap(subcommand)]
    User(CliUser),
    /// Manage user API tokens
    #[clap(subcommand)]
    Token(CliToken),

    /// Server commands
    #[clap(subcommand)]
    Server(CliServer),

    /// Generate mock benchmark data
    Mock(CliMock),

    #[cfg(feature = "docs")]
    /// Generate documentation
    Docs(CliDocs),
}

#[derive(Args, Debug)]
pub struct CliLocality {
    /// Run local only
    #[clap(long)]
    pub local: bool,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("backend")
        .multiple(true)
        .conflicts_with("local")
        .args(&["token", "host"]),
))]
pub struct CliBackend {
    /// Backend host URL (default https://api.bencher.dev)
    #[clap(long)]
    pub host: Option<Url>,

    /// User API token
    #[clap(long)]
    pub token: Option<Jwt>,

    /// Request attempt(s) (default 10)
    #[clap(long)]
    pub attempts: Option<usize>,

    /// Retry after second(s) (default 3)
    #[clap(long)]
    pub retry_after: Option<u64>,
}
