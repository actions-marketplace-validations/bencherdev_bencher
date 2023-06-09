use std::{collections::HashMap, str::FromStr};

use bencher_json::{JsonMetric, JsonMetrics, JsonResult, ResourceId};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, RequestContext};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    endpoints::{
        endpoint::{pub_response_ok, response_ok, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::project::QueryProject,
    model::user::auth::AuthUser,
    schema,
    util::cors::{get_cors, CorsResponse},
    ApiError,
};

use super::Resource;

const RESULT_RESOURCE: Resource = Resource::Result;

#[derive(Deserialize, JsonSchema)]
pub struct OnePath {
    pub project: ResourceId,
    pub result: Uuid,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/results/{result}",
    tags = ["projects", "results"]
}]
pub async fn one_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<OnePath>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/results/{result}",
    tags = ["projects", "results"]
}]
pub async fn get_one(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<OnePath>,
) -> Result<ResponseOk<JsonResult>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await.ok();
    let endpoint = Endpoint::new(RESULT_RESOURCE, Method::GetOne);

    let json = get_one_inner(
        rqctx.context(),
        path_params.into_inner(),
        auth_user.as_ref(),
    )
    .await
    .map_err(|e| endpoint.err(e))?;

    if auth_user.is_some() {
        response_ok!(endpoint, json)
    } else {
        pub_response_ok!(endpoint, json)
    }
}

#[allow(clippy::cast_sign_loss)]
async fn get_one_inner(
    context: &ApiContext,
    path_params: OnePath,
    auth_user: Option<&AuthUser>,
) -> Result<JsonResult, ApiError> {
    let conn = &mut *context.conn().await;

    let project_id =
        QueryProject::is_allowed_public(conn, &context.rbac, &path_params.project, auth_user)?.id;

    let perf = schema::perf::table
        .filter(schema::perf::uuid.eq(path_params.result.to_string()))
        .inner_join(
            schema::benchmark::table.on(schema::perf::benchmark_id.eq(schema::benchmark::id)),
        )
        .filter(schema::benchmark::project_id.eq(project_id))
        .inner_join(schema::report::table.on(schema::perf::report_id.eq(schema::report::id)))
        .select((
            schema::perf::uuid,
            schema::report::uuid,
            schema::perf::iteration,
            schema::benchmark::uuid,
        ))
        .first::<(String, String, i32, String)>(conn)
        .map(
            |(perf_uuid, report_uuid, iteration, benchmark_uuid)| -> Result<_, ApiError> {
                Ok(Perf {
                    uuid: Uuid::from_str(&perf_uuid)?,
                    report: Uuid::from_str(&report_uuid)?,
                    iteration: iteration as u32,
                    benchmark: Uuid::from_str(&benchmark_uuid)?,
                })
            },
        )
        .map_err(api_error!())??;

    let metrics = schema::perf::table
        .inner_join(schema::metric::table.on(schema::perf::id.eq(schema::metric::perf_id)))
        .inner_join(
            schema::metric_kind::table
                .on(schema::metric::metric_kind_id.eq(schema::metric_kind::id)),
        )
        .select((
            schema::metric_kind::uuid,
            schema::metric::value,
            schema::metric::lower_bound,
            schema::metric::upper_bound,
        ))
        .load::<(String, f64, Option<f64>, Option<f64>)>(conn)
        .map_err(api_error!())?;

    let mut metrics_map = HashMap::new();
    for (metric_kind_uuid, value, lower_bound, upper_bound) in metrics {
        metrics_map.insert(
            Uuid::from_str(&metric_kind_uuid)?,
            JsonMetric {
                value: value.into(),
                lower_bound: lower_bound.map(Into::into),
                upper_bound: upper_bound.map(Into::into),
            },
        );
    }

    Ok(perf.into_json(metrics_map))
}

struct Perf {
    pub uuid: Uuid,
    pub report: Uuid,
    pub iteration: u32,
    pub benchmark: Uuid,
}

impl Perf {
    fn into_json(self, metrics: JsonMetrics) -> JsonResult {
        let Self {
            uuid,
            report,
            iteration,
            benchmark,
        } = self;
        JsonResult {
            uuid,
            report,
            iteration,
            benchmark,
            metrics,
        }
    }
}
