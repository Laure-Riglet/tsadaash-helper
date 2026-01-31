use chrono::{DateTime, Local, Utc};
use inquire::{Confirm, Text, Select, InquireError};
use rusqlite::{Connection, OptionalExtension, Result};
use std::io;
use std::time::SystemTime;
use tsadaash::domain::{Continents, User};

fn connect() -> Result<Connection> {
    let conn: Connection = Connection::open("data/app.db")?;

    // Create a tiny table to verify everything works
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL, tz_continent TEXT NOT NULL, tz_city TEXT NOT NULL)",
        [],
    )?;

    return Ok(conn);
}

fn signup(conn: &Connection) -> Result<User> {
    // --- tiny helpers (MVP style: keep inside signup) ---

    fn yes(prompt: &str) -> bool {
        //let answer = read_line_trimmed(prompt).to_lowercase();
        // matches!(answer.as_str(), "y" | "yes")
        let ans = Confirm::new(prompt)
            .with_default(true)
            .prompt();

        match ans {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => {
                println!("Error reading input, assuming 'No'");
                false
            }
        }
    }

    fn ask_confirmed_text(field_pretty: &str, question: &str) -> String {
        loop {
            //let input = read_line_trimmed(question);
            let input = Text::new(question)
                .with_placeholder("Type your answer here")
                .prompt()
                .unwrap_or_default();

            println!("You entered: {} ➡️  {}", field_pretty, input);

            if yes("Is this correct?") {
                return input;
            }

            println!("Ok, let's try again.\n");
        }
    }

    fn ask_continent_confirmed() -> String {

    let options: Vec<String> = Continents::vec().iter().map(|s| s.to_string()).collect();
        
        loop {

            let ans: Result<String, InquireError> = Select::new("Choose your continent:", options.clone())
                .prompt();

            let continent: String = match ans {
                Ok(choice) => choice,
                Err(_) => {
                    println!("Error reading input, try again.\n");
                    continue;
                }
            };

            println!("You entered: Continent ➡️  {}", continent);

            if yes("Is this correct?") {
                return continent;
            }

            println!("Ok, let's try again.\n");
        }
    }

    // --- main signup flow: keep asking until user confirms everything ---

    loop {
        let name = ask_confirmed_text("Name", "What's your name?");
        let email = ask_confirmed_text("Email", "What's your email?");
        let tz_continent = ask_continent_confirmed();
        let tz_city = ask_confirmed_text(
            "Time zone city",
            "What's your time zone city?",
        );

        println!("\nSummary:");
        println!("Name: {}", name);
        println!("Email: {}", email);
        println!("Time zone: {}/{}", tz_continent, tz_city);

        if yes("Confirm signup?") {
            // Insert
            conn.execute(
                "INSERT INTO people (name, email, tz_continent, tz_city) VALUES (?1, ?2, ?3, ?4)",
                (&name, &email, &tz_continent, &tz_city),
            )?;

            // Build User (works if your User has pub fields; otherwise use User::new(...))
            let id = conn.last_insert_rowid() as i32;

            let user = User::new(id, name, email, tz_continent, tz_city);

            println!("\nSignup complete! Welcome, {}!", user.name());
            return Ok(user);
        }

        println!("\nOk — restarting signup.\n");
    }
}

fn signin(conn: &Connection) -> Result<Option<User>> {
    println!("What's your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim().to_string();

    let user = conn
        .query_row(
            "SELECT id, name, email, tz_continent, tz_city FROM people WHERE name = ?1",
            [&name],
            |row| {
                Ok(User::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            },
        )
        .optional()?; // <-- turns "no rows" into Ok(None)

    Ok(user)
}

fn ask_yes_no(prompt: &str) -> bool {
    let ans = Confirm::new(prompt)
        .with_default(false)
        .with_help_message("This is a help message")
        .prompt();

    match ans {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => {
            println!("Error reading input, assuming 'No'");
            false
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> rusqlite::Result<()> {
    let conn: Connection = connect()?;

    clear_screen();
    println!("");
    println!("=== Tsadaash ===\n");

    let current_user: User = if ask_yes_no("Are you a registered user?") {
        match signin(&conn)? {
            Some(user) => user,
            None => {
                println!("User not found. Please sign up first.");
                signup(&conn)?
            }
        }
    } else {
        signup(&conn)?
    };

    println!("Welcome, {}!", current_user.name());
    println!("Email: {}", current_user.email());
    println!(
        "TZ: {}/{}",
        current_user.tz_continent(),
        current_user.tz_city()
    );

    let now_dt_local: DateTime<Local> = SystemTime::now().into();
    println!("Local: {}", now_dt_local.format("%Y-%m-%d %H:%M:%S"));

    let tz_continent = current_user.tz_continent();
    let tz_city = current_user.tz_city();
    let tz_name = format!("{}/{}", tz_continent, tz_city); // IMPORTANT: slash

    let now_dt_tz: DateTime<chrono_tz::Tz> = match tz_name.parse::<chrono_tz::Tz>() {
        Ok(tz) => {
            let now_utc: DateTime<Utc> = SystemTime::now().into();
            now_utc.with_timezone(&tz)
        }
        Err(_) => {
            println!(
                "Warning: could not parse time zone {}. Using UTC time instead.",
                tz_name
            );
            let now_utc: DateTime<Utc> = SystemTime::now().into();
            now_utc.with_timezone(&chrono_tz::UTC)
        }
    };

    println!("In {}: {}", tz_name, now_dt_tz.format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}
