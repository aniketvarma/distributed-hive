use shared::Message;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("🐝 Drone is trying to connect...");

    // Connect to the Queen (localhost:8080)
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(socket) => {
            // delegate the per-connection work to a helper function
            handle_server(socket);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn handle_server(mut socket: TcpStream) {
    println!("🐝 Connected to the Queen!");

    let mut buffer = [0u8; 512]; // Create a buffer to hold incoming data

    // Read data from the TCP stream into the buffer
    match socket.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                eprintln!("Received zero bytes from server");
                return;
            }

            let received_data = &buffer[..bytes_read];

            match serde_json::from_slice::<Message>(received_data) {
                Ok(msg) => match msg {
                    Message::HelloDrone => println!("👑 Queen says Hello!"),
                    Message::Task(task_name) => {
                        println!("🚨 I received a JOB: {}", task_name);
                        println!("...Working...");

                        let reply = Message::TaskComplete(task_name);
                        let json_reply = serde_json::to_string(&reply).unwrap();
                        if let Err(e) = socket.write_all(json_reply.as_bytes()) {
                            eprintln!("Failed to write reply: {}", e);
                        } else {
                            println!("✅ Sent completion report to Queen!");
                        }
                    }
                    _ => println!("Unknown message received"),
                },
                Err(e) => {
                    eprintln!("Failed to parse message: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to read: {}", e);
        }
    }
}
