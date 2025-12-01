use shared::Message;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("🐝 Drone is trying to connect...");

    // Connect to the Queen (localhost:8080)

    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut socket) => {
            println!("🐝 Connected to the Queen!");

            let mut buffer = [0u8; 512]; // Create a buffer to hold incoming data

            // Read data from the TCP stream into the buffer

            match socket.read(&mut buffer) {
                Ok(bytes_read) => {
                    let received_data = &buffer[..bytes_read];

                    let msg: Message = serde_json::from_slice(received_data).unwrap();

                    match msg {
                        Message::HelloDrone => println!("👑 Queen says Hello!"),
                        Message::Task(task_name) => {
                            println!("I recieved a job");
                            println!("Processing task: {}", task_name);
                            println!("Working...");


                            let reply = Message::TaskComplete(task_name);
                            let json_reply = serde_json::to_string(&reply).unwrap();
                            socket.write_all(json_reply.as_bytes()).unwrap();
                            print!("Sent task completion message to Queen");
                        }
                        _ => println!("Unknown message received"),
                    }
                }
                Err(e) => {
                    println!("Failed to read: {}", e);
                }
            }
        }

        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
