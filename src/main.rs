mod cli;
mod domain;
mod db;

use cli::auth::{ signin, signup };
use cli::helpers::{ ask_yes_no, clear_screen, timezone_user };
use db::connection::connect;
use domain::User;
use rusqlite::{ Connection };

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

    if ask_yes_no("Do you wanna see datetime data?") {
        timezone_user(current_user).unwrap_or(());
    }

    Ok(())
}
