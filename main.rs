use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

struct AppState {
    encrypt_tool_path: String,
    decrypt_tool_path: String,
}

#[post("/encrypt")]
async fn encrypt(mut payload: Multipart, data: web::Data<AppState>) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let filepath = format!("./{}", field.content_disposition().get_filename().unwrap());
        let mut f = File::create(filepath.clone()).unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }

        let output = Command::new(&data.encrypt_tool_path)
            .arg(&filepath)
            .output()
            .expect("failed to execute process");

        return match output.status.success() {
            true => HttpResponse::Ok().body("File encrypted successfully"),
            false => HttpResponse::InternalServerError().body("Encryption failed"),
        };
    }

    HttpResponse::BadRequest().body("No file uploaded")
}

#[post("/decrypt")]
async fn decrypt(mut payload: Multipart, data: web::Data<AppState>) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let filepath = format!("./{}", field.content_disposition().get_filename().unwrap());
        let mut f = File::create(filepath.clone()).unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }

        let output = Command::new(&data.decrypt_tool_path)
            .arg(&filepath)
            .output()
            .expect("failed to execute process");

        return match output.status.success() {
            true => HttpResponse::Ok().body("File decrypted successfully"),
            false => HttpResponse::InternalServerError().body("Decryption failed"),
        };
    }

    HttpResponse::BadRequest().body("No file uploaded")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let encrypt_tool_path = env::var("ENCRYPT_TOOL_PATH").expect("ENCRYPT_TOOL_PATH is not set in .env");
    let decrypt_tool_path = env::var("DECRYPT_TOOL_PATH").expect("DECRYPT_TOOL_PATH is not set in .env");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                encrypt_tool_path: encrypt_tool_path.clone(),
                decrypt_tool_path: decrypt_tool_path.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(encrypt)
            .service(decrypt)
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}