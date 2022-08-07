use std::sync::Arc;

use bencher_json::{
    JsonBranch,
    JsonNewBranch,
    ResourceId,
};
use diesel::{
    expression_methods::BoolExpressionMethods,
    QueryDsl,
    RunQueryDsl,
};
use dropshot::{
    endpoint,
    HttpError,
    HttpResponseAccepted,
    HttpResponseHeaders,
    HttpResponseOk,
    Path,
    RequestContext,
    TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::{
        model::{
            branch::{
                InsertBranch,
                QueryBranch,
            },
            project::QueryProject,
        },
        schema,
    },
    diesel::ExpressionMethods,
    util::{
        cors::get_cors,
        headers::CorsHeaders,
        http_error,
        Context,
    },
};

#[derive(Deserialize, JsonSchema)]
pub struct GetLsParams {
    pub project: ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/branches",
    tags = ["projects", "branches"]
}]
pub async fn get_ls_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/branches",
    tags = ["projects", "branches"]
}]
pub async fn get_ls(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<Vec<JsonBranch>>, CorsHeaders>, HttpError> {
    let db_connection = rqctx.context();
    let path_params = path_params.into_inner();

    let conn = db_connection.lock().await;
    let query_project = QueryProject::from_resource_id(&*conn, &path_params.project)?;
    let json: Vec<JsonBranch> = schema::branch::table
        .filter(schema::branch::project_id.eq(&query_project.id))
        .order(schema::branch::name)
        .load::<QueryBranch>(&*conn)
        .map_err(|_| http_error!("Failed to get branches."))?
        .into_iter()
        .filter_map(|query| query.to_json(&*conn).ok())
        .collect();

    Ok(HttpResponseHeaders::new(
        HttpResponseOk(json),
        CorsHeaders::new_pub("GET".into()),
    ))
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/branches",
    tags = ["branches"]
}]
pub async fn post_options(
    _rqctx: Arc<RequestContext<Context>>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = POST,
    path = "/v0/branches",
    tags = ["branches"]
}]
pub async fn post(
    rqctx: Arc<RequestContext<Context>>,
    body: TypedBody<JsonNewBranch>,
) -> Result<HttpResponseHeaders<HttpResponseAccepted<JsonBranch>, CorsHeaders>, HttpError> {
    let db_connection = rqctx.context();
    let json_branch = body.into_inner();

    let conn = db_connection.lock().await;
    let insert_branch = InsertBranch::from_json(&*conn, json_branch)?;
    diesel::insert_into(schema::branch::table)
        .values(&insert_branch)
        .execute(&*conn)
        .map_err(|_| http_error!("Failed to create branch."))?;

    let query_branch = schema::branch::table
        .filter(schema::branch::uuid.eq(&insert_branch.uuid))
        .first::<QueryBranch>(&*conn)
        .map_err(|_| http_error!("Failed to create branch."))?;
    let json = query_branch.to_json(&*conn)?;

    Ok(HttpResponseHeaders::new(
        HttpResponseAccepted(json),
        CorsHeaders::new_auth("POST".into()),
    ))
}

#[derive(Deserialize, JsonSchema)]
pub struct GetOneParams {
    pub project: ResourceId,
    pub branch:  ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/branches/{branch}",
    tags = ["projects", "branches"]
}]
pub async fn get_one_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetOneParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/branches/{branch}",
    tags = ["projects", "branches"]
}]
pub async fn get_one(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetOneParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<JsonBranch>, CorsHeaders>, HttpError> {
    let db_connection = rqctx.context();
    let path_params = path_params.into_inner();
    let resource_id = path_params.branch.as_str();

    let conn = db_connection.lock().await;
    let project = QueryProject::from_resource_id(&*conn, &path_params.project)?;
    let query = if let Ok(query) = schema::branch::table
        .filter(
            schema::branch::project_id.eq(project.id).and(
                schema::branch::slug
                    .eq(resource_id)
                    .or(schema::branch::uuid.eq(resource_id)),
            ),
        )
        .first::<QueryBranch>(&*conn)
    {
        Ok(query)
    } else {
        Err(http_error!("Failed to get branch."))
    }?;
    let json = query.to_json(&*conn)?;

    Ok(HttpResponseHeaders::new(
        HttpResponseOk(json),
        CorsHeaders::new_pub("GET".into()),
    ))
}