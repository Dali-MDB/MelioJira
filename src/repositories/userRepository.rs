use crate::models::user::User;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn exists_by_id(pool: &MySqlPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query("SELECT 1 FROM Users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok((exists.is_some()))
}

pub async fn exists_by_email(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query("SELECT 1 FROM Users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok((exists.is_some()))
}

pub async fn fetch_user_by_id(pool: &MySqlPool, id: Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM Users
        WHERE id = ?;",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    let Some(user) = user else {
        //unwrap the Option<User>
        return Err(sqlx::Error::RowNotFound);
    };
    Ok(user)
}

pub async fn fetch_user_by_email(pool: &MySqlPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM Users
        WHERE email = ?;",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    let Some(user) = user else {
        //unwrap the Option<User>
        return Err(sqlx::Error::RowNotFound);
    };
    Ok(user)
}

pub async fn fetch_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM Users;")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn insert_user(
    pool: &MySqlPool,
    user_name: &str,
    email: &str,
    hashed_password: &str,
) -> Result<User, sqlx::Error> {
    let user = User {
        id: Uuid::new_v4(), //generate a new uuid
        user_name: String::from(user_name),
        email: String::from(email),
        password: String::from(hashed_password),
    };
    sqlx::query(
        "INSERT INTO Users (id, user_name, email, password)
            VALUES (?, ?, ?, ?);",
    )
    .bind(user.id)
    .bind(&user.user_name)
    .bind(&user.email)
    .bind(&user.password)
    .execute(pool)
    .await?;

    Ok(user)
}

pub async fn update_user_profile(
    pool: &MySqlPool,
    id: Uuid,
    user_name: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    /*
    IN THE CASE OF A PARTIAL UPDATE YOU PASS THE OLD VALUES OF THE ATTRIBUTES
    */
    sqlx::query(
        "
        UPDATE Users
        SET user_name = ?, email = ?
        WHERE id = ?;
    ",
    )
    .bind(user_name)
    .bind(email)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user_by_id(pool: &MySqlPool, id: Uuid) -> Result<(), sqlx::Error> {
    let rslt = sqlx::query(
        "DELETE FROM Users
             WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
