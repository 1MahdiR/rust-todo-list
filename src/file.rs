
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::Path;

const FILE_PATH: &str = "tasks.txt";

fn check_and_create_file(file_path: &str) {
	if !Path::new(file_path).exists() {
		File::create(FILE_PATH).expect("Failed to create file");
	}
}

pub fn read_tasks_from_file() -> Vec<String> {
	let mut result = Vec::new();

	check_and_create_file(FILE_PATH);
	
	for line in read_to_string(FILE_PATH).unwrap().lines() {
		result.push(line.to_string());
	}
	result
}

pub fn write_task_to_file(task: &str) {
	check_and_create_file(FILE_PATH);

	let mut file = OpenOptions::new()
		.append(true)
		.open(FILE_PATH)
		.expect("Failed to create file");
	
	writeln!(file, "{}", task).expect("Failed to write to file");
}

pub fn delete_a_task_from_file(task_index: i32) {
	let tasks = read_tasks_from_file();

	let mut file = File::create(FILE_PATH).expect("Failed to create file");

	for (i, j) in tasks.iter().enumerate() {
		if task_index - 1 != i as i32 {
			writeln!(file, "{}", j).expect("Failed to write to file");
		}
	}
}
