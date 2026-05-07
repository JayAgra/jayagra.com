use actix_files::NamedFile;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, Error as ActixError, HttpRequest, HttpResponse, HttpServer, Result, error::{ErrorBadRequest, ErrorInternalServerError}, middleware::{self, DefaultHeaders}, web, mime};
use actix_web_static_files::ResourceFiles;
use std::{env, io, path::PathBuf, str::FromStr};

mod resources;
mod gc;
mod gc_interface;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn static_ish(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = ("static-ish/".to_owned() + req.match_info().query("filename")).parse::<PathBuf>().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn get_public_key(req: HttpRequest) -> Result<HttpResponse, ActixError> {
    let requested_type: &str =
        req.match_info().get("key_type")
            .ok_or_else(|| ErrorBadRequest("Type parameter malformed"))
            .map_err(|_| {
                ErrorBadRequest("Type parameter cannot be processed")
            })?;
    
    let requested: usize = 
        req.match_info().get("id")
            .ok_or_else(|| ErrorBadRequest("Invalid object requested"))
            .and_then(|s| s.parse::<usize>().map_err(|_| {
                ErrorBadRequest("Malformed object identifier provided")
            }))?;

    if requested_type == "s_mime" {
        let supported: String =
            env::var("S_MIME").ok()
                .ok_or_else(|| ErrorBadRequest("Server config error"))?;
                
        let list: Vec<&str> = supported.split(",").collect::<Vec<&str>>();
        
        if requested >= list.len() {
            return Err(ErrorBadRequest("Requested certificate does not exist"))
        }

        let path: PathBuf = 
            ("keys/".to_owned() + list[requested]).parse::<PathBuf>()
                .map_err(|_| {
                    ErrorInternalServerError("Error finding requested certificate")
                })?;

        let response = NamedFile::open(path)
            .map_err(|_| {
                ErrorInternalServerError("Error reading requested certificate")
            })?
            .set_content_type(
                mime::Mime::from_str("application/pkix-cert")
                .unwrap_or(mime::APPLICATION_OCTET_STREAM)
            )
            .into_response(&req);


        Ok(response)
    } else if requested_type == "pgp" {
        let supported: String =
            env::var("PGP").ok()
                .ok_or_else(|| ErrorBadRequest("Server config error"))?;
                
        let list: Vec<&str> = supported.split(",").collect::<Vec<&str>>();
        
        if requested >= list.len() {
            return Err(ErrorBadRequest("Requested key does not exist"))
        }

        let path: PathBuf = 
            ("keys/".to_owned() + list[requested]).parse::<PathBuf>()
                .map_err(|_| {
                    ErrorInternalServerError("Error finding requested key")
                })?;

        let response = NamedFile::open(path)
            .map_err(|_| {
                ErrorInternalServerError("Error reading requested key")
            })?
            .set_content_type(
                mime::Mime::from_str("application/pgp")
                .unwrap_or(mime::APPLICATION_OCTET_STREAM)
            )
            .into_response(&req);

        Ok(response)
    } else {
        Err(ErrorBadRequest("Invalid type parameter"))
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // ratelimiting with governor
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_second(100000)
        .requests_per_hour(10000000)
        .finish()
        .unwrap();

    dotenvy::dotenv().ok();

    // config done. now, create the new HttpServer
    log::info!("[OK] starting jayagra.com on port 8080");

    HttpServer::new(move || {
        // generated resources from actix_web_files
        let generated = generate();
        // other static directories
        App::new()
            // use governor ratelimiting as middleware
            .wrap(Governor::new(&governor_conf))
            // logging middleware
            .wrap(middleware::Logger::default())
            // default headers for caching. overridden on most all api endpoints
            .wrap(
                DefaultHeaders::new()
                    .add(("Cache-Control", "public, max-age=2592000")) // 30 days
            )
            // html
            .route("/", web::get().to(resources::static_index))
            .route("/tools", web::get().to(resources::static_tools))
            .route("/about", web::get().to(resources::static_about))
            .route("/projects", web::get().to(resources::static_projects))
            .route("/misc", web::get().to(resources::static_miscellaneous))
            .route("/minesweeper", web::get().to(resources::static_minesweeper))
            .route("/snake", web::get().to(resources::static_snake))
            .route("/base64", web::get().to(resources::static_base64))
            .route("/timestamp", web::get().to(resources::static_timestamp))
            .route("/keys", web::get().to(resources::static_keys))
            .route("/support{args}*", web::get().to(resources::static_support))
            .route("/ec", web::get().to(resources::static_environmental_cycles))
            .route("/legal/privacy{args}*", web::get().to(resources::static_privacy))
            .route("/legal/cookies{args}*", web::get().to(resources::static_cookies))
            .route("/legal/tos{args}*", web::get().to(resources::static_tos))
            .route("/wordle.min.js", web::get().to(resources::static_wordle_js))
            .route("/sitemap.xml", web::get().to(resources::static_sitemap_xml))
            .route("/api/gc", web::post().to(gc_interface::great_circle_post))
            .route("/api/gc/{lat_1}/{lon_1}/{lat_2}/{lon_2}", web::get().to(gc_interface::great_circle_get))
            // icons
            .route("/android-chrome-192x192.png", web::get().to(resources::static_android_chrome_192))
            .route("/apple-touch-icon.png", web::get().to(resources::static_apple_touch_icon))
            .route("/favicon-16x16.png", web::get().to(resources::static_favicon_16))
            .route("/favicon-32x32.png", web::get().to(resources::static_favicon_32))
            .route("/favicon.ico", web::get().to(resources::static_favicon))
            // public/static
            .service(ResourceFiles::new("/static", generated).do_not_resolve_defaults())
            // other shit
            .route("/static-ish/{filename}", web::get().to(static_ish))
            .route("/keys/{key_type}/{id}", web::get().to(get_public_key))
    })
    .bind("0.0.0.0:8080")?
    .workers(8)
    .run()
    .await
}
