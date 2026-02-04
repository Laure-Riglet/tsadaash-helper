use crate::domain::Periodicity;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]

pub struct Task {
    pub title: String,
    pub periodicity: Periodicity,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        periodicity: Periodicity,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            title,
            periodicity,
            created_at,
            updated_at,
        }
    }
}