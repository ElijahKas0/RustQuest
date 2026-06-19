mod commands;
mod filesystem;
mod session;

use std::{io::{self,Write,BufRead, BufReader},env,sync::Arc,thread,net::{TcpListener,TcpStream}};
use commands::{execute,parse};
use crate::filesystem::FileSystem;
use crate::session::Session;


fn handle_client (mut stream: TcpStream,filesystem: Arc<FileSystem>,) {
	
	let _ = stream.write_all(
		b"Im jennie give you three wishes\nType help for summon jennie\n> "
	);

	let reader_stream =stream.try_clone().unwrap();

	let mut reader =
		BufReader::new(reader_stream);
	
	let peer_addr = stream
		.peer_addr()
		.map_or_else(|_| "unknown".to_string(),|addr| addr.to_string());
	println!("Handling connection from: {}",peer_addr);
	
	let mut session = Session::new();

	loop {

		let mut command = String::new();

    	match reader.read_line(&mut command) {
			Ok(n) => {
				if n==0 {
					println!("Client {} closed connection", peer_addr);
					break;
				}

                let parsed =
                    parse(&command);

                let response =
                    execute(
                        parsed,
						&mut session,
                        &filesystem,
                        
                    );
				
				if let Err(e) = match response {
					Ok(mut text) => {
						text.push_str("\n>");
						stream.write_all(text.as_bytes())
					},
					Err(mut err) => {
						err.push_str("\n>");
						stream.write_all(err.as_bytes())
					},
				} {
					eprintln!("Write error: {e}");
					break;
				}
				
			}
			Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
			Err(e) => {
				match e.kind() {
					io::ErrorKind::ConnectionReset => {
						println!("Client {} reset connection",peer_addr);
					}
					_ => {
						eprintln!(
						"Read error from client {}: {}",peer_addr,e
						);
						
					}
				
				}
				break;
			}
		}
	}
	println!("Connection finished for : {}", peer_addr);
}

fn main() {
	let addr = env::args()
	.nth(1)
	.unwrap_or_else(|| "0.0.0.0:9090".to_string());
	
	let listener = TcpListener::bind(&addr).expect("Failed to bind to address");

	let filesystem = Arc::new(
		FileSystem::new()
		);


	for stream_result in listener.incoming() {
		match stream_result {
			Ok(stream) => {

				let fs = Arc::clone(&filesystem);
				
				thread::spawn(move || {
					handle_client(stream,fs);
				});
			}
			Err(e) => {
				eprintln!("Failed to accept connection: {}",e)
			}
		}
	}
}
