#[derive(Debug, Clone)]

pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub is_recurring: bool,
    pub recurrence_interval: Option<String>,
    pub recurrence_unit: Option<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    pub fn new(
        id: i32,
        user_id: i32,
        title: String,
        is_recurring: bool,
        recurrence_interval: Option<String>,
        recurrence_unit: Option<String>,
        from_time: Option<String>,
        to_time: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            is_recurring,
            recurrence_interval,
            recurrence_unit,
            from_time,
            to_time,
            start_date,
            end_date,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn is_recurring(&self) -> bool {
        self.is_recurring
    }
    pub fn recurrence_interval(&self) -> Option<&String> {
        self.recurrence_interval.as_ref()
    }
    pub fn recurrence_unit(&self) -> Option<&String> {
        self.recurrence_unit.as_ref()
    }
    pub fn from_time(&self) -> Option<&String> {
        self.from_time.as_ref()
    }
    pub fn to_time(&self) -> Option<&String> {
        self.to_time.as_ref()
    }
    pub fn start_date(&self) -> Option<&String> {
        self.start_date.as_ref()
    }
    pub fn end_date(&self) -> Option<&String> {
        self.end_date.as_ref()
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}