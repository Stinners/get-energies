
use regex::Regex;

use results::FileResults;
use get_filetype::Reader;

lazy_static! {
    static ref CASSCF_RE: Regex = Regex::new(r"^ STATE   [0-9]+  ENERGY= +(-?[0-9]*\.[0-9]+)").unwrap();
    static ref END_RE: Regex = Regex::new(r"^ +\.+ END OF PROPERTY EVALUATION").unwrap();
    static ref START_READ: Regex = Regex::new(r" -MCCI- BASED ON OPTIMIZED ORBITALS").unwrap();
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

        // Use this to account for the fact that the first set of results
        // in the file is a CI with unoptimized orbitals
        else if !reading && START_READ.is_match(&line) {
            reading = true;
        }

        else if END_RE.is_match(&line) {
            results.start_new_calc();
        }
    }

}
