use socketutilities;
use std::net::{TcpListener, TcpStream};

//module for character related methods

pub struct Character
{
	display_name: String,
	gender: String,
	race: String,
	description: String,
	hp: i32,
	hp_current: i32,
	stam: i32,
	stam_current: i32,
	atk: i32,
	def: i32,
	agi: i32,
	int: i32,
	luk: i32,
	items: Vec<()>,
	equips: Vec<()>,
	skills: Vec<()>,
	history: Vec<()>,
}

impl Character
{
	pub fn new() -> Character
	{
		Character
		{
			display_name: String::new(),
			gender: String::new(),
			race: String::new(),
			description: String::new(),
			hp: 20,
			hp_current: 20,
			stam: 20,
			stam_current: 20,
			atk: 1,
			def: 1,
			agi: 1,
			int: 1,
			luk: 1,
			items: Vec::new(),
			equips: Vec::new(),
			skills: Vec::new(),
			history: Vec::new(),
		}
	}
}


pub mod Messages
{
	//\x1b[31;1m bold red
	pub const RACESELECTION: &'static str = "What would you like to be? \r\n[1]Human\t\t[2]Elf\t\t[3]Dwarf\t\t[4]Dragon\r\n";
	pub const GENDERSELECTION: &'static str = "Gender[m/f/u]?\r\n";
	pub const TYPESELECTION: &'static str = "What kind of {} would you like to be?\r\n[1]Intelligent\t\t[2]Atheletic\t\t[3]Average\r\n";
	pub const ATTRSELECTION: &'static str = "Are you satisfied with the following attributes?\r\n";
	pub const SUCCESS: &'static str = "Your character has been created\r\n";
}

enum CreationState
{
	New,
	Race,
	Gender,
	Type,
	Selection,
	
	Done
}

pub struct CharCreator<'client>
{
	stream: &'client TcpStream,	
	character: Character,
	state: CreationState,
}

impl<'client> CharCreator<'client>
{
	// constructor
	pub fn new(client: &TcpStream) -> CharCreator
	{
		CharCreator
		{
			stream: client,
			state: CreationState::New,
			character: Character::new(),
		}
	}
	
	fn is_input_valid(&self, min:i32, max:i32, input: String) -> i32
	{
		let selection: i32 = match input.trim().parse()
		{	Ok(num) => num,
			Err(_) => -1,
		};
		
		if selection > min && selection < max
		{
			selection
		}
		else
		{
			0
		}
	}
	
	fn is_string_valid(&self, selection: String, input:String) -> bool
	{
		let parts: Vec<&str> = selection.split(':').collect();
		let mut valid = false;
		
		for part in parts
		{
			if input == part.to_string() {valid = true; break;}				
		}
		
		valid
	}
	
	pub fn handle_input(&mut self)
	{
		let stream = self.stream;
		let mut input_string = String::new();
				
		loop
		{				
			match self.state
			{
				CreationState::New => {println!("state is new");},
				CreationState::Done => {},
				_=>
				{
					input_string = socketutilities::read_incoming_data(self.stream);
				}
			}
			
			let input = input_string.trim().to_string();
			match self.state
			{
				CreationState::New=>
				{
					socketutilities::send_message(stream, &Messages::RACESELECTION);
					self.state = CreationState::Race;
				}
				CreationState::Race=>{
					//save race
					if self.is_input_valid(1, 4, input) > 0
					{							
						socketutilities::send_message(stream, &Messages::GENDERSELECTION);
						self.state = CreationState::Gender;
					}
					else
					{
						self.state = CreationState::New;
					}
				},
				CreationState::Gender=>
				{
					//save gender
					if self.is_string_valid("m:f:u".to_string(), input)
					{							
						socketutilities::send_message(stream, &Messages::TYPESELECTION);
						self.state = CreationState::Type;
					}
					else
					{
						socketutilities::send_message(stream, &Messages::GENDERSELECTION);
					}
				},
				CreationState::Type => 
				{
					if self.is_input_valid(0, 4, input) > 0
					{
						socketutilities::send_message(stream, &Messages::ATTRSELECTION);
						self.state = CreationState::Selection;
					}
					else
					{
						socketutilities::send_message(stream, &Messages::TYPESELECTION);
					}
				},
				CreationState::Selection =>
				{
					if input == "y" || input == "yes"
					{
						socketutilities::send_message(stream, &Messages::SUCCESS);
						self.state = CreationState::Done;
						break;
					}
					else
					{
						socketutilities::send_message(stream, &Messages::ATTRSELECTION);
					}
				},
				CreationState::Done =>
				{
					break;
				}
			}		
		}
	}
	
}