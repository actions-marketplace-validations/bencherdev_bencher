use std::str::FromStr;

use bencher_json::report::new::JsonMetrics;
use diesel::{
    Insertable,
    Queryable,
    SqliteConnection,
};
use dropshot::HttpError;
use uuid::Uuid;

use crate::{
    db::{
        schema,
        schema::perf as perf_table,
    },
    diesel::{
        ExpressionMethods,
        QueryDsl,
        RunQueryDsl,
    },
    util::http_error,
};

pub mod latency;
pub mod min_max_avg;
pub mod throughput;

pub use latency::InsertLatency;
pub use min_max_avg::InsertMinMaxAvg;
pub use throughput::InsertThroughput;

const PERF_ERROR: &str = "Failed to get perf.";

#[derive(Queryable)]
pub struct QueryPerf {
    pub id: i32,
    pub uuid: String,
    pub report_id: i32,
    pub iteration: i32,
    pub benchmark_id: i32,
    pub latency_id: Option<i32>,
    pub throughput_id: Option<i32>,
    pub compute_id: Option<i32>,
    pub memory_id: Option<i32>,
    pub storage_id: Option<i32>,
}

impl QueryPerf {
    pub fn get_id(conn: &SqliteConnection, uuid: impl ToString) -> Result<i32, HttpError> {
        schema::perf::table
            .filter(schema::perf::uuid.eq(uuid.to_string()))
            .select(schema::perf::id)
            .first(conn)
            .map_err(|_| http_error!(PERF_ERROR))
    }

    pub fn get_uuid(conn: &SqliteConnection, id: i32) -> Result<Uuid, HttpError> {
        let uuid: String = schema::perf::table
            .filter(schema::perf::id.eq(id))
            .select(schema::perf::uuid)
            .first(conn)
            .map_err(|_| http_error!(PERF_ERROR))?;
        Uuid::from_str(&uuid).map_err(|_| http_error!(PERF_ERROR))
    }
}

#[derive(Insertable)]
#[table_name = "perf_table"]
pub struct InsertPerf {
    pub uuid:          String,
    pub report_id:     i32,
    pub iteration:     i32,
    pub benchmark_id:  i32,
    pub latency_id:    Option<i32>,
    pub throughput_id: Option<i32>,
    pub compute_id:    Option<i32>,
    pub memory_id:     Option<i32>,
    pub storage_id:    Option<i32>,
}

impl InsertPerf {
    pub fn from_json(
        conn: &SqliteConnection,
        report_id: i32,
        iteration: i32,
        benchmark_id: i32,
        metrics: JsonMetrics,
    ) -> Result<Self, HttpError> {
        Ok(InsertPerf {
            uuid: Uuid::new_v4().to_string(),
            report_id,
            iteration,
            benchmark_id,
            latency_id: InsertLatency::map_json(conn, metrics.latency)?,
            throughput_id: InsertThroughput::map_json(conn, metrics.throughput)?,
            compute_id: InsertMinMaxAvg::map_json(conn, metrics.compute)?,
            memory_id: InsertMinMaxAvg::map_json(conn, metrics.memory)?,
            storage_id: InsertMinMaxAvg::map_json(conn, metrics.storage)?,
        })
    }
}
