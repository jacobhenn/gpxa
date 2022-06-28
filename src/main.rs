#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]
#[warn(clippy::style)]
#[warn(clippy::suspicious)]
mod args;
mod dist;
mod elev;
mod speed;
// mod two_way;
mod util;

#[cfg(test)]
mod tests;

use std::{cmp::Ordering, fs::File, io::BufReader};

use anyhow::{bail, Context, Result};
use args::Args;
use dist::DistUnits;
use geo::{GeodesicDistance, Point};
use gpx::{Gpx, Track};
use speed::SpeedUnits;
use time::{Duration, OffsetDateTime};

use crate::util::pretty_duration;

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

fn analyze_track(args: &Args, track: &[WaypointExt]) -> Result<()> {
    if track.len() < 2 {
        bail!("invalid track: track contains less than two waypoints");
    }

    let dist_units = args.dist_units.unwrap_or_default();
    let speed_units = args.speed_units.as_ref().unwrap_or(match dist_units {
        DistUnits::Metres => &SpeedUnits::KmPerH,
        DistUnits::Feet => &SpeedUnits::MiPerH,
    });

    let dist = track.last().unwrap().dist;
    println!("total distance: {}", dist::pretty(dist, &dist_units));

    if let (Some(start), Some(end)) = (track[0].time, track.last().unwrap().time) {
        let total_time = end - start;
        println!("total time: {}", pretty_duration(total_time));

        let avg_speed = dist / total_time.as_seconds_f64();
        println!(
            "average speed: {:.2} {speed_units}",
            speed::convert(avg_speed, speed_units)
        );
    } else {
        println!("error: couldn't get start and end times");
    }

    match speed::median(track) {
        Ok(m) => println!(
            "median speed: {:.2} {speed_units}",
            speed::convert(m, speed_units)
        ),
        Err(e) => println!("error: couldnt get median speed: {e}"),
    }

    // println!();
    // speed::print_max(track, speed_units);

    println!();
    elev::print_max(track, &dist_units);
    elev::print_min(track, &dist_units);

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("error: {e:#}");
    }
}

fn go() -> Result<()> {
    let args: Args = argh::from_env();
    let file = File::open(args.path.clone()).context("couldn't open provided file")?;
    let reader = BufReader::new(file);
    let gpx: Gpx = gpx::read(reader).context("couldn't parse provided file as gpx")?;

    match gpx.tracks.len().cmp(&1) {
        Ordering::Greater => {
            if let Some(track) = args.track {
                let points = WaypointExt::prepare_track(&gpx.tracks[track]);
                analyze_track(&args, &points)?;
            } else {
                bail!(
                    "file contains multiple tracks. use --track <0..{}> to select one.",
                    gpx.tracks.len() - 1
                );
            }
        }
        Ordering::Equal => {
            let points = WaypointExt::prepare_track(&gpx.tracks[0]);
            analyze_track(&args, &points)?;
        }
        _ => bail!("file contains no tracks"),
    }

    Ok(())
}
