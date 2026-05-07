use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::resources;

pub const SUSPICIOUS: &[&str] = &[
    ".env", ".php", ".asp", ".aspx", ".jsp", ".cgi", ".sqlite", ".tmp", ".old", "admin", "login", ".git", "../"
];

pub async fn is_suspicious(req: HttpRequest) -> impl Responder {    
    let path = req.path()[..req.path().len().min(2048)].to_lowercase();

    if SUSPICIOUS.iter().any(|p| path.contains(p)) {
        HttpResponse::Unauthorized().body("fuck you")
    } else {
        resources::static_not_found().await
    }
}
