extern crate clap;
extern crate log;
extern crate simplelog;
use clap::{Arg, App};

mod parser;

use parser::vasp::poscar;

fn main() {
    let matches = App::new("$1")
        .version("0.1.0")
        .author("Alejandro Gallo <aamsgallo@gmail.com>")
        .about("$2")
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
