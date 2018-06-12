
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Lines};

use pychem;

pub type Reader = Lines<BufReader<File>>;

#[derive(PartialEq, Eq)]
pub enum Program {
    PyChem,
    QChem,
    //Gamess,
    Unknown,
}


pub fn get_filetype(path: &str) -> (Program, Option<Reader>) {

    let lines = get_iterator(path);

    // Return earli if the file can't be read
    if lines.is_err() {
        println!("Can't open {}", path);
        (Program::Unknown, None)

    } else { 
        // Look at the first fifty lines max
        let mut lines = lines.unwrap();
        let mut filetype = Program::Unknown;
        

        for line in lines.by_ref().take(50) {
            let line = line.unwrap();
            if pychem::check(&line) { filetype = Program::PyChem } 
            else if qchem::check() { filetype = Program::QChem }

            // When we find the filetype we want to return it along
            // with the remaining lines
            if filetype != Program::Unknown {
                break;
            }
        }
        (filetype, Some(lines))
    }
}

fn get_iterator(name: &str) -> Result<Reader> {
    let file = File::open(name)?;
    Ok(BufReader::new(file).lines())
}

