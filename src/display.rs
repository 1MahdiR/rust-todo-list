pub const MENU:[&str; 4] = [
	"1. Display todo list",
	"2. Add task",
	"3. Remove task",
	"4. Exit",
];

pub const TITLE:&str = "#-+-+-+-+-+-+-+- TODO LIST -+-+-+-+-+-+-+-+-#\n";
pub const FOOTER:&str ="#-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-#\n";

fn display_title() {
	println!("{TITLE}");
}

fn display_options() {
	for i in MENU {
		println!("{i}");
	}
	println!("");
}

fn display_footer() {
	println!("{FOOTER}");
}

pub fn display_in_box(text: &str) {
	let banner = "-".repeat(text.len() + 2);
	println!("+{}+", banner);
	println!("| {} |", text);
	println!("+{}+", banner);
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

	if tasks.len() == 0 {
		println!("There are no tasks!");
		return;
	}

	let banner = "-".repeat((maximum_length + 5) as usize);
	println!("+{}+", banner);
	for (i, j) in tasks.iter().enumerate() {
		print!("| {}: {}", i+1, j);
		print!("{}", " ".repeat((maximum_length as usize) - j.len()));
		print!(" |\n");
	}
	println!("+{}+", banner);
}

