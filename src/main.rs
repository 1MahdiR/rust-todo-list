
mod display;
mod input;
mod file;

use display::*;
use input::*;
use file::*;

fn main() {
	let mut tasks: Vec<String>;

	loop {
		display_menu();
		match get_menu_option() {
			Some(1) => {
				tasks = read_tasks_from_file();
				display_tasks(&tasks);
			},
			Some(2) => {
				let task = get_task_from_user();
				write_task_to_file(&task);
				display_in_box("Task was successfully added.");
			}
			Some(_) => println!("You typed: something else"),
			None => println!("You typed no number!"),
		}
	}
}
