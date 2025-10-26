use bcrypt::{DEFAULT_COST, hash};

/// Hash a plaintext password using bcrypt.
///
/// # Panics
/// Will panic if hashing fails (e.g., low system entropy).
pub fn hash_password(plain: &str) -> String {
    hash(plain, DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(plain: &str, hashed: &str) -> bool {
    // Example using argon2 or bcrypt (choose based on your hash_password implementation)
    // If you use bcrypt:
    bcrypt::verify(plain, hashed).unwrap_or(false)
    // If you use argon2:
    // argon2::verify_encoded(hashed, plain.as_bytes()).unwrap_or(false)
}
