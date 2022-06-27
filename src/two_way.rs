use std::collections::HashMap;

use geo::Point;
use gpx::{Track, Waypoint};

use crate::WaypointExt;

// distance in metres that we are searching for pairs in
pub const MAX_DIST: f64 = 20.0;

pub const MAX_DEG_DIST: f64 = 0.0001796638489386358;

pub const MIN_TIME: i32 = 120;

pub fn walking_pairs(track: &Vec<WaypointExt>) -> Option<usize> {
    let mut lcursor = 0;
    let mut rcursor = track.len() - 1;
    let mut unpaired = 0;


    todo!()
}
