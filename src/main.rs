use std::fs;
use std::env;
use std::collections::HashMap;

//recursively get all files in a directory
fn get_filenames(data: &mut HashMap<String, u64>, dir_name: &str) {
	let files = fs::read_dir(dir_name);

	//if the directory doesn't exist or is inaccessable, print an error and quit
	if files.is_err() {
		eprintln!("Cannot open directory: {}", dir_name);
		std::process::exit(-1);
	}

	//since we know there was no error, we can unwrap this without issue
	let files = files.unwrap();

	for file in files {
		let file_name = file.as_ref().unwrap().path().display().to_string();

		//skip line counting if item is a directory
		let descriptor = fs::metadata(&file_name).unwrap();

		//recursively call function to walk down the directory
		if descriptor.is_dir() {
			get_filenames(data, &file_name);

		//count the newlines in the file
		} else {
			split_file(data, file_name);
		}
	}
}

//count all the newlines in a file
fn count_lines(file_path: &str) -> u64 {
	let file_contents = match fs::read_to_string(file_path) {
		Ok(file) => file,
		Err(_error) => return 0,
	};
			
	let mut line_counter: u64 = 0;
	for c in file_contents.chars() {
		if c == '\n' {
			line_counter += 1;
		}
	}

	return line_counter;
}

//get the lines in a file
fn split_file(data: &mut HashMap<String, u64>, file_name: String) {
	let file_lines = count_lines(&file_name);
	
	//keep track of our data for printing later
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