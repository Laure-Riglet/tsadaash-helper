use rusqlite::{Connection};
use std::io;

fn main() {

    println!("What's your name?");

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim();

    println!("What's your email?");

    let mut email: String = String::new();

    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read input");

    let email: &str = email.trim();
    
    println!("Data received:");
    println!("Name: {}", name);
    println!("Email: {}", email);

    println!("Is this correct? [n]");
    let mut confirmation = String::new();

    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read input");

    let confirmation: String = confirmation.trim().to_lowercase();
    
    if confirmation != "y" && confirmation != "yes" {
        println!("Data not confirmed, exiting.");
        return;
    }

    println!("Thank you, data confirmed!");

    let conn: Connection = Connection::open("data/app.db").expect("Failed to open database");

    // Create a tiny table to verify everything works
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL)",
        [],
    ).expect("Failed to create people table");

    conn.execute(
        "INSERT INTO people (name, email) VALUES (?, ?)",
        [name, email],
    ).expect("Failed to insert data into people table");

}