#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![allow(clippy::struct_excessive_bools)]

mod args;
mod dist;
mod elev;
mod speed;
// mod two_way;
mod analyze;
mod util;

#[cfg(test)]
mod tests;

use std::{fs::File, io::BufReader};

use analyze::Stats;
use anyhow::{bail, Context, Result};
use args::{Args, Raw};
use geo::{GeodesicDistance, Point};
use gpx::{Gpx, Track};
use time::{Duration, OffsetDateTime};

/// Waypoint extention: a waypoint with extra data to assist in track analysis.
pub struct WaypointExt {
    /// Latitude & longitude combined.
    pub point: Point<f64>,

    /// Approximate speed at which waypoint was taken (m/s).
    pub speed: Option<f64>,

    /// Elevation (m).
    pub elev: Option<f64>,

    /// Distance travelled along the track so far (m).
    pub dist: f64,

    /// Time at which waypoint was taken.
    pub time: Option<OffsetDateTime>,

    /// Time duration between this waypoint and the next.
    pub inter_time: Option<Duration>,
}

impl WaypointExt {
    #[must_use]
    pub fn prepare_track(track: &Track) -> Vec<Self> {
        let mut points = track.segments.iter().flat_map(|seg| seg.points.iter());

        let mut v = Vec::new();
        if let Some(mut point) = points.next() {
            let mut dist = 0.0;
            for next_point in points {
                let inter_dist = next_point.point().geodesic_distance(&point.point());
                let inter_time = next_point.time.and_then(|t| {
                    point
                        .time
                        .map(|u| OffsetDateTime::from(t) - OffsetDateTime::from(u))
                });
                let speed = inter_time.map(|t| inter_dist / t.as_seconds_f64());
                v.push(Self {
                    point: point.point(),
                    speed,
                    elev: point.elevation,
                    dist,
                    time: point.time.map(OffsetDateTime::from),
                    inter_time,
                });
                dist += inter_dist;
                point = next_point;
            }
        }

        v
    }
}

fn main() -> Result<()> {
    let raw_args: Raw = argh::from_env();
    let args = Args::from(raw_args);

    if args.path.is_dir() {
        bail!("{:?} is a directory.", args.path);
    }

    let file = File::open(args.path.clone()).context("couldn't open provided file")?;
    let reader = BufReader::new(file);
    let gpx: Gpx = gpx::read(reader).with_context(|| format!("{:?} has invalid gpx", args.path))?;

    if gpx.tracks.is_empty() {
        bail!("file contains no tracks");
    } else if gpx.tracks.len() > 1 && args.track.is_none() {
        bail!(
            "{:?} has more than one track. use --track <0..{}> to specify.",
            args.path,
            gpx.tracks.len() - 1
        );
    }

    let i = args.track.unwrap_or_default();
    if let Some(track) = gpx.tracks.get(i) {
        let points = WaypointExt::prepare_track(track);
        let stats = Stats::from_track(&points, &args)?;
        stats.print(&args);
    } else {
        bail!(
            "{:?} has {} track{}, but index was {} (indices start at 0).",
            args.path,
            gpx.tracks.len(),
            if gpx.tracks.len() == 1 { "" } else { "s" },
            i,
        )
    }

    Ok(())
}
