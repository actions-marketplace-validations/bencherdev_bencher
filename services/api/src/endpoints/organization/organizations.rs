use std::sync::Arc;

use bencher_json::{
    organization::organization::JsonOrganizationPermission, JsonAllowed, JsonNewOrganization,
    JsonOrganization, ResourceId,
};
use bencher_rbac::organization::{Permission, Role};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, RequestContext, TypedBody};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    endpoints::{
        endpoint::{response_accepted, response_ok, ResponseAccepted, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::{
        organization::{
            organization::{InsertOrganization, QueryOrganization},
            organization_role::InsertOrganizationRole,
        },
        user::auth::AuthUser,
    },
    schema,
    util::{
        cors::{get_cors, CorsResponse},
        error::into_json,
        Context,
    },
    ApiError,
};

use super::Resource;

const ORGANIZATION_RESOURCE: Resource = Resource::Organization;
const PERMISSION_RESOURCE: Resource = Resource::OrganizationPermission;

#[endpoint {
    method = OPTIONS,
    path =  "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn dir_options(_rqctx: Arc<RequestContext<Context>>) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path = "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn get_ls(
    rqctx: Arc<RequestContext<Context>>,
) -> Result<ResponseOk<Vec<JsonOrganization>>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::GetLs);

    let json = get_ls_inner(rqctx.context(), &auth_user, endpoint)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_ls_inner(
    context: &Context,
    auth_user: &AuthUser,
    endpoint: Endpoint,
) -> Result<Vec<JsonOrganization>, ApiError> {
    let api_context = &mut *context.lock().await;
    let conn = &mut api_context.database;

    let mut sql = schema::organization::table.into_boxed();

    if !auth_user.is_admin(&api_context.rbac) {
        let organizations = auth_user.organizations(&api_context.rbac, Permission::View);
        sql = sql.filter(schema::organization::id.eq_any(organizations));
    }

    Ok(sql
        .order(schema::organization::name)
        .load::<QueryOrganization>(conn)
        .map_err(api_error!())?
        .into_iter()
        .filter_map(into_json!(endpoint))
        .collect())
}

#[endpoint {
    method = POST,
    path = "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn post(
    rqctx: Arc<RequestContext<Context>>,
    body: TypedBody<JsonNewOrganization>,
) -> Result<ResponseAccepted<JsonOrganization>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::Post);

    let json = post_inner(rqctx.context(), body.into_inner(), &auth_user)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_accepted!(endpoint, json)
}

async fn post_inner(
    context: &Context,
    json_organization: JsonNewOrganization,
    auth_user: &AuthUser,
) -> Result<JsonOrganization, ApiError> {
    let api_context = &mut *context.lock().await;
    let conn = &mut api_context.database;

    if !auth_user.is_admin(&api_context.rbac) {
        return Err(ApiError::CreateOrganization(auth_user.id));
    }

    // Create the organization
    let insert_organization = InsertOrganization::from_json(conn, json_organization);
    diesel::insert_into(schema::organization::table)
        .values(&insert_organization)
        .execute(conn)
        .map_err(api_error!())?;
    let query_organization = schema::organization::table
        .filter(schema::organization::uuid.eq(&insert_organization.uuid))
        .first::<QueryOrganization>(conn)
        .map_err(api_error!())?;

    // Connect the user to the organization as a `Maintainer`
    let insert_org_role = InsertOrganizationRole {
        user_id: auth_user.id,
        organization_id: query_organization.id,
        role: Role::Leader.to_string(),
    };
    diesel::insert_into(schema::organization_role::table)
        .values(&insert_org_role)
        .execute(conn)
        .map_err(api_error!())?;

    query_organization.into_json()
}

#[derive(Deserialize, JsonSchema)]
pub struct GetOneParams {
    pub organization: ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/organizations/{organization}",
    tags = ["organizations"]
}]
pub async fn one_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetOneParams>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path = "/v0/organizations/{organization}",
    tags = ["organizations"]
}]
pub async fn get_one(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetOneParams>,
) -> Result<ResponseOk<JsonOrganization>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::GetOne);

    let json = get_one_inner(rqctx.context(), path_params.into_inner(), &auth_user)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_one_inner(
    context: &Context,
    path_params: GetOneParams,
    auth_user: &AuthUser,
) -> Result<JsonOrganization, ApiError> {
    let api_context = &mut *context.lock().await;

    QueryOrganization::is_allowed_resource_id(
        api_context,
        &path_params.organization,
        auth_user,
        Permission::View,
    )?
    .into_json()
}

#[derive(Deserialize, JsonSchema)]
pub struct GetAllowedParams {
    pub organization: ResourceId,
    pub permission: JsonOrganizationPermission,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/organizations/{organization}/allowed/{permission}",
    tags = ["organizations", "allowed"]
}]
pub async fn allowed_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetAllowedParams>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path = "/v0/organizations/{organization}/allowed/{permission}",
    tags = ["organizations", "allowed"]
}]
pub async fn get_allowed(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetAllowedParams>,
) -> Result<ResponseOk<JsonAllowed>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(PERMISSION_RESOURCE, Method::GetOne);

    let json = get_allowed_inner(rqctx.context(), path_params.into_inner(), &auth_user)
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_allowed_inner(
    context: &Context,
    path_params: GetAllowedParams,
    auth_user: &AuthUser,
) -> Result<JsonAllowed, ApiError> {
    let api_context = &mut *context.lock().await;

    Ok(JsonAllowed {
        allowed: QueryOrganization::is_allowed_resource_id(
            api_context,
            &path_params.organization,
            auth_user,
            crate::model::organization::organization_role::Permission::from(path_params.permission)
                .into(),
        )
        .is_ok(),
    })
}