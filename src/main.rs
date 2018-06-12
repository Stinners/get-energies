
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod results;
mod get_filetype;
mod pychem;
mod qchem;


// stdlin imports 

// external crate imports 
use clap::{Arg, App, ArgMatches};

// internal module imports
use get_filetype::{get_filetype, Reader, Program};
use results::{FileResults, print_energies, get_all_methods};


/*  =================================================
 *       
 *                       Main  
 *
   ================================================= */

fn main() {
    let args = App::new("Energies")
        .version("0.1")
        .about("Geting energies from computational chemistry output files")
        .arg(Arg::with_name("Method")
             .short("m")
             .long("method")
             .help("Specify the method to read")
             .takes_value(true)
             .required(false))
        .arg(Arg::with_name("Get")
             //.value_name("GET")
             .help("Get energies from a file or files")
             .takes_value(true)
             .multiple(true)
             .value_delimiter(" ")
             .required(true))
        .get_matches();

    // extract the values 
    let filenames = args.values_of("Get").unwrap();

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
    
    let methods = get_methods(&results_list, &args);
    print_energies(&results_list, &methods); 

}

/*  =================================================
 *       
 *                  Helper Functions 
 *
   ================================================= */

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
        Program::QChem => unimplemented!(),
        _ => println!("Not implimented")
    };
}

