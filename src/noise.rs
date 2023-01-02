use crate::math::Vec2d;

/// Function to linearly interpolate between a0 and a1
/// Weight w should be in the range [0.0, 1.0]
fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    /* // You may want clamping by inserting:
     * if (0.0 > w) return a0;
     * if (1.0 < w) return a1;
     */
    // (a1 - a0) * w + a0
    (a1 - a0) * (3.0 - w * 2.0) * w * w + a0 // cubic interpolation (smooth step)
    //(a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0 // even smoother
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Create pseudorandom direction vector
fn random_gradient(ix: i32, iy: i32) -> Vec2d {
    let mut s = DefaultHasher::new();
    ix.hash(&mut s);
    iy.hash(&mut s);
    let hash = s.finish();
    let hash_bytes: [u8; 8] = hash.to_be_bytes();
    let hash_x = u32::from_be_bytes(hash_bytes[..4].try_into().expect("wrong size")) as f32;
    let hash_y = u32::from_be_bytes(hash_bytes[4..].try_into().expect("wrong size")) as f32;
    Vec2d { x: hash_x.cos(), y: hash_y.sin() }
}


/// Computes the dot product of the distance and gradient vectors.
fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32{
    // Get gradient from integer coordinates
    let gradient: Vec2d = random_gradient(ix, iy);

    // Compute the distance vector
    let dx = x - ix as f32;
    let dy = y - iy as f32;

    // Compute the dot-product
    // a lots of zeros here
    dx*gradient.x + dy*gradient.y
}

// Compute Perlin noise at coordinates x, y
pub fn perlin(x: f32, y: f32) -> u32 {
    // Determine grid cell coordinates
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    // Determine interpolation weights
    // Could also use higher order polynomial/s-curve here
    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    // Interpolate between grid point gradients
    let mut n0 = dot_grid_gradient(x0, y0, x, y);
    let mut n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, sx);

    n0 = dot_grid_gradient(x0, y1, x, y);
    n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, sx);

    let mut value = interpolate(ix0, ix1, sy); // Between -1 and 1.
    value = (value * 0.5 + 0.5) * 5.0; // now from 0 to 5
    value.round() as u32
}