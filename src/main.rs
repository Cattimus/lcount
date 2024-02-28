use std::fs;
use std::env;
use std::collections::HashMap;

//recursively get all files in a directory
fn get_filenames(data: &mut HashMap<String, u64>, dir_name: &str) {

	//Check if the selected directory exists and get a list of filenames
	let files = fs::read_dir(dir_name);
	if files.is_err() {
		eprintln!("Cannot open directory: {}", dir_name);
		std::process::exit(-1);
	}
	let files = files.unwrap();

	//iterate over the files in the directory
	for file in files {

		//this should never happen but just in case it does
		if file.is_err() {
			continue;
		}

		//get file path as a string
		let file_name = file.as_ref().unwrap().path().display().to_string();

		//skip line counting if item is a directory
		let descriptor = fs::metadata(&file_name).unwrap();

		//recursively call function to walk down the directory
		if descriptor.is_dir() {
			get_filenames(data, &file_name);

		//count the newlines in the file
		} else {
			count_lines(data, file_name);
		}
	}
}

//get the lines in a file
fn count_lines(data: &mut HashMap<String, u64>, file_name: String) {

	//read file as a string
	let file_contents = fs::read_to_string(&file_name);
	if file_contents.is_err() {
		return;
	}
	let file_contents = file_contents.unwrap();

	//count how many lines are in the file
	let mut file_lines: u64 = 0;
	for c in file_contents.chars() {
		if c == '\n' {
			file_lines += 1;
		}
	}
	
	//store results for printing later
	data.insert(file_name, file_lines);
}

fn main() {
	//get command line arguments
	let args: Vec<String> = env::args().collect();

	//assign path if a path is provided
	let mut path: String = ".".to_string(); 
	if args.len() > 1 {
		path = args[1].clone();
	}
	
	let mut data:HashMap<String, u64> = HashMap::new();
	get_filenames(&mut data, &path);
	
	//count the total amount of lines
	let mut total_lines: u64 = 0;
	for i in &data {
		total_lines += i.1;
	}
	
	//find the maximum number of digits
	let mut max_digits: usize = 0;
	let mut max_number = total_lines;
	while max_number > 0 {
		max_number = max_number / 10;
		max_digits += 1;
	}
	
	//print collumnated output
	for i in &data {
		println!("{:max_digits$} {}", i.1, i.0);
	}

	println!("{:max_digits$} {}", total_lines, "total");
}