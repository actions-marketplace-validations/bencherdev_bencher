use bencher_json::{JsonNewToken, JsonToken, ResourceId};
use diesel::{expression_methods::BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, RequestContext, TypedBody};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    context::ApiContext,
    endpoints::{
        endpoint::{response_accepted, response_ok, ResponseAccepted, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::{
        user::QueryUser,
        user::{
            auth::AuthUser,
            token::{same_user, InsertToken, QueryToken},
        },
    },
    schema,
    util::{
        cors::{get_cors, CorsResponse},
        error::into_json,
    },
    ApiError,
};

use super::Resource;

const TOKEN_RESOURCE: Resource = Resource::Token;

#[derive(Deserialize, JsonSchema)]
pub struct DirPath {
    pub user: ResourceId,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/users/{user}/tokens",
    tags = ["users", "tokens"]
}]
pub async fn dir_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<DirPath>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/users/{user}/tokens",
    tags = ["users", "tokens"]
}]
pub async fn get_ls(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<DirPath>,
) -> Result<ResponseOk<Vec<JsonToken>>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(TOKEN_RESOURCE, Method::GetLs);

    let context = rqctx.context();
    let path_params = path_params.into_inner();
    let json = get_ls_inner(context, path_params, &auth_user, endpoint)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_ls_inner(
    context: &ApiContext,
    path_params: DirPath,
    auth_user: &AuthUser,
    endpoint: Endpoint,
) -> Result<Vec<JsonToken>, ApiError> {
    let conn = &mut *context.conn().await;

    let query_user = QueryUser::from_resource_id(conn, &path_params.user)?;
    same_user!(auth_user, context.rbac, query_user.id);

    Ok(schema::token::table
        .filter(schema::token::user_id.eq(query_user.id))
        .order((schema::token::creation, schema::token::expiration))
        .load::<QueryToken>(conn)
        .map_err(api_error!())?
        .into_iter()
        .filter_map(into_json!(endpoint, conn))
        .collect())
}

#[endpoint {
    method = POST,
    path =  "/v0/users/{user}/tokens",
    tags = ["users", "tokens"]
}]
pub async fn post(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<DirPath>,
    body: TypedBody<JsonNewToken>,
) -> Result<ResponseAccepted<JsonToken>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(TOKEN_RESOURCE, Method::Post);

    let context = rqctx.context();
    let json_token = body.into_inner();
    let json = post_inner(context, path_params.into_inner(), json_token, &auth_user)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_accepted!(endpoint, json)
}

async fn post_inner(
    context: &ApiContext,
    path_params: DirPath,
    json_token: JsonNewToken,
    auth_user: &AuthUser,
) -> Result<JsonToken, ApiError> {
    let conn = &mut *context.conn().await;

    let insert_token = InsertToken::from_json(
        conn,
        &context.rbac,
        &context.secret_key,
        &path_params.user,
        json_token,
        auth_user,
    )?;

    diesel::insert_into(schema::token::table)
        .values(&insert_token)
        .execute(conn)
        .map_err(api_error!())?;

    schema::token::table
        .filter(schema::token::uuid.eq(&insert_token.uuid))
        .first::<QueryToken>(conn)
        .map_err(api_error!())?
        .into_json(conn)
        .map_err(api_error!())
}

#[derive(Deserialize, JsonSchema)]
pub struct OnePath {
    pub user: ResourceId,
    pub token: Uuid,
}

#[allow(clippy::unused_async)]
#[endpoint {
    method = OPTIONS,
    path =  "/v0/users/{user}/tokens/{token}",
    tags = ["users", "tokens"]
}]
pub async fn one_options(
    _rqctx: RequestContext<ApiContext>,
    _path_params: Path<OnePath>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
    method = GET,
    path =  "/v0/users/{user}/tokens/{token}",
    tags = ["users", "tokens"]
}]
pub async fn get_one(
    rqctx: RequestContext<ApiContext>,
    path_params: Path<OnePath>,
) -> Result<ResponseOk<JsonToken>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(TOKEN_RESOURCE, Method::GetOne);

    let context = rqctx.context();
    let path_params = path_params.into_inner();
    let json = get_one_inner(context, path_params, &auth_user)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_one_inner(
    context: &ApiContext,
    path_params: OnePath,
    auth_user: &AuthUser,
) -> Result<JsonToken, ApiError> {
    let conn = &mut *context.conn().await;

    let query_user = QueryUser::from_resource_id(conn, &path_params.user)?;
    same_user!(auth_user, context.rbac, query_user.id);

    schema::token::table
        .filter(
            schema::token::user_id
                .eq(query_user.id)
                .and(schema::token::uuid.eq(&path_params.token.to_string())),
        )
        .first::<QueryToken>(conn)
        .map_err(api_error!())?
        .into_json(conn)
        .map_err(api_error!())
}
