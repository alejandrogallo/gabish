extern crate log;
extern crate regex;

use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
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

fn parse_vector<T: FromStr>(line: &str) -> Result<Vec<T>, ParseError> {
    let re_3_vector = Regex::new(r"[^\s]+").unwrap();
    let mut v: Vec<T> = vec![];
    for m in re_3_vector.find_iter(line) {
        match m.as_str().parse::<T>() {
            Ok(val) => v.push(val),
            Err(_) => {
                log::error!("Error casting elements from line {}", line);
                log::error!("More precisely: {}", m.as_str());
                return Err(ParseError {});
            }
        };
    //match re_3_vector.find_iter(line) {
        //Some(m) => {
            //let mut v: Vec<T> = vec![];
            //for mm in m {
                //match mm.unwrap().as_str().parse::<T>() {
                    //Ok(val) => v.push(val),
                    //Err(_) => {
                        //log::error!("Error casting elements from line {}", line);
                        //log::error!("More precisely: {}", m.get(i).unwrap().as_str());
                        //return Err(ParseError {});
                    //}
                //};
            //}
            //Ok(v)
        //},
        //None => {
            //log::error!(
            //"Line {} is not a vector, your input file is probably compromised",
            //line
            //);
            //Err(ParseError {})
        //},
    }
    Ok(v)
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
                let vec: Vec<f64> = parse_vector(line)?;
                poscar.basis.push(vec);
            },
            6 => {
                let symbols: Vec<String> = parse_vector(line)?;
                poscar.atomic_species = symbols;
            },
            7 => {
                let symbols: Vec<usize> = parse_vector(line)?;
                poscar.atomic_numbers = symbols;
                poscar.natoms = poscar.atomic_numbers.iter().sum();
            },
            8 => poscar.modeline = line.to_string(),
            _ => {
                let vec: Vec<f64> = parse_vector(line)?;
                if poscar.coordinates.len() >= poscar.natoms {
                    log::warn!(
                        "All coordinates retrieved, stopping at line {}",
                        lineno
                    );
                    break;
                }
                poscar.coordinates.push(vec);
            },
        }
    }

    if poscar.coordinates.len() != poscar.natoms {
        log::error!("Number of coordinates and atoms is different!");
        return Err(ParseError {});
    }
    log::debug!("Natoms {}", poscar.natoms);
    log::debug!("Mode   {}", poscar.modeline);

    Ok(poscar)

}
