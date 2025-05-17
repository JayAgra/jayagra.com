use actix_web::{
    http::header::{CacheControl, CacheDirective, ContentType},
    HttpResponse,
};
use std::{include_bytes, include_str};

const INDEX_HTML: &str = include_str!("../public/index.html");
const MINESWEEPER_HTML: &str = include_str!("../public/minesweeper.html");
const SNEK_HTML: &str = include_str!("../public/snek.html");
const TWENTY48_HTML: &str = include_str!("../public/2048.html");
const TWENTY48_8_HTML: &str = include_str!("../public/2048_8.html");
const BASE64_HTML: &str = include_str!("../public/base64.html");
const ENVIRONMENT_CYCLES_HTML: &str = include_str!("../public/ec.html");
const APP_SUPPORT_HTML: &str = include_str!("../public/support.html");
const PRIVACY_HTML: &str = include_str!("../public/privacy.html");
const COOKIES_HTML: &str = include_str!("../public/cookies.html");
const SCOUTING_HTML: &str = include_str!("../public/scouting.html");
const WORDLE_JS: &str = include_str!("../public/wordle.min.js");
const SITEMAP_XML: &str = include_str!("../public/sitemap.xml");
const SITE_WEBMANI: &str = include_str!("../public/site.webmanifest");
// icons aren't utf8 so it needs include bytes instead
const ANDROID_CHROME_192: &[u8] = include_bytes!("../public/android-chrome-192x192.png");
// const ANDROID_CHROME_512: &[u8] = include_bytes!("../public/android-chrome-512x512.png");
const APPLE_TOUCH_ICON: &[u8] = include_bytes!("../public/apple-touch-icon.png");
const FAVICON_16: &[u8] = include_bytes!("../public/favicon-16x16.png");
const FAVICON_32: &[u8] = include_bytes!("../public/favicon-32x32.png");
const FAVICON_ICO: &[u8] = include_bytes!("../public/favicon.ico");

pub async fn static_index(/*req: HttpRequest*/) -> HttpResponse {
    // redirect requests not to port 443 (port 80) to 443
    /*match req.app_config().local_addr().port() {
        443 => {*/
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(23328000u32)]))
                .body(INDEX_HTML)
        /*}
        _ => {
            HttpResponse::PermanentRedirect()
                .append_header(("location", format!("https://{}", env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()))))
                .finish()
        }
    } */
}

pub async fn static_minesweeper() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(MINESWEEPER_HTML)
}

pub async fn static_snek() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(SNEK_HTML)
}

pub async fn static_2048() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(TWENTY48_HTML)
}

pub async fn static_2048_8() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(TWENTY48_8_HTML)
}

pub async fn static_base64() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(BASE64_HTML)
}

pub async fn static_environmental_cycles() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(ENVIRONMENT_CYCLES_HTML)
}

pub async fn static_support() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(APP_SUPPORT_HTML)
}

pub async fn static_privacy() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(PRIVACY_HTML)
}

pub async fn static_cookies() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(COOKIES_HTML)
}

pub async fn static_scouting() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(SCOUTING_HTML)
}

pub async fn static_wordle_js() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(WORDLE_JS)
}

pub async fn static_sitemap_xml() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(SITEMAP_XML)
}

pub async fn static_site_webmanifest() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(SITE_WEBMANI)
}

// serve static favicons
pub async fn static_android_chrome_192() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(4838400u32)]))
        .body(ANDROID_CHROME_192)
}

pub async fn static_apple_touch_icon() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(4838400u32)]))
        .body(APPLE_TOUCH_ICON)
}

pub async fn static_favicon_16() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(4838400u32)]))
        .body(FAVICON_16)
}

pub async fn static_favicon_32() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(4838400u32)]))
        .body(FAVICON_32)
}

pub async fn static_favicon() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Content-Type", "image/x-icon"))
        .insert_header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(4838400u32)]))
        .body(FAVICON_ICO)
}