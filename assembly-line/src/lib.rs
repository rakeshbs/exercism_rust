// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

fn cars_per_hour(speed: u8) -> u32 {
    221 * speed as u32
}

fn rate_of_success(speed: u8) -> f64 {
    if speed >= 1 && speed <= 4 {
        return 100.0;
    } else if speed >= 5 && speed <= 8 {
        return 90.0;
    } else {
        return 77.0;
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total_cars_possible = cars_per_hour(speed) as f64;
    println!("tot {}", total_cars_possible);
    rate_of_success(speed) * total_cars_possible / 100.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
