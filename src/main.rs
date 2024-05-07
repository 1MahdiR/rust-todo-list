
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
			Some(3) => {
				tasks = read_tasks_from_file();
				if tasks.len() != 0 {
					let task_index = get_task_index_from_user();
					match task_index {
						Some(parsed_int) => {
							if parsed_int > tasks.len() as i32 || parsed_int < 1 {
								println!("No task with number {}.", parsed_int);
							} else {
								delete_a_task_from_file(parsed_int);
								display_in_box("Task was successfully deleted.");
							}
						}
						None => println!("You entered no number!"),
					}
				} else {
					println!("There is no task to delete!");
				}
			}
			Some(4) => {
				break;
			}
			Some(_) => println!("You typed: something else"),
			None => println!("You typed no number!"),
		}
	}
	println!("BYE!")
}
