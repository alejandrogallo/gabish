#[macro_use]
extern crate clap;
extern crate log;
extern crate simplelog;
use clap::{Arg, AppSettings, App};

pub mod parser;

use parser::vasp::poscar;

fn main() {
    let matches = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author("Alejandro Gallo <aamsgallo@gmail.com>")
        .about(crate_description!())
        .arg(Arg::with_name("file")
             .takes_value(true)
             .short("f"))
        .get_matches();

    simplelog::CombinedLogger::init(
        vec![
            simplelog::TermLogger::new(
                log::LevelFilter::Info,
                simplelog::Config::default()
            ).unwrap()
        ]
    ).unwrap();

    let filename = matches.value_of("file").unwrap_or("default value");

    let _p = poscar::parse(filename).unwrap();

}
