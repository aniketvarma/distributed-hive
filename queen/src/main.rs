use std::net::TcpListener;
use std::io::{Write, Read};
use shared::Message; // <--- Look! We are using our own library!

fn main() {
    // 1. Bind to the port (Start the Server)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("👑 Queen is listening on port 8080...");

    // 2. Loop forever, waiting for Drones
    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                println!("🐝 A Drone just connected!");

                // 3. Create the Message object
                let msg = Message::Task("resize_image_123.jpg".to_string());

                // 4. SERIALIZE: Turn the fancy Rust Enum into a text string (JSON)
                // This is where Serde does the magic work.
                let json_message = serde_json::to_string(&msg).unwrap();

                // 5. Write the JSON string to the TCP stream (Send it)
                socket.write_all(json_message.as_bytes()).unwrap();

                print!("👑 Sent task to Drone, waiting for response...\n");

                let mut buffer = [0u8; 512];

                match socket.read(&mut buffer) {

                 Ok(bytes_read) => {

                    let received_data = &buffer[..bytes_read];
                    let reply_msg: Message = serde_json::from_slice(received_data).unwrap();
                    println!("🎉 Queen received report: {:?}", reply_msg);
                 }

                    Err(e) => {
                        println!("Failed to read from socket: {}", e);
                    }
                }
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
