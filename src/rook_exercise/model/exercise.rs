use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub id: String,
    pub user_id: String,
    pub activity: String,
    pub description: String,
    pub start_dt: NaiveDateTime,
    pub end_dt: NaiveDateTime,
    pub created_dt: DateTime<Utc>,
    pub modified_dt: DateTime<Utc>,
}