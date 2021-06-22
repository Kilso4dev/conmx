mod components;
mod views;
mod conmx;
pub mod style;

use iced::{
    Application,
    Settings,
};

use crate::conmx_core;
use crate::cli::CliOptValues;
use crate::err;
use conmx::ConMX;

pub fn run(_cliopts: CliOptValues, conf: conmx_core::Config) -> Result<(),err::ConmxErr> {
    let mut settings = Settings::with_flags(conf);
    settings.antialiasing = true;
    ConMX::run(settings)
        .map_err(|e| err::ConmxErr::Net(format!("{}", e)))
}
