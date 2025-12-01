use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use shared::Message; 
use std::thread;

fn main() {
    // 1. Bind to the port (Start the Server)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("👑 Queen is listening on port 8080...");

    // 2. Loop forever, waiting for Drones
    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                println!("🐝 A Drone just connected!");

              thread::spawn(move || {handle_client(socket);});
                
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}


fn handle_client(mut socket: TcpStream){

    let msg = Message::Task("image1.png".to_string());
    let json_msg = serde_json::to_string(&msg).unwrap();
    socket.write_all(json_msg.as_bytes()).unwrap();
    print!("Task sent to background thread\n");


    let mut  buffer = [0; 512];

    match socket.read(&mut buffer){
       Ok(bytes_read) => {
        if bytes_read == 0 {return;}
        let received_data = &buffer[..bytes_read];
        
        if let Ok(reply_msg) = serde_json::from_slice::<Message>(received_data){
            println!("Received reply from drone: {:?}", reply_msg);
        } else {
            println!("Failed to parse message from drone");
        }

       }

         Err(e) => {
          println!("Failed to read from socket: {}", e);
         }
    } 
}
