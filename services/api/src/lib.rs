#![allow(clippy::result_large_err)]

#[macro_use]
extern crate diesel;

pub mod config;
pub mod context;
pub mod endpoints;
pub mod error;
pub mod model;
pub mod schema;
pub mod util;

pub use error::{ApiError, WordStr};
