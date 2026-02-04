use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub tz_continent: String,
    pub tz_city: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        tz_continent: String,
        tz_city: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            username,
            email,
            password,
            tz_continent,
            tz_city,
            created_at,
            updated_at,
        }
    }
}