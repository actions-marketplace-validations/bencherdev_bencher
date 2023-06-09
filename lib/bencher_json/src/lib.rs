pub use bencher_valid::{
    BenchmarkName, BranchName, Email, GitHash, Jwt, NonEmpty, ResourceId, Sanitize, Secret, Slug,
    Url, UserName, ValidError, MAX_LEN,
};
#[cfg(feature = "plus")]
pub use bencher_valid::{
    CardCvc, CardNumber, ExpirationMonth, ExpirationYear, PlanLevel, PlanStatus,
};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod organization;
pub mod project;
pub mod system;
pub mod urlencoded;
pub mod user;

pub use organization::{member::JsonMember, JsonNewOrganization, JsonOrganization};
pub use project::{
    alert::JsonAlert,
    benchmark::JsonBenchmark,
    branch::{JsonBranch, JsonNewBranch},
    metric::JsonMetric,
    metric_kind::{JsonMetricKind, JsonNewMetricKind},
    perf::{JsonPerf, JsonPerfQuery},
    report::{JsonNewReport, JsonReport},
    result::{JsonMetrics, JsonResult},
    testbed::{JsonNewTestbed, JsonTestbed},
    threshold::{JsonNewThreshold, JsonThreshold},
    JsonNewProject, JsonProject,
};
pub use system::{
    auth::{JsonAuthToken, JsonLogin, JsonSignup},
    backup::JsonBackup,
    config::JsonConfig,
    restart::JsonRestart,
    version::JsonVersion,
};
pub use user::{
    token::{JsonNewToken, JsonToken},
    JsonUser,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonEmpty {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonAllowed {
    pub allowed: bool,
}

pub fn sanitize_json<T>(json: &T) -> serde_json::Value
where
    T: Clone + Serialize + Sanitize,
{
    if cfg!(debug_assertions) {
        serde_json::json!(json)
    } else {
        let mut sanitized = json.clone();
        sanitized.sanitize();
        serde_json::json!(sanitized)
    }
}
