use crate::helpers::create_app;
use doe_sangue_backend::routes::{User, UserId};
use reqwest::Response;
use std::collections::HashMap;
use sqlx::{types::Uuid, Row};

#[actix_rt::test]
async fn create_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("name", "André");
    map.insert("email", "andre@email.com");
    map.insert("role", "Doador");
    
    let response = client
        .post(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    
        assert_eq!(200, response.status().as_u16());
    // salva o user_id que é retornado na resposta da criação do usuário
    let user_id: UserId = response.json().await.unwrap();

    let user_from_backend = sqlx::query!("SELECT email, name, role FROM users WHERE id = $1", user_id.id)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(user_from_backend.name, "André");
    assert_eq!(user_from_backend.email, "andre@email.com");
    assert_eq!(user_from_backend.role, "Doador");
}

#[actix_rt::test]
async fn get_all_users_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // faz o pedido de todos os usuários na rota /users
    let response = client
        .get(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // verifica se foram retornados 2 usuários
    let users: Vec<User> = response.json().await.unwrap();
    assert_eq!(2, users.len());
}

#[actix_rt::test]
async fn get_user_by_id_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // id do usuário salvo no script do banco
    let username = String::from("Usuário Teste");
    let user_id = "b4fff169-b165-4ca3-bff4-1f1b437123a0";

    // consulta o usuário criado usando HTTP GET pela rota /users/{id}
    let response = client
        .get(&format!("{}/users/{}", &app.address, user_id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved_user: User = response.json().await.unwrap();

    // verifica se o username do usuário retornado é igual ao que foi criado
    assert_eq!(saved_user.name, username);
}

#[actix_rt::test]
async fn update_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

     // id do usuário salvo no script do banco
     let user_id = "b4fff169-b165-4ca3-bff4-1f1b437123a0";

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut updated_user = HashMap::new();
    updated_user.insert("email", "update_name@gmail.com");
    updated_user.insert("name", "Update Name");
    updated_user.insert("role", "Admin");


    let response = client
    .put(&format!("{}/usuarios/{}", &app.address, user_id))
    .header("Content-Type", "application/json")
    .json(&updated_user)
    .send()
    .await
    .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let user_uuid: Uuid = Uuid::parse_str(&user_id).unwrap();

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT name, email, role FROM users WHERE id = $1", user_uuid)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.name, "Update Name");
    assert_eq!(saved.email, "update_name@gmail.com");
    assert_eq!(saved.role, "Admin");
}

#[actix_rt::test]
async fn delete_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

    // id do usuário salvo no script do banco
    let user_id = "b4fff169-b165-4ca3-bff4-1f1b437123a0";

    let response = client
        .delete(&format!("{}/users/{}", &app.address, user_id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi removido
    let count: i64 = sqlx::query("SELECT COUNT(name) as count FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o usuário não foi removido, levantando falha
    assert_eq!(count, 1);
}
