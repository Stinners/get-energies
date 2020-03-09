
use std::collections::{HashMap, HashSet};

use crate::get_filetype::Program;

// Energies is a hashmap of method names to energy values
pub type Energies = HashMap<String, Vec<String>>;

// The results from a single file
// Each method in a calculation corresponds to a 
// single line of output
#[derive(Debug)]
pub struct FileResults {
    pub program: Program,
    pub filename: String,
    pub calculations: Vec<Energies>,
}

impl FileResults {
    pub fn new(filename: &str) -> FileResults {
        FileResults {
            filename: filename.to_string(),
            program: Program::Unknown,
            calculations: vec![HashMap::new()],
        }
    }

    pub fn add_energy<M>(&mut self, method: M, energy: &str) 
		where M: Into<String>,
	{
		   
        let method = method.into();
        let latest_calc = self.calculations.last_mut().unwrap();
        latest_calc.entry(method)
            .and_modify(|energies| energies.push(energy.to_string()))
            .or_insert(vec![energy.to_string()]);
    }

    pub fn start_new_calc(&mut self) {
        self.calculations.push(HashMap::new());
    }

}

// Convert a vector of energies to the appriate csv representation 
fn build_line(energies: &Vec<String>, n: usize) -> String {
	energies.iter()
        .fold(format!("{:>3}, ", n), |string, energy| string + &format!("{}, ", energy))
}

// Construct the text to print for a single method by iterating over all the 
// results and calculations 
fn make_method_lines(results: &Vec<FileResults>, method: &str, print_errors: bool) -> String {
	let mut method_string = String::new();
	// Loop over all the result files to find all energies of a particular method
	for (file_num, result) in results.iter().enumerate() {
        // Loop over each calculation in the file
		for (calc_num, energies) in result.calculations.iter().enumerate() {
			if energies.len() == 0 { continue; }
			let line = match energies.get(method) {
				Some(values) => build_line(values, file_num+calc_num), 
				None => {
					if print_errors {
						format!("No {} energies found for {} calculation {}",
								method, result.filename, calc_num)
					} else {
						String::new()
					}
				},
			};
			method_string.push_str(&line);
			method_string.push('\n');
		}
	}
	method_string
}


pub fn print_energies(results: &Vec<FileResults>, methods: &Vec<String>, print_errors: bool) {

	for method in methods.iter() {
		let method_text = make_method_lines(results, method, print_errors);
		if method_text.len() > 0 {
			println!("===> {},", method);
			println!("{}", method_text);
		}
	}

}

pub fn get_all_methods(results: &Vec<FileResults>) -> Vec<String> {
	let mut methods: HashSet<String> = HashSet::new();
	for result in results.iter() {
		for calc in result.calculations.iter() {
			for key in calc.keys() {
				methods.insert(key.to_string());
			}
		}
	}
	methods.into_iter().collect()
}



