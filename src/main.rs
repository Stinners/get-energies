
mod results;
mod get_filetype;

// Modules containing the functionality for each program 
mod pychem;
mod gamess;
mod qchem;


// external crate imports 
use clap::{Arg, App, ArgMatches};

// internal module imports
use crate::{
    get_filetype::{get_filetype, Reader, Program},
    results::{FileResults, print_energies, get_all_methods},
};


/*  =================================================
 *       
 *                       Main  
 *
 *  ================================================= */

fn main() {
    let args = App::new("Energies")
        .version("0.1")
        .about("Extracting energies from computational chemistry output files")
        .arg(Arg::with_name("Method")
             .short("m")
             .long("method")
             .help("Specify the method to read")
             .takes_value(true)
             .required(false))
        .arg(Arg::with_name("File")
             .help("File to extract energies from")
             .takes_value(true)
             .multiple(true)
             .value_delimiter(" ")
             .required(true))
        .arg(Arg::with_name("Reverse")
             .help("Reverse the order energies are printed in")
             .short("r")
             .takes_value(false)
             .multiple(false))
        .arg(Arg::with_name("Don't Print Missing")
             .help("Print a blank line for a missing energy value rather than an error message")
             .short("p")
             .takes_value(false))
        .get_matches();

    // extract the values 
    let filenames = args.values_of("File").unwrap();

    let mut results_list = vec![];

    for file in filenames {
        let mut results = FileResults::new(file);
        let (filetype, lines) = get_filetype(file);

        if filetype != Program::Unknown {
            results.program = filetype;
            read_energies(&mut results, lines.unwrap());
            results_list.push(results);
        } else {
            println!("Could not read file: {}", file);
        }
    }

    if args.is_present("Reverse") {
        reverse_results(&mut results_list);
    }

    let methods = get_methods(&results_list, &args);
    let print_missing = !args.is_present("Don't Print Missing");
    print_energies(&results_list, &methods, print_missing); 

}

/*  =================================================
 *       
 *                  Helper Functions 
 *
 *  ================================================= */

// Either takes the provided method and uppercases it or get all list of 
// all methods found in the input 
fn get_methods(results: &Vec<FileResults>, args: &ArgMatches) -> Vec<String> {
    match args.value_of("Method") {
        Some(method) => vec![method.to_uppercase()],
        None => get_all_methods(&results),
    }
}


// This function is just for dispatching to the various readers
fn read_energies(results: &mut FileResults, lines: Reader) {
    match results.program {
        Program::PyChem => pychem::read(results, lines),
        Program::QChem => qchem::read(results, lines),
        Program::Gamess => gamess::read(results, lines),
        Program::Unknown => unreachable!(),
    };
}

// If theres just one file in the results we reverse all the calculations 
// in it, if there are multiple files we reverse the order of the files
fn reverse_results(results_list: &mut Vec<FileResults>) {
    if results_list.len() == 1 {
        results_list[0].calculations.reverse();
    } else if results_list.len() > 1 {
        results_list.reverse()
    }
}

