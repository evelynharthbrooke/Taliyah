// pub mod color_utils;
pub mod git_utils;
pub mod locale_utils;
pub mod net_utils;
pub mod parsing_utils;

use serenity::{client::Context, model::id::UserId};
use sqlx::Row;
use std::{fs::File, io::prelude::Read};
use tracing::error;

use crate::{config::ConfigurationData, data::DatabasePool, error::EllieError};

pub fn read_config(file: &str) -> ConfigurationData {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str::<ConfigurationData>(&contents).unwrap()
}

pub async fn get_profile_field(context: &Context, field: &str, user_id: UserId) -> Result<String, EllieError> {
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
    match sqlx::query(format!("SELECT {} FROM profile_data WHERE user_id = $1", field).as_str())
        .bind(user_id.0 as i64)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => match row.try_get(0).map_err(EllieError::Database) {
            Ok(row) => Ok(row),
            Err(err) => {
                error!("Field not set in database: {}", err);
                Ok("Field not set.".to_string())
            }
        },
        Err(err) => {
            error!("Error querying database: {}", err);
            Ok("Database unsuccessfully queried.".to_string())
        }
    }
}

// converts integers to human-readable integers separated by
// commas, e.g. "1000000" displays as "1,000,000" when fed through
// this function.
pub fn format_int(int: usize) -> String {
    let mut string = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}

/// Calculates the average sum of an array of i64's.
pub fn calculate_average_sum(ints: &[i64]) -> f64 {
    ints.iter().sum::<i64>() as f64 / ints.len() as f64
}
