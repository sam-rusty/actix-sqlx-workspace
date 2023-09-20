use std::env::var;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

pub mod crud;
pub mod db;
pub mod encryption;
pub mod error;
pub mod guard;
pub mod middleware;
pub mod query_param;
pub mod queue;
pub mod response;
pub mod users;

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Country {
    CA,
    US,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone, Copy, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

/// Load `.env` file to use via `std::env`
pub fn load_env(required_vars: Option<Vec<&str>>) {
    dotenv().ok();
    if let Some(required_vars) = required_vars {
        // make sure all vars are set
        required_vars.iter().for_each(|n| {
            var(n).unwrap_or_else(|_| panic!("{n} var must be set"));
        });
    }
}
