use std::fs;
use std::env;

//recursively get all files in a directory
fn get_filenames(lines: &mut Vec<u64>, names: &mut Vec<String>, dir_name: &str) {
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
		if descriptor.is_dir() {
			get_filenames(lines, names, &file_name);
		} else {
			split_file(lines, names, &file_name);
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
fn split_file(lines: &mut Vec<u64>, names: &mut Vec<String>, file_name: &str) {
	let file_lines = count_lines(&file_name);
		
	//keep track of data for printing later
	lines.push(file_lines);
	names.push(file_name.to_string());
}

fn main() {
	//get command line arguments
	let args: Vec<String> = env::args().collect();

	//assign path if a path is provided
	let mut path: String = ".".to_string(); 
	if args.len() > 1 {
		path = args[1].clone();
	}
	
	let mut lines: Vec<u64> = Vec::new();
	let mut names: Vec<String> = Vec::new();
	get_filenames(&mut lines, &mut names, &path);
	
	//count the total amount of lines
	let mut total_lines: u64 = 0;
	for i in &lines {
		total_lines += *i;
	}
	
	//find the maximum number of digits
	let mut max_digits: usize = 0;
	let mut max_number = total_lines;
	while max_number > 0 {
		max_number = max_number / 10;
		max_digits += 1;
	}
	
	//print collumnated output
	for i in 0..names.len() {
		println!("{:max_digits$} {}", lines[i], names[i]);
	}

	println!("{:max_digits$} {}", total_lines, "total");
}