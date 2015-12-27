extern crate time;
use std::net::{TcpStream};

// traits
use std::io::Read;
use std::io::Write;

// method to convert input into string
pub fn read_incoming_data(mut stream: &TcpStream) -> String
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

//sending messages to client
pub fn send_message(mut stream: &TcpStream, message: &str)
{
	let now = time::now();	
	let msg = format!("[{}:{}:{}]{}", now.tm_hour, now.tm_min, now.tm_sec, message);
	match stream.write(msg.as_bytes())
	{
		Err(_) => {},
		Ok(_) => {},
	}
}