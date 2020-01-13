
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Lines};

use crate::{
    gamess,
    pychem,
    qchem,
};

pub type Reader = Lines<BufReader<File>>;

#[derive(PartialEq, Eq, Debug)]
pub enum Program {
    PyChem,
    QChem,
    Gamess,
    Unknown,
}

pub fn get_filetype(path: &str) -> (Program, Option<Reader>) {

    match get_iterator(path) {

        // Return early if the file can't be read 
        Err(_) => (Program::Unknown, None),

        // Otherwise look at the first 50 lines maximum
        Ok(mut lines) => {
            let mut filetype = Program::Unknown;

            for line in lines.by_ref().take(50) {
                let line = line.unwrap();
                if pychem::check(&line) { filetype = Program::PyChem } 
                else if qchem::check(&line) { filetype = Program::QChem }
                else if gamess::check(&line) { filetype = Program::Gamess }

                // When we find the filetype we want to return it along
                // with the remaining lines
                if filetype != Program::Unknown {
                    break;
                }
            }
            (filetype, Some(lines))
        }
    }
}

fn get_iterator(name: &str) -> Result<Reader> {
    let file = File::open(name)?;
    Ok(BufReader::new(file).lines())
}

