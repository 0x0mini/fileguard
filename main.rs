use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

struct AppState {
    encrypt_tool_path: String,
    decrypt_tool_path: String,
}

#[post("/encrypt")]
async fn encrypt(mut payload: Multipart, data: web::Data<AppState>) -> impl Responder {
    process_file(&mut payload, &data.encrypt_tool_path, "File encrypted successfully", "Encryption failed").await
}

#[post("/decrypt")]
async fn decrypt(mut payload: Multipart, data: web::Data<AppState>) -> impl Responder {
    process_file(&mut payload, &data.decrypt_tool_path, "File decrypted successfully", "Decryption failed").await
}

async fn process_file(payload: &mut Multipart, tool_path: &str, success_message: &str, error_message: &str) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().body("Invalid file upload"),
        };
        let filepath = match field.content_disposition().get_filename() {
            Some(name) => format!("./{}", name),
            None => return HttpResponse::BadRequest().body("Filename missing from content disposition"),
        };
        let mut f = match File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to create file"),
        };

        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(data) => {
                    if let Err(_) = f.write_all(&data) {
                        return HttpResponse::InternalServerError().body("Failed to write file");
                    }
                }
                Err(_) => return HttpResponse::BadRequest().body("Error in file chunks"),
            };
        }

        let output = match Command::new(tool_path).arg(&filepath).output() {
            Ok(output) if output.status.success() => return HttpResponse::Ok().body(success_message),
            Ok(_) => return HttpResponse::InternalServerError().body(error_message),
            Err(_) => return HttpResponse::InternalServerError().body("Failed to execute process"),
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