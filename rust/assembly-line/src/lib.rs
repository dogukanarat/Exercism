// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

fn success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1 | 2 | 3 | 4 => 1.0,
        5 | 6 | 7 | 8 => 0.9,
        9 | 10 => 0.77,
        _ => panic!("Invalid speed: {}", speed),
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * success_rate(speed) * 221.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0 ) as u32
}
