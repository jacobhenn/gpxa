use crate::{args::Args, dist, elev, speed, WaypointExt};

use anyhow::{Context, Result, bail};
use time::{Duration, OffsetDateTime};

pub struct Stats {
    total_dist: Option<Result<f64>>,
    total_time: Option<Result<Duration>>,
    avg_speed: Option<Result<f64>>,
    median_speed: Option<Result<f64>>,
    max_elev: Option<Result<f64>>,
    min_elev: Option<Result<f64>>,
}

impl Stats {
    pub fn from_track(track: &[WaypointExt], _args: &Args) -> Self {
        let total_dist = dist::total(track).context("couldnt get total distance");
        let total_time = time_total(track).context("couldnt get total time");
        let mean_speed = total_dist.and_then(|d| total_time.and_then(|t| {
            let secs = t.as_seconds_f64();
            if secs == 0.0 {
                bail!("zero elapsed time");
            } else {
                Ok(d / secs)
            }
        }));
        Self {
            total_dist: Some(dist::total(track).context("couldnt get total distance")),
            total_time: Some(time_total(track).context("couldnt get total time")),
            avg_speed: Some(mean_speed.context("couldn't get mean speed")),
            median_speed: Some(speed::median(track).context("couldnt get median speed")),
            max_elev: Some(elev::max(track).context("couldnt get max elevation")),
            min_elev: Some(elev::min(track).context("couldnt get min elevation")),
        }
    }

    pub fn print(&self, args: &Args) {
        args.print_dist_stat("total distance", &self.total_dist, args.dist_units);
        args.print_stat(
            "total time",
            &self.total_time,
            |t| t.to_string(),
            |t| t.whole_milliseconds().to_string(),
        );
        args.print_speed_stat("average speed", &self.avg_speed, args.speed_units);
        args.print_speed_stat("median speed", &self.median_speed, args.speed_units);

        if args.pretty {
            println!()
        }

        args.print_dist_stat("max elevation", &self.max_elev, args.dist_units);
        args.print_dist_stat("min elevation", &self.min_elev, args.dist_units);
    }
}

pub fn time_start(track: &[WaypointExt]) -> Result<OffsetDateTime> {
    track
        .first()
        .context("couldnt get first waypoint")
        .and_then(|p| p.time.context("first waypoint contains no time"))
}

pub fn time_end(track: &[WaypointExt]) -> Result<OffsetDateTime> {
    track
        .last()
        .context("couldnt get last waypoint")
        .and_then(|p| p.time.context("last waypoint contains no time"))
}

pub fn time_total(track: &[WaypointExt]) -> Result<Duration> {
    time_start(track)
        .context("couldnt get start time")
        .and_then(|t| {
            time_end(track)
                .context("couldnt get end time")
                .map(|u| u - t)
        })
}
