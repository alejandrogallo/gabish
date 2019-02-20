extern crate log;
extern crate regex;

use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::str::FromStr;

pub struct ParseError {}

#[derive(Debug)]
pub struct Poscar {
    pub modeline: String,
    pub basis: Vec<Vec<f64>>,
    pub coordinates: Vec<Vec<f64>>,
    pub name: String,
    pub natoms: usize,
    pub atomic_species: Vec<String>,
    pub atomic_numbers: Vec<usize>,
    pub header_constant: f64,
}

impl Poscar {
    fn empty() -> Self {
        Poscar {
            modeline: "".to_string(),
            basis: vec![],
            coordinates: vec![],
            name: "".to_string(),
            natoms: 0,
            atomic_species: vec![],
            atomic_numbers: vec![],
            header_constant: 0.0,
        }
    }
}

fn parse_three_vector<T: FromStr>(line: &str) -> Result<Vec<T>, ParseError> {
    let re_3_vector = Regex::new(
        r"(?x)
        ^
        \s*
        (?P<x>[+-]?\d+[.]?\d*[eE]?[-+]?\d*)
        \s*
        (?P<y>[+-]?\d+[.]?\d*[eE]?[-+]?\d*)
        \s*
        (?P<z>[+-]?\d+[.]?\d*[eE]?[-+]?\d*)
        .*
        $
    ",
    )
    .unwrap();
    match re_3_vector.captures(line) {
        Some(m) => {
            Ok(vec![
                m.name("x").unwrap().as_str().parse::<T>().unwrap(),
                m.name("y").unwrap().as_str().parse::<T>().unwrap(),
                m.name("z").unwrap().as_str().parse::<T>().unwrap(),
            ])
        },
        None => {
            log::error!(
            "Line {} is not a vector, your input file is probably compromised",
            line
            );
            Err(ParseError {})
        },
    }
}

pub fn parse(filepath: &str) -> Result<Poscar, ParseError> {
    log::info!("Parsing {}", filepath);
    let mut file = fs::File::open(filepath).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let lines = text.lines();

    let re_empty_line = Regex::new(r"^ *$").unwrap();

    let mut poscar = Poscar::empty();
    let mut lineno: usize = 0;

    for line in lines {
        if re_empty_line.is_match(line) {
            log::warn!("Line {} in {} is empty, ignoring it", lineno, filepath);
            continue;
        }

        lineno += 1;

        match lineno {
            1 => poscar.name = line.to_string(),
            2 => poscar.header_constant = line.parse::<f64>().unwrap_or(0.0),
            3 | 4 | 5 => {
                let vec: Vec<f64> = parse_three_vector(line)?;
                poscar.basis.push(vec);
            },
            8 => poscar.modeline = line.to_string(),
            _ => continue,
        }

    }
    println!("{:?}", poscar);

    Ok(poscar)
}
