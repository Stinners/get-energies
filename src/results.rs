
use std::collections::{HashMap, HashSet};

use get_filetype::Program;

// Energies is a hashmap of method names to energy values
pub type Energies = HashMap<String, Vec<String>>;

// The results from a single file
// Each method in a calculation corresponds to a 
// single line of output
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

    pub fn add_energy(&mut self, method: &str, energy: &str) {
        let method = method.to_string();
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
fn build_line(energies: &Vec<String>) -> String {
	energies.iter().fold(String::new(), |string, energy| string + &format!("{}, ", energy))
}

// Construct the text to print for a single method by iterative over all the 
// results and calculations 
fn make_method_lines(results: &Vec<FileResults>, method: &str) -> String {
	let mut method_string = String::new();
	for result in results.iter() {
		for (i, energies) in result.calculations.iter().enumerate() {
			if energies.len() == 0 { continue; }
			let line = match energies.get(method) {
				Some(values) => build_line(values), 
				None => format!("No {} energies found for {} calculation {}",
								method, result.filename, i),
			};
			method_string.push_str(&line);
			method_string.push('\n');
		}
	}
	method_string
}


pub fn print_energies(results: &Vec<FileResults>, methods: &Vec<String>) {

	for method in methods.iter() {
		let method_text = make_method_lines(results, method);
		if method_text.len() > 0 {
			println!("===> {}", method);
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



