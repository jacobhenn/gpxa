use crate::WaypointExt;

/// get the maximum elevation of a track in integer metres.
pub fn max(track: &[WaypointExt]) -> Option<f64> {
    track.iter().filter_map(|p| p.elev).reduce(f64::max)
}

/// get the minimum elevation of a track in integer metres.
pub fn min(track: &[WaypointExt]) -> Option<f64> {
    track.iter().filter_map(|p| p.elev).reduce(f64::min)
}
