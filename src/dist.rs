use std::{fmt::Display, str::FromStr};

use anyhow::bail;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DistUnits {
    Metres,
    Feet,
}

impl DistUnits {
    fn per_m(self: DistUnits) -> f64 {
        match self {
            Self::Metres => 1.0,
            Self::Feet => 3.281,
        }
    }
}

impl Default for DistUnits {
    fn default() -> Self {
        Self::Metres
    }
}

impl Display for DistUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metres => write!(f, "m"),
            Self::Feet => write!(f, "ft"),
        }
    }
}

impl FromStr for DistUnits {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(Self::Metres),
            "ft" => Ok(Self::Feet),
            s => bail!("invalid distance unit '{}'", s),
        }
    }
}

pub fn convert(dist: f64, units: &DistUnits) -> f64 {
    dist * units.per_m()
}

pub fn pretty(dist: f64, units: &DistUnits) -> String {
    let udist = convert(dist, units);
    if units == &DistUnits::Feet && udist >= 5280.0 {
        format!("{:.2} mi", udist / 5280.0)
    } else {
        format!("{:.2} {units}", udist)
    }
}
