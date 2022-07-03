use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Context, Result};

use crate::WaypointExt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Units {
    Metres,
    Feet,
}

impl Units {
    const fn per_m(self) -> f64 {
        match self {
            Self::Metres => 1.0,
            Self::Feet => 3.281,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::Metres
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metres => write!(f, "m"),
            Self::Feet => write!(f, "ft"),
        }
    }
}

impl FromStr for Units {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(Self::Metres),
            "ft" => Ok(Self::Feet),
            _ => bail!("must be one of: m, ft"),
        }
    }
}

pub fn convert(dist: f64, units: Units) -> f64 {
    dist * units.per_m()
}

pub fn pretty(dist: f64, units: Units) -> String {
    let dist_converted = convert(dist, units);
    if units == Units::Feet && dist_converted >= 5280.0 {
        format!("{:.2} mi", dist_converted / 5280.0)
    } else {
        format!("{:.2} {units}", dist_converted)
    }
}

pub fn total(track: &[WaypointExt]) -> Result<f64> {
    track
        .last()
        .map(|p| p.dist)
        .context("couldnt get last waypoint")
}
