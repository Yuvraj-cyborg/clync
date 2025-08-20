use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct ClipboardData {
    content: String,
}

struct AppState {
    clipboard_data: Mutex<HashMap<String, ClipboardData>>,
}

async fn sync_clipboard(
    data: web::Json<ClipboardData>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut clipboard_data = state.clipboard_data.lock().unwrap();
    clipboard_data.insert("shared".to_string(), data.into_inner());
    HttpResponse::Ok().json("Clipboard synced successfully")
}

async fn get_clipboard(state: web::Data<AppState>) -> impl Responder {
    let clipboard_data = state.clipboard_data.lock().unwrap();
    if let Some(data) = clipboard_data.get("shared") {
        HttpResponse::Ok().json(data)
    } else {
        HttpResponse::NotFound().json("No clipboard data available")
    }
}

pub async fn start_server(port: u16) -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        clipboard_data: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/sync", web::post().to(sync_clipboard))
            .route("/get", web::get().to(get_clipboard))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
