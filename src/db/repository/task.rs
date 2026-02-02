use crate::domain::Task;
use rusqlite::Connection;

pub fn select_all_by_user_id(
    conn: &Connection,
    user_id: i32,
) -> rusqlite::Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, title, description, is_recurring, recurrence_interval, recurrence_unit, 
                from_time, to_time, start_date, end_date, created_at, updated_at 
         FROM tasks 
         WHERE user_id = ?1",
    )?;
    let mut rows = stmt.query([user_id])?;

    let mut tasks = Vec::new();
    while let Some(row) = rows.next()? {
        let task = Task::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
        );
        tasks.push(task);
    }
    Ok(tasks)
}

pub fn select_by_id(
    conn: &Connection,
    task_id: i32,
) -> rusqlite::Result<Option<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, title,  is_recurring, recurrence_interval, recurrence_unit, 
                from_time, to_time, start_date, end_date, created_at, updated_at 
         FROM tasks 
         WHERE id = ?1",
    )?;
    let mut rows = stmt.query([task_id])?;

    if let Some(row) = rows.next()? {
        let task = Task::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
        );
        Ok(Some(task))
    } else {
        Ok(None)
    }
}

pub fn insert(
    conn: &Connection,
    user_id: i32,
    title: &str,
    is_recurring: bool,
    recurrence_interval: Option<&str>,
    recurrence_unit: Option<&str>,
    from_time: Option<&str>,
    to_time: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> rusqlite::Result<Task> {
    conn.execute(
        "INSERT INTO tasks (user_id, title, is_recurring, recurrence_interval, recurrence_unit, 
                            from_time, to_time, start_date, end_date) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            user_id,
            title,
            is_recurring,
            recurrence_interval,
            recurrence_unit,
            from_time,
            to_time,
            start_date,
            end_date,
        ),
    )?;

    let id = conn.last_insert_rowid() as i32;

    select_by_id(conn, id).and_then(|opt_task| {
        opt_task.ok_or(rusqlite::Error::QueryReturnedNoRows)
    })
}

pub fn delete_by_id(conn: &Connection, task_id: i32) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        [task_id],
    )?;
    Ok(())
}

pub fn update(
    conn: &Connection,
    task_id: i32,
    title: &str,
    is_recurring: bool,
    recurrence_interval: Option<&str>,
    recurrence_unit: Option<&str>,
    from_time: Option<&str>,
    to_time: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> rusqlite::Result<Task> {
    conn.execute(
        "UPDATE tasks 
         SET title = ?1, is_recurring = ?2, recurrence_interval = ?3, recurrence_unit = ?4, from_time = ?5, to_time = ?6, start_date = ?7, end_date = ?8 
         WHERE id = ?9",
        (
                    title,
            is_recurring,
            recurrence_interval,
            recurrence_unit,
            from_time,
            to_time,
            start_date,
            end_date,
            task_id,
        ),
    )?;

    select_by_id(conn, task_id).and_then(|opt_task| {
        opt_task.ok_or(rusqlite::Error::QueryReturnedNoRows)
    })
}