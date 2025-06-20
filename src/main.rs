use actix_files::NamedFile;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::{self, DefaultHeaders}, web, App, HttpServer, HttpRequest, Result};
use actix_web_static_files;
use dotenv::dotenv;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{env, io, path::PathBuf};

mod static_files;
mod gc;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn static_ish(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = ("static-ish/".to_owned() + req.match_info().query("filename")).parse::<PathBuf>().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // load environment variables from .env file
    dotenv().ok();

    // ratelimiting with governor
    let governor_conf = GovernorConfigBuilder::default()
        // these may be a lil high but whatever
        .per_nanosecond(100)
        .burst_size(25000)
        .finish()
        .unwrap();

    /*
     *  generate a self-signed certificate for localhost (run from bearTracks directory):
     *  openssl req -x509 -newkey rsa:4096 -nodes -keyout ./ssl/key.pem -out ./ssl/cert.pem -days 365 -subj '/CN=localhost'
     */
    // create ssl builder for tls config
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("./ssl/key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("./ssl/cert.pem").unwrap();

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
                    .add(("Cache-Control", "public, max-age=23328000"))
            )
            // html
            .route("/", web::get().to(static_files::static_index))
            .route("/minesweeper", web::get().to(static_files::static_minesweeper))
            .route("/snek", web::get().to(static_files::static_snek))
            .route("/2048", web::get().to(static_files::static_2048))
            .route("/2048_8", web::get().to(static_files::static_2048_8))
            .route("/base64", web::get().to(static_files::static_base64))
            .route("/ec", web::get().to(static_files::static_environmental_cycles))
            .route("/scouting{args}}*", web::get().to(static_files::static_scouting))
            .route("/support{args}*", web::get().to(static_files::static_support))
            .route("/legal/privacy{args}*", web::get().to(static_files::static_privacy))
            .route("/legal/cookies{args}*", web::get().to(static_files::static_cookies))
            .route("/wordle.min.js", web::get().to(static_files::static_wordle_js))
            .route("/site.webmanifest", web::get().to(static_files::static_site_webmanifest))
            .route("/sitemap.xml", web::get().to(static_files::static_sitemap_xml))
            // icons
            .route("/android-chrome-192x192.png", web::get().to(static_files::static_android_chrome_192))
            .route("/apple-touch-icon.png", web::get().to(static_files::static_apple_touch_icon))
            .route("/favicon-16x16.png", web::get().to(static_files::static_favicon_16))
            .route("/favicon-32x32.png", web::get().to(static_files::static_favicon_32))
            .route("/favicon.ico", web::get().to(static_files::static_favicon))
            // public/static
            .service(actix_web_static_files::ResourceFiles::new("/static", generated))
            // other shit
            .route("/static-ish/{filename}", web::get().to(static_ish))
    })
    // .bind_openssl(format!("{}:443", env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string())), builder)?
    .bind("0.0.0.0:8080")?
    .workers(8)
    .run()
    .await
}
