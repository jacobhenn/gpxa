use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Result};

use crate::{WaypointExt, util::weighted_median};

pub enum SpeedUnits {
    KmPerH,
    MiPerH,
    MPerS,
    FtPerS,
    MinPerMi,
    MinPerKm,
}

impl FromStr for SpeedUnits {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "km/h" => Ok(Self::KmPerH),
            "mi/h" => Ok(Self::MiPerH),
            "m/s" => Ok(Self::MPerS),
            "ft/s" => Ok(Self::FtPerS),
            "min/mi" => Ok(Self::MinPerMi),
            "min/km" => Ok(Self::MinPerKm),
            s => bail!("invalid speed unit {s}"),
        }
    }
}

impl Display for SpeedUnits {
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

pub fn convert(speed: f64, units: &SpeedUnits) -> f64 {
    match units {
        SpeedUnits::KmPerH => speed * 3.6,
        SpeedUnits::MiPerH => speed * 2.237,
        SpeedUnits::MPerS => speed * 1.0,
        SpeedUnits::FtPerS => speed * 3.281,
        SpeedUnits::MinPerMi => 26.82 / speed,
        SpeedUnits::MinPerKm => 16.67 / speed,
    }
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
