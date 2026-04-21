use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use std::collections::HashMap;
use std::env;

//
// 🔐 VERIFICACIÓN DEL WEBHOOK (Meta)
//
#[get("/webhook")]
async fn verify(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let mode = query.get("hub.mode");
    let token = query.get("hub.verify_token");
    let challenge = query.get("hub.challenge");

    let verify_token =
        env::var("VERIFY_TOKEN").unwrap_or_else(|_| "mi_token_seguro".to_string());

    if mode == Some(&"subscribe".to_string()) && token == Some(&verify_token) {
        if let Some(ch) = challenge {
            return HttpResponse::Ok().body(ch.to_string()); // 👈 clave
        }
    }

    HttpResponse::Unauthorized().finish()
}

//
// 📩 RECEPCIÓN DE MENSAJES
//
#[post("/webhook")]
async fn receive(body: String) -> HttpResponse {
    println!("📩 Mensaje recibido:\n{}", body);
    HttpResponse::Ok().finish()
}

//
// 🚀 SERVIDOR
//
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🤖 Bot activo en puerto 3000");

    HttpServer::new(|| {
        App::new()
            .service(verify)   // 👈 importante
            .service(receive)  // 👈 importante
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
