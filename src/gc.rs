// great circle distance calculator

use std::f64::consts::PI;

pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64
}

impl Coordinate {
    fn to_rad(&mut self) {
        self.latitude = (self.latitude * PI) / 180.0;
        self.longitude = (self.longitude * PI) / 180.0;
    }

    pub fn vincenty(self, coordinate: Coordinate) -> (f64, f64, f64) {
        vincenty([self, coordinate])
    }
}

const SEMI_MAJOR_AXIS: f64 = 6378137.0; // notation a. in meters, from WGS-84
const FLATTENING: f64 = 1.0 / 298.257223563; // notation f. from WGS-84 (unit-less)
const SEMI_MINOR_AXIS: f64 = 6356752.314245; // notation b. in meters, from WGS-84. also (1-f)a

fn vincenty(mut points: [Coordinate; 2]) -> (f64, f64, f64) {
    points.iter_mut().for_each(|p| p.to_rad());

    // reduced latitudes
    let u1 = ((1.0 - FLATTENING) * points[0].latitude.tan()).atan();
    let u2 = ((1.0 - FLATTENING) * points[1].latitude.tan()).atan();

    // reduce runtime by keeping these ready
    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    // difference in longitude (Δλ = λ2 - λ1)
    let delta_lon = points[1].longitude - points[0].longitude;

    // intermediate results are held here

    // first guess for longitude difference on auxillary sphere (λ)
    let mut lambda = delta_lon;
    // previous guess, used to check convergence
    let mut lambda_prev = 0.0;
    // maximum tries to avoid death in event of error
    let mut iter_limit = 4096;

    // angular separation (σ)
    let mut sigma: f64 = 0.0;
    
    // sine and cosine of angular separation (σ)
    let mut sin_sigma: f64 = 0.0;
    let mut cos_sigma: f64 = 0.0;

    // cosine of azimuthal angle (α)
    let mut cos2_alpha: f64 = 0.0;
    
    // cosine of corrected angular separation (σm)
    let mut cos2_sigma_m: f64 = 0.0;

    while (lambda - lambda_prev).abs() > 1e-12 && iter_limit > 0 {
        iter_limit -= 1;

        // components of the angular separation (σ)
        sin_sigma = ((cos_u2 * lambda.sin()).powi(2) + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * lambda.cos()).powi(2)).sqrt(); // sin(σ)
        cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * lambda.cos(); // cos(σ)
        sigma = sin_sigma.atan2(cos_sigma); // angular separation (σ)

        // azimuthal angle (α) of the geodesic
        let sin_alpha = cos_u1 * cos_u2 * lambda.sin() / sin_sigma; // sin(α)
        cos2_alpha = 1.0 - sin_alpha.powi(2); // cos²(α)

        // correction for σm (corrected angular separation)
        cos2_sigma_m = cos_sigma - 2.0 * sin_u1 * sin_u2 / cos2_alpha; // cos(σm)
        // correction factor
        let correction = FLATTENING / 16.0 * cos2_alpha * (4.0 + FLATTENING * (4.0 - 3.0 * cos2_alpha));

        // recalculate longitude difference (λ) for the next iter
        lambda_prev = lambda;
        lambda = delta_lon + (1.0 - correction) * FLATTENING * sin_alpha * (sigma + correction * sin_sigma * (cos2_sigma_m + correction * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2))));
    }

    if iter_limit == 0 {
        // did not converge :(
        return (0.0, 0.0, 0.0);
    }

    // compute distance
    let u2 = cos2_alpha * (SEMI_MAJOR_AXIS.powi(2) - SEMI_MINOR_AXIS.powi(2)) / SEMI_MINOR_AXIS.powi(2); // (cos(α))^2 * ((a^2)-(b^2))/(b^2)
    let a_coeff = 1.0 + (u2 / 16384.0) * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2))); // 
    let b_coeff = (u2 / 1024.0) * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));

    // correction term in distance formula (Δσ)
    let delta_sigma = b_coeff * sin_sigma * 
        (cos2_sigma_m + (b_coeff / 4.0) * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m.powi(2)) - (b_coeff / 6.0)  * cos2_sigma_m * (-3.0 + 4.0 * sin_sigma.powi(2)) * (-3.0 + 4.0 * cos2_sigma_m.powi(2))));

    let distance = SEMI_MINOR_AXIS * a_coeff * (sigma - delta_sigma); // s

    // azimuths α1 and α2
    let mut alpha1 = (cos_u2 * lambda.sin()).atan2((cos_u1 * sin_u2) - (sin_u1 * cos_u2 * lambda.cos())).to_degrees();
    let mut alpha2 = (cos_u1 * lambda.sin()).atan2((-1.0 * sin_u1 * cos_u2) + (cos_u1 * sin_u2 * lambda.cos())).to_degrees();

    // normalize azimuths to [0, 360]
    if alpha1 < 0.0 { alpha1 += 360.0 }
    if alpha2 < 0.0 { alpha2 += 360.0 }

    // normalize azimuth 2 to *reverse azimuth*, [0, 360)
    alpha2 = (alpha2 + 180.0) % 360.0;

    (distance, alpha1, alpha2)
}
