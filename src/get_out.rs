use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::resources;

pub const SUSPICIOUS: &[&str] = &[
    ".env", ".php", ".asp", ".aspx", ".jsp", ".cgi", ".sqlite", ".db", ".db-shm", ".db-wal", ".tmp", ".old", "admin", ".git", "../", "~"
];

pub async fn is_suspicious(req: HttpRequest) -> impl Responder {    
    let path = req.path()[..req.path().len().min(2048)].to_lowercase();

    if SUSPICIOUS.iter().any(|p| path.contains(p)) {
        HttpResponse::Unauthorized().body("no")
    } else {
        resources::static_not_found().await
    }
}
