use crate::helpers::create_app;
use doe_sangue::routes::{User, UserId};
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

    let userFromBackend = sqlx::query!("SELECT email, name, role FROM users",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved usuario.");

    assert_eq!(userFromBackend.name, "André");
    assert_eq!(userFromBackend.email, "andre@email.com");
    assert_eq!(userFromBackend.role, "Doador");
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
    let user_id: Uuid = "b4fff169-b165-4ca3-bff4-1f1b437123a0";

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
     let username = String::from("Usuário Teste");
     let user_id: Uuid = "b4fff169-b165-4ca3-bff4-1f1b437123a0";/

    // instancia um usuário e modifica o username, mantendo o mesmo id
    let updated_user = User {
        user_id,
        email: String::from("update_name@gmail.com"),
        name: String::from("Update Name"),
        role: String::from("Admin"),
    };

    // gera um HashMap que será mapeado pro json a ser enviado na requisição de atualização
    let mut map = HashMap::new();
    map.insert("id", updated_user.id.to_string());
    map.insert("name", updated_user.name);
    map.insert("email", updated_user.email);
    map.insert("role", updated_user.role);

    let response = client
        .put(&format!("{}/users", &app.address))
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi atualizado
    let saved = sqlx::query!("SELECT username FROM users WHERE id = $1", updated_user.id)
                    .fetch_one(&app.db_pool)
                    .await
                    .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, updated_user.username);
}

#[actix_rt::test]
async fn delete_user_returns_200() {
    let app = create_app().await;
    let client = reqwest::Client::new();

     // id do usuário salvo no script do banco
     let user_id: Uuid = "b4fff169-b165-4ca3-bff4-1f1b437123a0";/

    let response = client
        .delete(&format!("{}/users/{}", &app.address, user_id))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // finalmente, verifica se o usuário foi removido
    let count: i64 = sqlx::query("SELECT COUNT(username) as count FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.")
        .try_get("count")
        .unwrap();

    // verifica se foi retornada alguma coisa, se sim, o usuário não foi removido, levantando falha
    assert_eq!(count, 1);
}
