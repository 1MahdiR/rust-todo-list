pub const MENU:[&str; 4] = [
	"1. Display todo list",
	"2. Add task",
	"3. Remove task",
	"4. Exit",
];

pub const TITLE:&str = "#-+-+-+-+-+-+-+- TODO LIST -+-+-+-+-+-+-+-+-#\n";
pub const FOOTER:&str ="#-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-#\n";

pub fn display_title() {
	println!("{TITLE}");
}

pub fn display_options() {
	for i in MENU {
		println!("{i}");
	}
	println!("");
}

pub fn display_footer() {
	println!("{FOOTER}");
}

pub fn display_menu() {
	println!("\n\n\n");
	display_title();
	display_options();
	display_footer();
}

pub fn display_tasks(tasks: &Vec<String>) {
	let mut maximum_length = 0;
	for i in tasks {
		let length = i.len() as i32;
		if maximum_length < length {
			maximum_length = length;
		}
	}
	
	//println!("{maximum_length}");
	for (i, j) in tasks.iter().enumerate() {
		println!("{i}: {j}");
	}
}
