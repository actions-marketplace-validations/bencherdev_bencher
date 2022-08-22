use std::str::FromStr;

use bencher_json::threshold::{
    JsonNewThreshold,
    JsonThreshold,
};
use diesel::{
    Insertable,
    SqliteConnection,
};
use dropshot::HttpError;
use uuid::Uuid;

use super::{
    branch::QueryBranch,
    testbed::QueryTestbed,
};
use crate::{
    db::{
        schema,
        schema::threshold as threshold_table,
    },
    diesel::{
        ExpressionMethods,
        QueryDsl,
        RunQueryDsl,
    },
    util::http_error,
};

mod t_test;
mod z_score;

const THRESHOLD_ERROR: &str = "Failed to get threshold.";

#[derive(Queryable)]
pub struct QueryThreshold {
    pub id:         i32,
    pub uuid:       String,
    pub branch_id:  i32,
    pub testbed_id: i32,
    pub z_score_id: Option<i32>,
    pub t_test_id:  Option<i32>,
}

impl QueryThreshold {
    pub fn get_id(conn: &SqliteConnection, uuid: &Uuid) -> Result<i32, HttpError> {
        schema::threshold::table
            .filter(schema::threshold::uuid.eq(uuid.to_string()))
            .select(schema::threshold::id)
            .first(conn)
            .map_err(|_| http_error!(THRESHOLD_ERROR))
    }

    pub fn get_uuid(conn: &SqliteConnection, id: i32) -> Result<Uuid, HttpError> {
        let uuid: String = schema::threshold::table
            .filter(schema::threshold::id.eq(id))
            .select(schema::threshold::uuid)
            .first(conn)
            .map_err(|_| http_error!(THRESHOLD_ERROR))?;
        Uuid::from_str(&uuid).map_err(|_| http_error!(THRESHOLD_ERROR))
    }

    pub fn to_json(self, conn: &SqliteConnection) -> Result<JsonThreshold, HttpError> {
        let Self {
            id: _,
            uuid,
            branch_id,
            testbed_id,
            z_score_id,
            t_test_id,
        } = self;
        Ok(JsonThreshold {
            uuid:    Uuid::from_str(&uuid).map_err(|_| http_error!(THRESHOLD_ERROR))?,
            branch:  QueryBranch::get_uuid(conn, branch_id)?,
            testbed: QueryTestbed::get_uuid(conn, testbed_id)?,
            z_score: None,
            t_test:  None,
        })
    }
}

#[derive(Insertable)]
#[table_name = "threshold_table"]
pub struct InsertThreshold {
    pub uuid:       String,
    pub branch_id:  i32,
    pub testbed_id: i32,
}

impl InsertThreshold {
    pub fn from_json(
        conn: &SqliteConnection,
        json_threshold: JsonNewThreshold,
    ) -> Result<Self, HttpError> {
        let JsonNewThreshold { branch, testbed } = json_threshold;
        Ok(Self {
            uuid:       Uuid::new_v4().to_string(),
            branch_id:  QueryBranch::get_id(conn, &branch)?,
            testbed_id: QueryTestbed::get_id(conn, &testbed)?,
        })
    }
}