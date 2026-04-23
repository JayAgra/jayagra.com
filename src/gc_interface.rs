use actix_web::{Error as ActixError, HttpRequest, HttpResponse, Result, error::ErrorBadRequest, web};
use serde::Serialize;

use crate::gc;

// in meters & degrees
#[derive(Serialize)]
pub struct GreatCircleData {
    pub distance: f64,
    pub azimuth_a: f64,
    pub azimuth_b: f64
}

pub async fn great_circle_post(data: web::Json<[gc::Coordinate; 2]>) -> Result<HttpResponse, ActixError> {
    for coordinate in &data.0 {
        if coordinate.latitude.abs() > 90.0 || coordinate.longitude.abs() > 180.0 {
            return Ok(HttpResponse::BadRequest().body("Impossible data"))
        }
    }

    let result: (f64, f64, f64) = gc::vincenty(data.0);
    Ok(HttpResponse::Ok().json(
        GreatCircleData {
            distance: result.0,
            azimuth_a: result.1,
            azimuth_b: result.2
        }
    ))
}

pub async fn great_circle_get(req: HttpRequest) -> Result<HttpResponse, ActixError> {
    fn parse_parameter(param: Option<&str>) -> Result<f64, ActixError> {
        param
            .ok_or_else(|| ErrorBadRequest("Missing Data"))
            .and_then(|s| s.parse::<f64>().map_err(|_| {
                ErrorBadRequest("Invalid Data")
            }))
    }

    let lat_1 = parse_parameter(req.match_info().get("lat_1"))?;
    let lon_1 = parse_parameter(req.match_info().get("lon_1"))?;
    let lat_2 = parse_parameter(req.match_info().get("lat_2"))?;
    let lon_2 = parse_parameter(req.match_info().get("lon_2"))?;

    let data: [gc::Coordinate; 2] = [
        gc::Coordinate {
            latitude: lat_1,
            longitude: lon_1
        },
        gc::Coordinate {
            latitude: lat_2,
            longitude: lon_2
        }
    ];

    for coordinate in &data {
        if coordinate.latitude.abs() > 90.0 || coordinate.longitude.abs() > 180.0 {
            return Ok(HttpResponse::BadRequest().body("Impossible data"))
        }
    }

    let result: (f64, f64, f64) = gc::vincenty(data);

    Ok(HttpResponse::Ok().json(
        GreatCircleData {
            distance: result.0,
            azimuth_a: result.1,
            azimuth_b: result.2
        }
    ))
}