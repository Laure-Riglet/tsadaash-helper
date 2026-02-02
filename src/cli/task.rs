use crate::cli::helpers::clear_screen;
use chrono::{NaiveDate, Weekday};
use inquire::{Confirm, DateSelect, Select, Text};
use rusqlite::Result;

pub fn menu(user_id: u32) -> Result<(), rusqlite::Error> {
    let options = vec![
        "Create Task",
        "View Tasks",
        "Update Task",
        "Delete Task",
        "Back to Main Menu",
    ];

    loop {
        clear_screen();
        println!("=== Task Management ===");
        let choice = Select::new("Please choose an option:", options.clone())
            .prompt()
            .unwrap_or_else(|_| "Back to Main Menu");

        match choice {
            "Create Task" => {
                create_task(user_id)?;
            }
            "View Tasks" => {
                println!("Viewing tasks...");
                // Implement task viewing logic here
            }
            "Update Task" => {
                println!("Updating a task...");
                // Implement task updating logic here
            }
            "Delete Task" => {
                println!("Deleting a task...");
                // Implement task deletion logic here
            }
            "Back to Main Menu" => break,
            _ => {
                println!("Invalid option. Press Enter to try again.");
                break;
            }
        }
    }

    Ok(())
}

fn create_task(user_id: u32) -> Result<()> {
    // Form
    let title = Text::new("Enter task title:")
        .with_placeholder("Type your answer here")
        .prompt()
        .unwrap_or_default();

    let is_recurring = Confirm::new("Is this task recurring?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    match is_recurring {
        false => {
            let recurrence_interval: Option<String> = None;
            let recurrence_unit: Option<String> = None;
        }
        true => {
            // Recurring task details
            let recurrence_interval = Text::new("Enter recurrence interval (e.g., '2'):")
                .with_placeholder("Type your answer here")
                .prompt()
                .unwrap_or_default();

            let recurrence_unit = Select::new(
                "Select recurrence unit:",
                vec!["days", "weeks", "months", "years"],
            )
            .prompt()
            .unwrap_or_else(|_| "days");
        }
    }

    let from_time = Text::new("Enter start time of completion (HH:MM) or leave blank:")
        .with_placeholder("Type your answer here")
        .prompt()
        .unwrap_or_default();

    let to_time = Text::new("Enter end time of completion (HH:MM) or leave blank:")
        .with_placeholder("Type your answer here")
        .prompt()
        .unwrap_or_default();

    let start_date = DateSelect::new("Select start date of task completion:")
        .with_starting_date(NaiveDate::from_ymd(2026, 2, 1))
        .with_min_date(NaiveDate::from_ymd(2026, 2, 2))
        .with_max_date(NaiveDate::from_ymd(2026, 5, 31))
        .with_week_start(Weekday::Mon)
        .prompt()
        .unwrap_or_default();

    let end_date = DateSelect::new("Select end date of task completion:")
        .with_starting_date(NaiveDate::from_ymd(2026, 2, 1))
        .with_min_date(NaiveDate::from_ymd(2026, 2, 2))
        .with_max_date(NaiveDate::from_ymd(2026, 5, 31))
        .with_week_start(Weekday::Mon)
        .prompt()
        .unwrap_or_default();

    println!("Task '{}' created successfully!", title);
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();    

    Ok(())
}
