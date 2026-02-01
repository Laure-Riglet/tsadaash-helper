#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    password: String,
    pub tz_continent: String,
    pub tz_city: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        email: String,
        password: String,
        tz_continent: String,
        tz_city: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password,
            tz_continent,
            tz_city,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn tz_continent(&self) -> &str {
        &self.tz_continent
    }
    pub fn tz_city(&self) -> &str {
        &self.tz_city
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}