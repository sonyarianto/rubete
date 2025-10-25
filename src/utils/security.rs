use bcrypt::{DEFAULT_COST, hash};

/// Hash a plaintext password using bcrypt.
///
/// # Panics
/// Will panic if hashing fails (e.g., low system entropy).
pub fn hash_password(plain: &str) -> String {
    hash(plain, DEFAULT_COST).expect("Failed to hash password")
}
