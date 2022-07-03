use crate::{args::Args, dist, elev, speed, WaypointExt};

use anyhow::{bail, Context, Result};
use time::{Duration, OffsetDateTime};

pub struct Stats {
    total_dist: Option<f64>,
    total_time: Option<Duration>,
    avg_speed: Option<f64>,
    median_speed: Option<f64>,
    max_elev: Option<f64>,
    min_elev: Option<f64>,
}

impl Stats {
    pub fn from_track(track: &[WaypointExt], args: &Args) -> Result<Self> {
        let mut total_dist = None;
        let mut total_time = None;
        let mut avg_speed = None;
        let mut median_speed = None;
        let mut max_elev = None;
        let mut min_elev = None;

        if args.mask.total_dist {
            total_dist = Some(dist::total(track).context("couldnt get total distance")?);
        }

        if args.mask.total_time {
            let time = time_total(track).context("couldnt get total time")?;
            if let Some(total_dist) = total_dist {
                if time.is_zero() {
                    bail!("no time elapsed");
                }
                avg_speed = Some(total_dist / time.as_seconds_f64());
            }
            total_time = Some(time);
        }

        if args.mask.median_speed {
            median_speed = Some(speed::median(track).context("couldnt get median speed")?);
        }

        if args.mask.max_elev {
            max_elev = Some(elev::max(track).context("couldnt get max elevation")?);
        }

        if args.mask.min_elev {
            min_elev = Some(elev::min(track).context("couldnt get min elevation")?);
        }

        Ok(Self {
            total_dist,
            total_time,
            avg_speed,
            median_speed,
            max_elev,
            min_elev,
        })
    }

    pub fn print(&self, args: &Args) {
        args.print_dist_stat("total distance", &self.total_dist, args.dist_units);
        args.print_stat("total time", &self.total_time, Duration::to_string, |t| {
            t.whole_milliseconds().to_string()
        });
        args.print_speed_stat("average speed", &self.avg_speed, args.speed_units);
        args.print_speed_stat("median speed", &self.median_speed, args.speed_units);
        args.mask.print_newln_0();
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
