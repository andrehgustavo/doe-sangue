use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};
//use uuid::Uuid;
use super::serializers::my_uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub role: String,
}


#[derive(Serialize, Deserialize)]
pub struct UserId {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
}

#[tracing::instrument(name = "Add New User", skip(user, pool))]
pub async fn add_user(
    user: web::Json<UserData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let row = sqlx::query!(
        r#"
        INSERT INTO users (id, name, email, role)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        Uuid::new_v4(),
        user.name,
        user.email,
        user.role,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let temp_id = UserId {
        id: row.id,
    };
    Ok(HttpResponse::Ok().json(&temp_id))
}

#[tracing::instrument(name = "List all users", skip(pool))]
pub async fn list_all(pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, email, role
        FROM users
        ORDER BY name
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let mut users: Vec<User> = Vec::new();
    for row in rows {
        let user = User {
            id: row.id,
            name: row.name,
            email: row.email,
            role: row.role,
        };
        users.push(user);
    }

    Ok(HttpResponse::Ok().json(users))
}

#[tracing::instrument(name = "Get user by id", skip(req, pool))]
pub async fn get_user(
    req: web::HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();

    let row = sqlx::query!(
        r#"
        SELECT id, name, email, role
        FROM users
        WHERE id = $1
        "#,
        id,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    let user_temp = User {
        id: row.id,
        name: row.name,
        email: row.email,
        role: row.role,
    };
    Ok(HttpResponse::Ok().json(&user_temp))
}

#[tracing::instrument(name = "Update user", skip(user, pool))]
pub async fn edit_user(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, email = $2, role = $3
        WHERE id = $4
        "#,
        user.name,
        user.email,
        user.role,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Delete user", skip(req, pool))]
pub async fn delete_user(
    req: web::HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let id: Uuid = req.match_info().get("id").unwrap().parse().unwrap();
    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}
