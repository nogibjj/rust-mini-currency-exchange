use chrono::{DateTime, Datelike};
use std::io;
use std::f64;

fn main() {
    println!("Enter the date in format YYYY-MM-DD:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let date = input.trim();
    let date = DateTime::parse_from_rfc3339(format!("{date}T00:00:00Z").as_str()).unwrap();
    let year = date.year() as f64;
    let month = date.month() as f64;
    let day = date.day() as f64;

    let jd = 367.0 * year - (7.0 * (year + ((month + 9.0) / 12.0).floor())) / 4.0 +
        (275.0 * month) / 9.0 + day - 730531.5;

    let m = (357.52911 + 0.98560028 * jd) % 360.0;
    let m = m.to_radians();
    let c = 1.914 * m.sin() + 0.0199 * 2.0 * m.sin();
    let l = (218.316 + 13.176396 * jd + c) % 360.0;

    let phase = (l / 360.0 + 0.5).fract();

    let moon_shape = match phase {
        x if x < 0.06 => "New Moon",
        x if x < 0.22 => "Waxing Crescent",
        x if x < 0.28 => "First Quarter",
        x if x < 0.44 => "Waxing Gibbous",
        x if x < 0.56 => "Full Moon",
        x if x < 0.72 => "Waning Gibbous",
        x if x < 0.78 => "Last Quarter",
        x if x < 0.94 => "Waning Crescent",
        _ => "New Moon",
    };

    println!("The moon shape for {date} is: {moon_shape}");
}
