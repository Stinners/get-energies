
use regex::Regex;

use results::FileResults;
use get_filetype::Reader;

lazy_static! {
    static ref NEW_CALC: Regex = Regex::new("^User input: [0-9]+ of [0-9]+").unwrap();
    static ref HF_RE: Regex = Regex::new(r"^ SCF   energy in the final basis set = (-?[0-9]*\.[0-9]+)").unwrap();
    static ref SFCI_RE: Regex = Regex::new(r"RAS-CI total energy for state +[0-9]+: +(-?[0-9]*\.[0-9]+)").unwrap();
    static ref CCSDT_RE: Regex = Regex::new(r"^ +CCSD\(T\) total energy += +(-?[0-9]*\.[0-9]+)").unwrap();
    static ref MP2_RE: Regex = Regex::new(r"^ +MP2 energy + = +(-?[0-9]*\.[0-9]+)").unwrap();
}


pub fn check(line: &str) -> bool {
    lazy_static! { static ref CHECK_RE: Regex = Regex::new(r" +Welcome to Q-Chem").unwrap(); }
    CHECK_RE.is_match(line)
}

pub fn read(results: &mut FileResults, lines: Reader) {
    
    for line in lines {
        let line = line.unwrap();

        // Start a new row of the FileResults if we find the start
        // of a new calculation 
        if NEW_CALC.is_match(&line) {
            results.start_new_calc();
        }

        // Get the energies that are of the basic form 
        // match a line and take the first capture
        else if capture_energy(&line, &HF_RE, "HF", results) {}
        else if capture_energy(&line, &SFCI_RE, "SFCI", results) {}
        else if capture_energy(&line, &CCSDT_RE, "CCSD(T)", results) {}
        else if capture_energy(&line, &MP2_RE, "MP2", results) {}
    }
}

fn capture_energy(line: &str, regex: &Regex, method: &str, results: &mut FileResults) -> bool {
    if let Some(energy_match) = regex.captures(line) {
        results.add_energy(method, &energy_match[1]);
        true 
    } else {
        false 
    }
}


