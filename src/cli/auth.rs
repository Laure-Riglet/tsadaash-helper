use crate::domain::{Continents, User};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use inquire::{
    autocompletion::Replacement, validator::Validation, Autocomplete, Confirm, CustomUserError,
    InquireError, Password, PasswordDisplayMode, Select, Text,
};

use serde_json::{from_str, Value};
use rusqlite::{Connection, OptionalExtension, Result};

fn get_argon2_instance() -> Argon2<'static> {
    //            Regarding Argon2 parameters:
    //            ----------------------------
    //            For API keys / tokens:
    //            - higher m is OK (e.g., 96 MB --> 98304 KiB, 128 MB --> 131072 KiB)
    //            - t can be lower
    //            - UX doesn’t matter as much
    //
    //            For low-RAM environments:
    //            - don’t go above ~32 MB (32768 KiB)
    //            - keep t ≥ 2

    let params = Params::new(
        65536, // m: memory in KiB (64 MB)
        3,     // t: iterations
        1,     // p: parallelism
        None,  // output length (None = default)
    )
    .expect("invalid Argon2 params");

    return Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
}

pub fn signup(conn: &Connection) -> Result<User> {
    // --- tiny helpers (MVP style: keep inside signup) ---

    fn yes(prompt: &str) -> bool {
        //let answer = read_line_trimmed(prompt).to_lowercase();
        // matches!(answer.as_str(), "y" | "yes")
        let ans = Confirm::new(prompt).with_default(true).prompt();

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

    fn get_cities_for_continent(continent: &str) -> Vec<String> {
        let cities: Value =
            from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/tz_cities.json"))).expect("Failed to parse tz.json");
        let mut city_list: Vec<String> = Vec::new();
        if let Value::Object(map) = cities {
            if let Some(Value::Array(city_array)) = map.get(continent) {
                for city in city_array {
                    if let Value::String(city_name) = city {
                        city_list.push(city_name.clone());
                    }
                }
            }
        }
        city_list
    }

    fn ask_confirmed_city(continent: &str) -> String {
        #[derive(Clone)]
        struct CityAutocomplete {
            cities: Vec<String>,
        }

        impl Autocomplete for CityAutocomplete {
            fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
                let input_lc = input.to_lowercase();

                Ok(self
                    .cities
                    .iter()
                    .filter(|city| city.to_lowercase().starts_with(&input_lc))
                    .cloned()
                    .collect())
            }

            fn get_completion(
                &mut self,
                input: &str,
                highlighted_suggestion: Option<String>,
            ) -> Result<Replacement, CustomUserError> {
                // Replacement is likely Option<String> in your inquire version
                Ok(highlighted_suggestion.or_else(|| Some(input.to_string())))
            }
        }

        loop {
            let cities: Vec<String> = get_cities_for_continent(continent);
            let ac = CityAutocomplete { cities };

            let input = Text::new("What's your time zone city?")
                .with_placeholder("Type your answer here")
                .with_autocomplete(ac)
                .prompt()
                .unwrap_or_default();

            println!("You entered: Time zone city ➡️  {}", input);

            if yes("Is this correct?") {
                return input;
            }

            println!("Ok, let's try again.\n");
        }
    }

    fn ask_continent_confirmed() -> String {
        let options: Vec<String> = Continents::vec().iter().map(|s| s.to_string()).collect();

        loop {
            let ans: Result<String, InquireError> =
                Select::new("Choose your continent:", options.clone()).prompt();

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

    fn ask_confirmed_password(field_pretty: &str, question: &str) -> String {
        fn encrypt_password(password: &str) -> Option<String> {
            let salt = SaltString::generate(&mut OsRng);
            let argon2_instance = get_argon2_instance();
            return argon2_instance
                .hash_password(password.as_bytes(), &salt)
                .ok()
                .map(|hash| hash.to_string());
        }

        loop {
            let validator = |input: &str| {
                if input.chars().count() < 10 {
                    Ok(Validation::Invalid(
                        "Keys must have at least 10 characters.".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            };

            let name = Password::new(question)
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_custom_confirmation_message(&format!("{} (confirm):", field_pretty))
                .with_custom_confirmation_error_message("The keys don't match.")
                .with_validator(validator)
                .with_help_message("It is recommended to generate a new one only for this purpose")
                .prompt();

            match name {
                Ok(password) => return encrypt_password(&password).unwrap_or_default(),
                Err(_) => println!("An error happened when asking for your key, try again later."),
            }
        }
    }

    // --- main signup flow: keep asking until user confirms everything ---

    loop {
        let username = ask_confirmed_text("Username", "What's your username?");
        let email = ask_confirmed_text("Email", "What's your email?");
        let password = ask_confirmed_password("Password", "What's your password?");
        let tz_continent = ask_continent_confirmed();
        let tz_city = ask_confirmed_city(&tz_continent);

        println!("\nSummary:");
        println!("Username: {}", username);
        println!("Email: {}", email);
        println!("Time zone: {}/{}", tz_continent, tz_city);

        if yes("Confirm signup?") {
            // Insert
            conn.execute(
                "INSERT INTO people (username, email, password, tz_continent, tz_city) VALUES (?1, ?2, ?3, ?4, ?5)",
                (&username, &email, &password, &tz_continent, &tz_city),
            )?;

            // Build User (works if your User has pub fields; otherwise use User::new(...))
            let id = conn.last_insert_rowid() as i32;

            let user = User::new(id, username, email, password, tz_continent, tz_city);

            println!("\nSignup complete! Welcome, {}!", user.name());
            return Ok(user);
        }

        println!("\nOk — restarting signup.\n");
    }
}

pub fn signin(conn: &Connection) -> Result<Option<User>> {
    let identifier = Text::new("Username or Email:")
        .with_placeholder("Type your username or email here")
        .prompt()
        .unwrap_or_default()
        .trim()
        .to_string();

    let password_input = Password::new("Password:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap_or_default();

    let user: Option<User> = conn
        .query_row(
            "SELECT id, username, email, password, tz_continent, tz_city
             FROM people
             WHERE username = ?1 OR email = ?1",
            [&identifier],
            |row| {
                Ok(User::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .optional()?;

    let generic_msg = "We couldn't verify your account with the provided credentials.";

    let argon2 = get_argon2_instance();

    // A *valid* dummy hash generated once using the same Argon2 params you use in production.
    // (Generate it once with your app and paste it here.)
    const DUMMY_HASH: &str = "$argon2id$v=19$m=65536,t=3,p=1$2aYZPLsX/K0wjEZ1Hy6leg$ZxY80K0Lq3nS/PKsOciRJodOH9u8BRVdiAhjKFDUbCE";

    let dummy_parsed =
        PasswordHash::new(DUMMY_HASH).expect("DUMMY_HASH must be a valid PHC string");

    let user_hash = user
        .as_ref()
        .and_then(|u| PasswordHash::new(u.password()).ok());

    let parsed_hash: &PasswordHash = user_hash.as_ref().unwrap_or(&dummy_parsed);

    let ok = argon2
        .verify_password(password_input.as_bytes(), parsed_hash)
        .is_ok();

    if ok {
        if let Some(u) = user {
            return Ok(Some(u));
        }
    }

    println!("{}", generic_msg);
    Ok(None)
}