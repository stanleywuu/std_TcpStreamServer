use character;

use std::net::{TcpListener, TcpStream};
use commands::Messages;
use connection;
use storage;
use socketutilities;

// traits
use std::io::Read;
use std::io::Write;

//this module deals with sign up and login

enum LogonState
{
	New,
	Username,
	Password,
	Register_New_User,
	Register_Password,
	Register_Password_Confirm,
	Register_Creation,
	
	
	Done
}

pub struct logon_manager<'client>
{
	stream: &'client TcpStream,
	username: String,
	password: String,
	logon_state: LogonState,
	
}

impl<'client> logon_manager<'client>
{
	// constructor
	pub fn new(client: &TcpStream) -> logon_manager
	{
		logon_manager
		{
			stream: client,
			username: String::new(),
			password: String::new(),
			logon_state: LogonState::New,
		}
	}
	
	// initial welcome message
	fn welcome_message(&mut self)
	{
		let msg = Messages::WELCOME_MESSAGE;		
		match self.stream.write(msg.as_bytes()) {
			Err(_) => {},
			Ok(_) => 
			{
				self.logon_state = LogonState::Username;
			},
		}
	}
	
	//check whether user already exists
	fn user_exists(&self) -> bool
	{
		let mut db = storage::get_db();
		return db.entry_exists("player", &self.username[..]);
	}
	
	//save user
	fn save_player(&self)
	{
		let mut db = storage::get_db();
		let mut data:Vec<storage::DataColumn> = Vec::new();
		
		data.push(storage::DataColumn::new("password".to_string(), self.password.clone()));
		data.push(storage::DataColumn::new("stage".to_string(), "creation".to_string()));
		
		match db.insert("player", &self.username[..], data)
		{
			Ok(_) => println!("Successfully saved user {}|", &self.username),
			Err(e) => println!("Failed to create user {}", e),
		}
	}
	
	//process inputs
	fn process_commands(&mut self)
	{
		let stream = self.stream;
		let mut input_string = String::new();
		
		match self.logon_state
		{
			LogonState::Register_Creation => {},
			LogonState::Done => {},
			_=>
			{
				input_string= socketutilities::read_incoming_data(self.stream);
			}
		}
		
		let mut input = input_string.trim().to_string();
		
		println!("Received command: {} EOF", input);
		match self.logon_state
		{
			LogonState::New => {},
			LogonState::Username =>
			{
				//determines if username exists
				self.username = input.to_string();
				if !self.user_exists()
				{
					socketutilities::send_message(stream, &Messages::REGISTER_MESSAGE);
					self.logon_state = LogonState::Register_New_User;
				}
				else
				{
					socketutilities::send_message(stream, &Messages::ENTER_PASSWORD);
					self.logon_state = LogonState::Password;
				}
			},
			LogonState::Password =>
			{
				//retrive record and compare password
				self.logon_state = LogonState::Done;
			}
			LogonState::Register_New_User =>
			{
				//checks whether input is y or n
				if input == "y" || input == "yes"
				{
					socketutilities::send_message(stream, &Messages::REGISTER_PASSWORD);
					self.logon_state = LogonState::Register_Password;
				}
				else
				{
					self.logon_state = LogonState::New;
					socketutilities::send_message(stream, &Messages::WELCOME_MESSAGE);	
				}
			},			
			LogonState::Register_Password =>
			{
				self.password = input.to_string();
				
				socketutilities::send_message(stream, &Messages::CONFIRM_PASSWORD);
				self.logon_state = LogonState::Register_Password_Confirm;				
			},
			LogonState::Register_Password_Confirm =>
			{
				if input.to_string() == self.password
				{
					self.save_player();
					socketutilities::send_message(stream, &Messages::CREATE_CHARACTER);
					self.logon_state = LogonState::Register_Creation;
				}				
				else
				{
					self.logon_state = LogonState::Register_Password;
					socketutilities::send_message(stream, &Messages::REGISTER_PASSWORD);
				}
			},
			LogonState::Register_Creation => 
			{
				let mut character_creator = character::CharCreator::new(self.stream);
				println!("creating character");
				character_creator.handle_input();
				self.logon_state = LogonState::Done;
			},
			
			LogonState::Done => {},
		}
	}
	
	pub fn handle_logon(&mut self)
	{	
		loop
		{
			match self.logon_state
			{
				LogonState::New => self.welcome_message(),
				LogonState::Done => {
					break;
					}
				_ => {
					self.process_commands();
				}
			}						
		}
	}
}