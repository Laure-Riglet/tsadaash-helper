use argon2::{
    self,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use crate::domain::User;

pub fn get_argon2_instance() -> Argon2<'static> {
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

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon2_instance();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(user: Option<User>, password: &str) -> (bool, Option<User>) {
    let argon2 = get_argon2_instance();
    match user {
        Some(u) => {
            let hash_str = u.password();
            let parsed_hash = PasswordHash::new(hash_str);
            return argon2
                .verify_password(password.as_bytes(), &parsed_hash.unwrap())
                .map(|_| (true, Some(u)))
                .unwrap_or((false, None));
        }
        None => {
            // Simulate password verification to mitigate timing attacks
            let dummy_hash: &str =
                "$argon2id$v=19$m=65536,t=3,p=1$2aYZPLsX/K0wjEZ1Hy6leg$ZxY80K0Lq3nS/PKsOciRJodOH9u8BRVdiAhjKFDUbCE";
            let dummy_parsed =
                PasswordHash::new(dummy_hash);
            return argon2
                .verify_password(password.as_bytes(), &dummy_parsed.unwrap())
                .map(|_| (false, None))
                .unwrap_or((false, None));
        }
    }
}