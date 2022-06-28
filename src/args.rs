use std::{fmt::Display, path::PathBuf, str::FromStr};

use argh::FromArgs;

use anyhow::{bail, Result};

use atty::Stream::Stdout;

use crate::{
    dist::{self, DistUnits},
    speed::{self, SpeedUnits},
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

#[derive(FromArgs)]
/// A GPX (GPS Exchange) file analyzer.
pub struct RawArgs {
    /// a path to a gpx file
    #[argh(positional)]
    pub path: PathBuf,

    /// which track in the gpx file to analyze (0-based). not necessary if there is only one track.
    #[argh(option, short = 't')]
    pub track: Option<usize>,

    /// when to print stat names and units. values: always, auto, never. auto detects if output is being piped. defaults to auto.
    #[argh(option, short = 'p')]
    pub pretty: Option<Pretty>,

    /// distance units to use. ft for feet and m for metres. defaults to metres.
    #[argh(option, short = 'u')]
    pub dist_units: Option<DistUnits>,

    /// speed units to use. values: km/h, mi/h, m/s, ft/s, min/km, min/mi. defaults to (mi|km)/hr depending on --dist-units.
    #[argh(option, short = 'v')]
    pub speed_units: Option<SpeedUnits>,
}

pub struct Args {
    pub path: PathBuf,
    pub track: usize,
    pub pretty: bool,
    pub dist_units: DistUnits,
    pub speed_units: SpeedUnits,
}

impl From<RawArgs> for Args {
    fn from(raw_args: RawArgs) -> Self {
        let pretty = match raw_args.pretty {
            Some(Pretty::Always) => true,
            Some(Pretty::Never) => false,
            Some(Pretty::Auto) | None => atty::is(Stdout),
        };

        let dist_units = raw_args.dist_units.unwrap_or(DistUnits::Metres);
        let speed_units = raw_args.speed_units.unwrap_or(match dist_units {
            DistUnits::Metres => SpeedUnits::KmPerH,
            DistUnits::Feet => SpeedUnits::MiPerH,
        });

        Self {
            path: raw_args.path,
            track: raw_args.track.unwrap_or_default(),
            pretty,
            dist_units,
            speed_units,
        }
    }
}

impl Args {
    pub fn print_stat<T, F, G>(&self, name: &str, stat: &Option<Result<T>>, pretty: F, ugly: G)
    where
        T: Display,
        F: Fn(&T) -> String,
        G: Fn(&T) -> String,
    {
        match stat {
            Some(Ok(s)) => {
                if self.pretty {
                    println!("{name}: {}", pretty(s));
                } else {
                    println!("{}", ugly(s));
                }
            }
            Some(Err(e)) => {
                if self.pretty {
                    eprintln!("error: {e:#}");
                } else {
                    panic!("uncaught Err while printing '{name}': {e}");
                }
            }
            None => (),
        }
    }

    pub fn print_dist_stat(&self, name: &str, stat: &Option<Result<f64>>, units: DistUnits) {
        self.print_stat(
            name,
            stat,
            |d| dist::pretty(d, &units),
            |d| dist::convert(*d, &units).to_string(),
        );
    }

    pub fn print_speed_stat(&self, name: &str, stat: &Option<Result<f64>>, units: SpeedUnits) {
        self.print_stat(
            name,
            stat,
            |d| speed::pretty(d, &units),
            |d| speed::convert(*d, &units).to_string(),
        );
    }
}
