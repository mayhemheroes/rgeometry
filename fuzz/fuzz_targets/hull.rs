#![no_main]
use libfuzzer_sys::fuzz_target;
use rgeometry::{algorithms::convex_hull::graham_scan::convex_hull, data::Point};

fuzz_target!(|input: Vec<(i32, i32)>| {
    let points: Vec<Point<i32>> = input.iter().map(|x| Point::new([x.0, x.1])).collect();
    let _ = convex_hull(points);
});
