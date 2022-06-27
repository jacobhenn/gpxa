use std::path::PathBuf;

use argh::FromArgs;

use crate::{dist::DistUnits, speed::SpeedUnits};

#[derive(FromArgs)]
/// A GPX (GPS Exchange) file analyzer.
pub struct Args {
    /// a path to a gpx file
    #[argh(positional)]
    pub path: PathBuf,

    /// which track in the gpx file to analyze. not necessary if there is only one track.
    #[argh(option, short = 't')]
    pub track: Option<usize>,

    /// distance units to use. ft for feet and m for metres. defaults to metres.
    #[argh(option, short = 'u')]
    pub dist_units: Option<DistUnits>,

    /// speed units to use. values: km/h, mi/h, m/s, ft/s, min/km, min/mi. defaults to (mi|km)/hr depending on --dist-units.
    #[argh(option, short = 'v')]
    pub speed_units: Option<SpeedUnits>,
}
