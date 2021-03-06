use actix_web::{web, HttpResponse, HttpRequest, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
    role: String
}

pub async fn add_user(
    user: web::Form<FormData>,
    pool: web::Data<PgPool>, // Renamed!
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, role)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.email,
        user.name,
        user.role
    )
    // We got rid of the double-wrapping using .app_data()
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn list_all() -> HttpResponse{
    HttpResponse::Ok().body("Aqui serão listados todos os usuários")
}
pub async fn get_user(req: HttpRequest) -> impl Responder{
    let id = req.match_info().get("id").unwrap();
    HttpResponse::Ok().body("Listagem dos dados de um usuário específico")
}
pub async fn edit_user(req: HttpRequest) -> impl Responder{
    let id = req.match_info().get("id").unwrap();
    HttpResponse::Ok().body("Editar usuário específico")
}