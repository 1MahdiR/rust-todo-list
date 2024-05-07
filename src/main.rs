
mod display;
mod input;
mod file;

use display::*;
use input::*;
use file::*;

fn main() {
	let mut tasks: Vec<String> = Vec::new();

	loop {
		display_menu();
		match get_menu_option() {
			Some(1) => {
				tasks = read_tasks_from_file();
				display_tasks(&tasks);
			},
			Some(_) => println!("You typed: something else"),
			None => println!("You typed no number!"),
		}
	}
}
