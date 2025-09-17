use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
    Argon2, Params, Algorithm, Version
};

fn create_argon2() -> Result<Argon2<'static>, argon2::Error> {
    let params = Params::new(
        65536, // memory cost (64 MiB) 
        3,     // time cost (iterations)
        4,     // parallelism (match cores)
        Some(32) // output length in bytes
    )?;
    
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = create_argon2()?;
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = create_argon2()?;
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

fn main() {
    let password = "somethingsomething123";
    
    match hash_password(password) {
        Ok(hashed) => {
            println!("Hashed: {}", hashed);
            
            match verify_password(password, &hashed) {
                Ok(is_valid) => println!("Password valid: {}", is_valid),
                Err(e) => println!("Verification error: {:?}", e),
            }
            
            match verify_password("wrongpassword", &hashed) {
                Ok(is_valid) => println!("Wrong password valid: {}", is_valid),
                Err(e) => println!("Verification error: {:?}", e),
            }
        },
        Err(e) => println!("Hashing error: {:?}", e),
    }
}