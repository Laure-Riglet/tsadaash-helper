#[derive(Debug, Clone)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    tz_continent: String,
    tz_city: String,
}

impl User {
    pub fn new(
        id: i32,
        name: String,
        email: String,
        password: String,
        tz_continent: String,
        tz_city: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password,
            tz_continent,
            tz_city,
        }
    }

    // Getters
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
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
}