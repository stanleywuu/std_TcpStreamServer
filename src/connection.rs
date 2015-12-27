use std::net::{TcpStream};
use commands;
use commands::Messages;
use logon;

// traits
use std::io::Read;
use std::io::Write;

pub fn read_incoming_data(stream: &mut TcpStream) -> String
{
		let mut buf;
		let mut buf_ps = Vec::new();
		
		buf = [0;500];
		let _ = match stream.read(&mut buf) {
			Err(e) => panic!("Got an error: {}", e),
			Ok(m) => {
				if m == 0 {
					// we've got an EOF											
				}
				m
			},
		};
		
		for index in 0..499
		{
			if buf[index] == 0
			{
				break;
			}
			buf_ps.push(buf[index]);
		}
		
		String::from_utf8(buf_ps.clone()).unwrap()
}

fn str_2_u8<'a>(raw: &'a str) -> &'a [u8]
{
	return raw.as_bytes();
}
	
enum PlayerState
{
	New,
}

pub struct Player
{
	id: i32,
	stream: TcpStream,
	state: PlayerState
}

impl Player
{
	pub fn new(tcp: TcpStream) -> Player
	{
		Player
		{
			id: 0,
			stream: tcp,
			state: PlayerState::New,
		}
	}


	pub fn handle_client(&mut self) {
				
		loop {
			// clear out the buffer so we don't send garbage
			println!("incoming connection");
			//send welcome message first
			match self.state
			{
				PlayerState::New =>
				{	
					let mut stream= &self.stream;
					let mut logon_stream = stream;
					let mut logon_mgr = logon::logon_manager::new(logon_stream);
					logon_mgr.handle_logon();					
				},
				//_ => {}
			}
			
			let input = read_incoming_data(&mut self.stream);
			
			commands::parse_input(&input);			
		}
	}
}