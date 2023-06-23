use std::fs::File;
use std::io::Write;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::fs::read_json;
use crate::get_services::{
    handle_accounts_services, handle_memberships, handle_subdomain, handle_user,
};
use crate::post_services::handle_accounts_scripts;

mod fs;
mod get_services;
mod post_services;
mod put_services;

async fn save_file(mut payload: actix_web::web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        body.extend_from_slice(&chunk);
    }

    // Génération d'un nom de fichier unique
    let file_name = format!("/uploads/uploaded_file_{}", "index.js");

    // Chemin de destination pour enregistrer le fichier
    let destination = format!("./{}", file_name);

    // Ouverture d'un nouveau fichier pour écrire les données téléchargées
    /* File::create(&destination)
            .write_all(&body)
            .expect("cannot write into file");
    */
    Ok(HttpResponse::Ok().body(read_json(
        "./src/defaultResponses/put_accounts_scripts.json",
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_user)
            .service(handle_memberships)
            .service(handle_subdomain)
            .service(handle_accounts_services)
            .service(handle_accounts_scripts)
            .route(
                "/client/v4/accounts/{accounts}/workers/scripts/{scripts}",
                web::put().to(save_file),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
