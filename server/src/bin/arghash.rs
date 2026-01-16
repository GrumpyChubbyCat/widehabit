use std::env;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::{OsRng}},
};

fn main() -> Result<(), argon2::password_hash::Error>{
    let args: Vec<String> = env::args().collect();
    let password_arg = args[1].as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash: argon2::PasswordHash<'_> = argon2
        .hash_password(password_arg, &salt)?;
    println!("Your hash is: {}", password_hash);
    Ok(())
}
