
use std::fs::read_to_string;

pub fn read_tasks_from_file() -> Vec<String> {

	let mut result = Vec::new();
	
	for line in read_to_string("tasks.txt").unwrap().lines() {
		result.push(line.to_string())
	}
	result
}
