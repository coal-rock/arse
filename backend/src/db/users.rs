use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::SqlitePool;

pub async fn does_admin_user_exist(db: SqlitePool) -> bool {
    sqlx::query!(r#"SELECT COUNT(*) as "number_of_users" FROM users"#)
        .fetch_one(&db)
        .await
        .unwrap()
        .number_of_users
        > 0
}

pub async fn add_user(db: SqlitePool, username: String, password: String) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        username,
        password_hash,
    )
    .execute(&db)
    .await
    .unwrap();
}

pub enum CredentialsStatus {
    UserDoesNotExist,
    PasswordIncorrect,
    Success,
}

pub async fn validate_credentials(
    db: SqlitePool,
    username: String,
    password: String,
) -> CredentialsStatus {
    let user = sqlx::query!(
        r#"SELECT password_hash FROM users WHERE username == $1"#,
        username
    )
    .fetch_optional(&db)
    .await
    .unwrap();

    let user = match user {
        Some(user) => user,
        None => return CredentialsStatus::UserDoesNotExist,
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();

    match Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        true => CredentialsStatus::Success,
        false => CredentialsStatus::PasswordIncorrect,
    }
}
