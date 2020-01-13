
use regex::Regex;
use lazy_static::lazy_static;

use crate::{
    results::FileResults,
    get_filetype::Reader
};

lazy_static! {
    static ref CASSCF_RE: Regex = Regex::new(r"^ STATE   [0-9]+  ENERGY= +(-?[0-9]*\.[0-9]+)").unwrap();
    //static ref END_RE: Regex = Regex::new(r"^ +\.+ END OF PROPERTY EVALUATION").unwrap();
    static ref START_READ: Regex = Regex::new(r" -MCCI- BASED ON OPTIMIZED ORBITALS").unwrap();
    static ref GENERAL_RE: Regex = Regex::new(r"^ +FINAL ([A-Z]+) ENERGY IS +(-?[0-9]*\.[0-9]+)").unwrap();
    static ref CIS_RE: Regex = Regex::new(r"^ +EXCITED STATE +[0-9]+ +ENERGY= +(-?[0-9]*\.[0-9]+)").unwrap();
    static ref END_RE: Regex = Regex::new(r"^ -+ SURFACE MAPPING GEOMETRY").unwrap();
}


pub fn check(line: &str) -> bool {
    lazy_static! { static ref CHECK_RE: Regex = Regex::new(r"^ +\* +GAMESS VERSION").unwrap(); }
    CHECK_RE.is_match(line)
}

pub fn read(results: &mut FileResults, lines: Reader) {

    let mut reading = false;

    for line in lines {
        let line = line.unwrap();

        if let Some(captures) = CASSCF_RE.captures(&line) {
            if reading {
                results.add_energy("CASSCF", &captures[1]);
            }
        }

        else if let Some(captures) = CIS_RE.captures(&line) {
            results.add_energy("CIS", &captures[1]);
        }

        // General Regex 
        //  This catches at least different forms of HF
        else if let Some(captures) = GENERAL_RE.captures(&line) {
            results.add_energy(&captures[1], &captures[2]);
        }

        // Use this to account for the fact that the first set of results
        // in the file is a CI with unoptimized orbitals
        else if !reading && START_READ.is_match(&line) {
            reading = true;
        }

        // This finds the end of a single point calculation 
        // in a file containing a potential energy surface scan
        else if END_RE.is_match(&line) {
            results.start_new_calc();
        }
    }

}
