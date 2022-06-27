use crate::{
    dist::{self, DistUnits},
    WaypointExt,
};

/// get the maximum elevation of a track in integer metres.
pub fn max(track: &[WaypointExt]) -> Option<f64> {
    track.iter().filter_map(|p| p.elev).reduce(f64::max)
}

/// get the minimum elevation of a track in integer metres.
pub fn min(track: &[WaypointExt]) -> Option<f64> {
    track.iter().filter_map(|p| p.elev).reduce(f64::min)
}

pub fn print_max(track: &[WaypointExt], units: &DistUnits) {
    if let Some(max) = max(track) {
        println!("max elevation: {:.2} {units}", dist::convert(max, units));
    } else {
        println!("error: couldn't get max elevation");
    }
}

pub fn print_min(track: &[WaypointExt], units: &DistUnits) {
    if let Some(min) = min(track) {
        println!("min elevation: {:.2} {units}", dist::convert(min, units));
    } else {
        println!("error: couldn't get min elevation");
    }
}
