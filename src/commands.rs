//this file contains methods used to process commands

pub mod Messages
{
	//\x1b[31;1m bold red
	pub const WELCOME_MESSAGE: &'static str = "Welcome to the mud\r\nWhat's your name?\r\n";
	pub const REGISTER_MESSAGE: &'static str = "This appears to your first time here, \r\nwould you like to visit us in the mud world?\r\n";
	pub const ENTER_PASSWORD: &'static str = "Please enter your pass code\r\n";
	pub const REGISTER_USERNAME: &'static str = "Please enter a username\r\n";
	pub const REGISTER_PASSWORD: &'static str = "Password please:\r\n";
	pub const CONFIRM_PASSWORD: &'static str = "Please confirm your password:\r\n";
	pub const CREATE_CHARACTER: &'static str =	"Let's build your character\r\n";
}

fn process_new_user()
{
	println!("process new user");
}

pub fn parse_input(input_raw: &String)
{
	let input = input_raw.clone().trim().to_lowercase();
	let parts: Vec<&str> = input.split_whitespace().collect();
	
	println!("Received command {}", input);
	match parts[0]
	{
		"new" =>
		{
			process_new_user();
		},
		_ =>
		{
			
		}
	}
}