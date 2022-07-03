use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Result};

use crate::{util::weighted_median, WaypointExt};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Units {
    KmPerH,
    MiPerH,
    MPerS,
    FtPerS,
    MinPerMi,
    MinPerKm,
}

impl FromStr for Units {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "km/h" => Ok(Self::KmPerH),
            "mi/h" => Ok(Self::MiPerH),
            "m/s" => Ok(Self::MPerS),
            "ft/s" => Ok(Self::FtPerS),
            "min/mi" => Ok(Self::MinPerMi),
            "min/km" => Ok(Self::MinPerKm),
            _ => bail!("must be one of: km/h, mi/h, m/s, ft/s, min/mi, min/km"),
        }
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KmPerH => write!(f, "km/h"),
            Self::MiPerH => write!(f, "mi/h"),
            Self::MPerS => write!(f, "m/s"),
            Self::FtPerS => write!(f, "ft/s"),
            Self::MinPerMi => write!(f, "min/mi"),
            Self::MinPerKm => write!(f, "min/km"),
        }
    }
}

pub fn convert(speed: f64, units: Units) -> f64 {
    match units {
        Units::KmPerH => speed * 3.6,
        Units::MiPerH => speed * 2.237,
        Units::MPerS => speed * 1.0,
        Units::FtPerS => speed * 3.281,
        Units::MinPerMi => 26.82 / speed,
        Units::MinPerKm => 16.67 / speed,
    }
}

pub fn pretty(speed: f64, units: Units) -> String {
    format!("{:.2} {units}", convert(speed, units))
}

pub fn median(track: &[WaypointExt]) -> Result<f64> {
    let weighted_speeds: Vec<(f64, f64)> = track
        .iter()
        .filter_map(|p| {
            p.speed
                .and_then(|s| p.inter_time.map(|t| (s, t.as_seconds_f64())))
        })
        .collect();
    weighted_median(&weighted_speeds)
}
