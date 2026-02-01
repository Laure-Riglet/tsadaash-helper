use inquire::Confirm;
use chrono::{ DateTime, Local, Utc };
use std::time::SystemTime;
use crate::domain::User;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn ask_yes_no(prompt: &str) -> bool {
    let ans = Confirm::new(prompt).with_default(false).prompt();

    match ans {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => {
            println!("Error reading input, assuming 'No'");
            false
        }
    }
}

pub fn timezone_user(user: User) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "TZ: {}/{}",
        user.tz_continent(),
        user.tz_city()
    );

    let now_dt_local: DateTime<Local> = SystemTime::now().into();
    println!("Local: {}", now_dt_local.format("%Y-%m-%d %H:%M:%S"));

    let tz_continent = user.tz_continent();
    let tz_city = user.tz_city();
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
