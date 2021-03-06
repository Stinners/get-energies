
use regex::Regex;
use lazy_static::lazy_static;

use crate::{
    results::FileResults,
    get_filetype::Reader,
};

lazy_static! {
    static ref HF_RE: Regex = Regex::new(r"^ *Final HF energy: +(-?[0-9]*\.[0-9]*)").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"(-?[0-9]*\.[0-9]+)").unwrap();
    static ref NOCI_CHECK: Regex = Regex::new(r"^ *NOCI Energies").unwrap();
    static ref MP2_REGEX: Regex = Regex::new(r"Total MP2 energy: +(-?[0-9]*\.[0-9]*)").unwrap();
}


pub fn check(line: &str) -> bool {
    lazy_static! {  static ref CHECK_RE: Regex =  Regex::new(r"^pychem").unwrap(); }
    CHECK_RE.is_match(line)
}

pub fn read(results: &mut FileResults, lines: Reader) {

    let mut read_noci = false;
    
    for line in lines {
        let line = line.unwrap();
        
        // Look for the HF energies 
        let hf_match = HF_RE.captures(&line);
        if let Some(energy_match) = hf_match {
            results.add_energy("HF", &energy_match[1]);
        }

        let mp2_match = MP2_REGEX.captures(&line);
        if let Some(mp2_energy) = mp2_match {
            results.add_energy("MP2", &mp2_energy[1]);
        }

        // Now deal with NOCI
        // First set up to read when the NOCI Energies line is found 
        else if NOCI_CHECK.is_match(&line) {
            read_noci = true;
        } 
        // Then actually read all the energies
        // On the next line
        else if read_noci {
            for energy in NUMBER_REGEX.captures_iter(&line) {
                results.add_energy("NOCI", &energy[1]);
            }
            read_noci = false;
        }
    }
}
