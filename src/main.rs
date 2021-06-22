#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

mod err;
mod cli;
mod gui;
mod dmx;
mod node;
mod localization;
mod conmx_core;

use log::{Level, error};

use std::io::Write;
use chrono::Local;
use env_logger::{
    fmt::Color,
    Env,
};
use localization::localized;

use log::LevelFilter;


fn main() {
    dotenv::dotenv().ok();

    // Setup logging
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .format(|buf, rec| {
            let lvl = rec.level();
            let mut lvl_style = buf.style();

            match lvl {
                Level::Trace => lvl_style.set_color(Color::White),
                Level::Debug => lvl_style.set_color(Color::Blue),
                Level::Info => lvl_style.set_color(Color::Green),
                Level::Warn => lvl_style.set_color(Color::Yellow),
                Level::Error => lvl_style.set_color(Color::Red),
            };
            
            writeln!(buf,
                "{}: [{:>6}]: [{:>10}] \t {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                lvl_style.value(rec.level()),
                rec.target(),
                rec.args(),
            )
        })
        //.filter_module("conmx", LevelFilter::Info)
        .init();

    println!("{}", localized("This is a test!"));
    println!("{}", localized("main:test-2"));

    let conf = conmx_core::Config {};

    match cli::CliOpts::parse() {
        Ok(cliopts) => {
            match cliopts.validate() {
                cli::CliOpts::Unvalidated(_) => error!("Options given are not composited correctly!"),
                cli::CliOpts::Validated(opts_val) => match gui::run(opts_val, conf) {
                    Ok(()) => (),
                    Err(e) => error!("{}", e),
                }
            }
        }
        Err(e) => error!("Error {} occured while using the app!", e)
    }
}
