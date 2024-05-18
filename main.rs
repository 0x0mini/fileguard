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
    let mut has_processed_file = false;
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().body("Invalid file upload"),
        };

        let filepath = match field.content_disposition().get_filename() {
            Some(name) => format!("./{}", name),
            None => return HttpResponse::BadRequest().body("Filename missing from content disposition"),
        };

        match File::create(&filepath) {
            Ok(mut file) => {
                if write_file(&mut field, &mut file).await.is_err() {
                    return HttpResponse::BadRequest().body("Failed to write file");
                }
                match execute_tool(tool_path, &filepath, success_message, error_message) {
                    Ok(_) => has_processed_file = true,
                    Err(response) => return response,
                }
            }
            Err(_) => return HttpResponse::InternalServerError().body("Failed to create file"),
        }
    }

    if has_processed_file {
        HttpResponse::Ok().body(success_message)
    } else {
        HttpResponse::BadRequest().body("No file uploaded")
    }
}

async fn write_file(field: &mut actix_multipart::Field, file: &mut File) -> io::Result<()> {
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        file.write_all(&data)?;
    }
    Ok(())
}

fn execute_tool(tool_path: &str, filepath: &str, success_message: &str, error_message: &str) -> Result<(), HttpResponse> {
    let output = Command::new(tool_path)
    .arg(filepath)
    .output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => Err(HttpResponse::InternalServerError().body(error_message)),
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to execute process")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let encrypt_tool_path = env::var("ENCRYPT_TOOL_PATH").expect("ENCRYPT_TOOL_PATH is not set in .env");
    let decrypt_tool_path = env::var("DECRYPT_TOOL_PATH").expect("DECRYPT_TOOL_PATH is not set in .env");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                encrypt_tool_path: encrypt_tool_path.clone(),
                decrypt_tool_path: decrypt_tool_path.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(encrypt)
            .service(decrypt)
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}