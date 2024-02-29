use std::fs;
use std::env;
use std::collections::HashMap;

struct LcountData {
	pub results: HashMap<String, u64>,
	pub ignore_list: HashMap<String, i32>,
	pub base_path: String,
	pub dot_mode: bool
}

impl LcountData {
	pub fn new() -> Self {
		LcountData {
			results: HashMap::new(),
			ignore_list: HashMap::new(),
			base_path: String::from(".\\"),
			dot_mode: true
		}
	}
}

//recursively get all files in a directory
fn get_filenames(data: &mut LcountData, dir_name: &str) {

	//don't check directory if it's in the ignore list
	if data.ignore_list.contains_key(dir_name) {
		return;
	}

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
			//ignore the file if it's in the ignore list
			if !data.ignore_list.contains_key(&file_name) {
				count_lines(data, file_name);
			}
		}
	}
}

//get the lines in a file
fn count_lines(data: &mut LcountData, file_name: String) {

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
	data.results.insert(file_name, file_lines);
}

fn print_helptext() {
	println!("[lcount]");
	println!("-h --help -help");
	println!("Show this help text");
	println!();

	println!("-t --target");
	println!("Target folder to check the lines of");
	println!();

	println!("-i --ignore");
	println!("Folder/file to be ignored, can be used multiple times");
	println!("can also be passed as a comma,separated,list");
	println!();
}

fn main() {
	//get command line arguments
	let args: Vec<String> = env::args().collect();

	let mut data = LcountData::new();

	//iterate through arguments
	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"-i" | "-ignore" => {
				
				//this has to be done for windows otherwise it may not recognize the path
				let mut argstr = args[i+1].to_string();
				argstr = argstr.replace("/", "\\");

				//extract values from comma,separated,list
				if argstr.contains(',') {
					for arg in argstr.split(',') {
						data.ignore_list.insert(arg.to_string(), 1);
					}

				//add value directly to ignore list
				} else {
					data.ignore_list.insert(argstr.to_string(), 1);
				}

				i += 1;
			},

			"-h" | "--help" | "-help" => {
				print_helptext();
				std::process::exit(0);
			},

			"-t" | "--target" => {
				data.base_path = args[i+1].to_string();
				data.dot_mode = false;
				i += 1;
			}

			arg => {
				eprintln!("Unrecognized argument: {}", arg);
			}
		}

		i += 1;
	}
	
	let path_copy = data.base_path.to_string();
	get_filenames(&mut data, &path_copy);

	//count the total amount of lines
	let mut total_lines: u64 = 0;
	for i in &data.results {
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
	for i in &data.results {
		println!("{:max_digits$} {}", i.1, i.0);
	}

	println!("{:max_digits$} {}", total_lines, "total");
}