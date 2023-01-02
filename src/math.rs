use std::f32::consts::PI;

/// Regular 2d vector
pub struct Vec2d {
    pub x: f32,
    pub y: f32
}

/// Loi normale centré en 1;0
pub fn gauss(mu: f32, sigma: f32, x: f32) -> f32 {
    1_f32 / sigma / (2_f32 * PI).sqrt() * (-(x - mu).powf(2_f32)/2_f32/sigma.powf(2_f32)).exp()
}

/// Return true if a point is within a certain radius
pub fn is_in_circle(xr: u32, yr: u32, r: u32, x: u32, y: u32) -> bool {
    let delta_x = (x as i32 - xr as i32).pow(2) as f32;
    let delta_y = (y as i32 - yr as i32).pow(2) as f32;
    let distance = (delta_x + delta_y).sqrt();
    distance < r as f32
}