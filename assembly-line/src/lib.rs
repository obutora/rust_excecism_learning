// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as f64;

    if speed <= 4.0 {
        speed * 221.0
    } else if speed <= 8.0 {
        speed * 221.0 * 0.9
    } else {
        speed * 221.0 * 0.77
    }
    // unimplemented!("calculate hourly production rate at speed: {}", speed)

}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    let per_hour = production_rate_per_hour(speed);
    let per_minute = per_hour / 60.0;
    per_minute as u32
}
