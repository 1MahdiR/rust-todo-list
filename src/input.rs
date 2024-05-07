
use std::io::{stdin,stdout,Write};

pub fn get_menu_option() -> Option<i32> {
	print!("Enter a number: ");
	let mut user_input = String::new();
	let _ = stdout().flush();
	stdin().read_line(&mut user_input).expect("Did not enter a correct string!!");
	
	if let Some('\n') = user_input.chars().next_back() {
		user_input.pop();
	}
	if let Some('\r') = user_input.chars().next_back() {
		user_input.pop();
	}
	
	match user_input.parse::<i32>() {
		Ok(parsed_int) => Some(parsed_int),
		Err(_) => None,
	}
}

pub fn get_task_from_user() -> String {
	println!("What task do you have in mind?");
	print!("Enter your task: ");

	let mut user_input = String::new();
	let _ = stdout().flush();
	stdin().read_line(&mut user_input).expect("Did not enter a correct string!!");
	if let Some('\n') = user_input.chars().next_back() {
		user_input.pop();
	}
	if let Some('\r') = user_input.chars().next_back() {
		user_input.pop();
	}
	
	user_input
}

pub fn get_task_index_from_user() -> Option<i32> {
	print!("Enter the number of the task you want to delete: ");
	let mut user_input = String::new();
	let _ = stdout().flush();
	stdin().read_line(&mut user_input).expect("Did not enter a correct string!!");
	
	if let Some('\n') = user_input.chars().next_back() {
		user_input.pop();
	}
	if let Some('\r') = user_input.chars().next_back() {
		user_input.pop();
	}
	
	match user_input.parse::<i32>() {
		Ok(parsed_int) => Some(parsed_int),
		Err(_) => None,
	}
}