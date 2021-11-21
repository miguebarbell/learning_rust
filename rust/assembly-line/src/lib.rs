// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let result = production_rate_per_hour(speed) / 60 as f64;
    result as u32
}
