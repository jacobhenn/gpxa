use std::{fmt::Display, path::PathBuf, str::FromStr};

use argh::FromArgs;

use anyhow::{bail, Result};

use atty::Stream::Stdout;

use crate::{
    dist,
    speed,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pretty {
    Always,
    Auto,
    Never,
}

impl FromStr for Pretty {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            _ => bail!("must be one of: always, auto, never"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct StatsMask {
    pub total_dist: bool,
    pub total_time: bool,
    pub avg_speed: bool,
    pub median_speed: bool,
    pub max_elev: bool,
    pub min_elev: bool,
}

impl StatsMask {
    pub const ALL: Self = Self {
        total_dist: true,
        total_time: true,
        avg_speed: true,
        median_speed: true,
        max_elev: true,
        min_elev: true,
    };

    pub const fn has_seg(&self, i: usize) -> bool {
        match i {
            0 => self.total_dist || self.total_time || self.avg_speed || self.median_speed,
            1 => self.max_elev || self.min_elev,
            _ => false,
        }
    }


    pub fn print_newln_0(&self) {
        if self.has_seg(0) && self.has_seg(1) {
            println!();
        }
    }
}

impl FromStr for StatsMask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Self::default();
        for v in s.split(',') {
            match v {
                "total-dist" => mask.total_dist = true,
                "total-time" => mask.total_time = true,
                "avg-speed" => mask.avg_speed = true,
                "median-speed" => mask.median_speed = true,
                "max-elev" => mask.max_elev = true,
                "min-elev" => mask.min_elev = true,
                s => {
                    if s.is_empty() {
                        bail!("empty comma-separated value");
                    }
                    bail!("unexpected value '{s}'. see --help for a list of valid values.");
                }
            }
        }

        Ok(mask)
    }
}

#[derive(FromArgs)]
/// A GPX (GPS Exchange) file analyzer.
pub struct Raw {
    /// a path to a gpx file
    #[argh(positional)]
    pub path: PathBuf,

    /// which track in the gpx file to analyze (0-based index). not necessary if there is only one track.
    #[argh(option, short = 't')]
    pub track: Option<usize>,

    /// when to print stat names and units. values: always, auto, never. auto detects if output is being piped. defaults to auto.
    #[argh(option, short = 'p')]
    pub pretty: Option<Pretty>,

    /// distance units to use. ft for feet and m for metres. defaults to metres.
    #[argh(option, short = 'd')]
    pub dist_units: Option<dist::Units>,

    /// speed units to use. values: km/h, mi/h, m/s, ft/s, min/km, min/mi. defaults to mi or km per hr depending on --dist-units.
    #[argh(option, short = 's')]
    pub speed_units: Option<speed::Units>,

    /// what to display in output. comma-separated list of values. values: total-dist, total-time, avg-speed, median-speed, max-elev, min-elev. order of values does not affect order of output. defaults to all of them.
    #[argh(option, short = 'o')]
    pub output: Option<StatsMask>,
}

pub struct Args {
    pub path: PathBuf,
    pub track: Option<usize>,
    pub pretty: bool,
    pub dist_units: dist::Units,
    pub speed_units: speed::Units,
    pub mask: StatsMask,
}

impl From<Raw> for Args {
    fn from(raw_args: Raw) -> Self {
        let pretty = match raw_args.pretty {
            Some(Pretty::Always) => true,
            Some(Pretty::Never) => false,
            Some(Pretty::Auto) | None => atty::is(Stdout),
        };

        let dist_units = raw_args.dist_units.unwrap_or(dist::Units::Metres);
        let speed_units = raw_args.speed_units.unwrap_or(match dist_units {
            dist::Units::Metres => speed::Units::KmPerH,
            dist::Units::Feet => speed::Units::MiPerH,
        });

        Self {
            path: raw_args.path,
            track: raw_args.track,
            pretty,
            dist_units,
            speed_units,
            mask: raw_args.output.unwrap_or(StatsMask::ALL),
        }
    }
}

impl Args {
    pub fn print_stat<T, F, G>(&self, name: &str, stat: &Option<T>, pretty: F, ugly: G)
    where
        T: Display,
        F: Fn(&T) -> String,
        G: Fn(&T) -> String,
    {
        match stat {
            Some(s) => {
                if self.pretty {
                    println!("{name}: {}", pretty(s));
                } else {
                    println!("{}", ugly(s));
                }
            }
            None => (),
        }
    }

    pub fn print_dist_stat(&self, name: &str, stat: &Option<f64>, units: dist::Units) {
        self.print_stat(
            name,
            stat,
            |d| dist::pretty(*d, units),
            |d| dist::convert(*d, units).to_string(),
        );
    }

    pub fn print_speed_stat(&self, name: &str, stat: &Option<f64>, units: speed::Units) {
        self.print_stat(
            name,
            stat,
            |d| speed::pretty(*d, units),
            |d| speed::convert(*d, units).to_string(),
        );
    }
}
