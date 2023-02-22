use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub contents: String,
    pub time: DateTime<Utc>
}