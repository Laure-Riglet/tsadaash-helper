use rusqlite::{Connection, Result};

pub fn connect() -> Result<Connection> {
    let conn: Connection = Connection::open("data/app.db")?;

    // Create a tiny table to verify everything works
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS people (
                id INTEGER PRIMARY KEY, 
                username TEXT NOT NULL, 
                email TEXT NOT NULL, 
                password TEXT NOT NULL, 
                tz_continent TEXT NOT NULL, 
                tz_city TEXT NOT NULL, 
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        [],
    )?;

    return Ok(conn);
}