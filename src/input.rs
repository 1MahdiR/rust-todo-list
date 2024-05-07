
use std::io::{stdin,stdout,Write};

use super::MENU;

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
	println!("You typed: {}", user_input);
	
	match user_input.parse::<i32>() {
		Ok(parsed_int) => Some(parsed_int),
		Err(_) => None,
	}
}

