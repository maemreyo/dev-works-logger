mod git;
pub mod gql_client;
use actix_web::{
    get, post, web::Json, App, HttpResponse, HttpServer, Responder,
    Result,
};
use dotenv::dotenv;
use git::Git;

use crate::gql_client::GqlClient;

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let client = GqlClient::new_client();
    // Trigger action to get latest commits of repos
    let result = Git::get_latest_commits(
        &client,
        &dotenv::var("GITHUB_USERNAME").expect("Username not found"),
        Some(2),
        None,
    )
    .await
    .unwrap();

    Ok(Json(result))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize env variables
    dotenv().ok();

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
