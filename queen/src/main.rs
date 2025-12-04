use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use shared::Message; 
use std::thread;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};



#[tokio::main]
async fn main() {

    // Setup database
    //this created a file named hive.db in you file

    let pool = SqlitePoolOptions::new()
    .connect("sqlite://hive.db?mode=rwc").await.expect("Fail to connect to the databsae");

    

    // Creating A Table storing task name and corresponding result
    sqlx::query("CREATE TABLE IF NOT EXISTS completed_tasks(id INTEGER PRIMARY KEY, task_name TEXT NOT NULL, result_hash TEXT NOT NULL )",).execute(&pool).await.unwrap();

    println!("Database Connected, Table Ready.");



    // 1. Bind to the port (Start the Server)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("👑 Queen is listening on port 8080...");


    // 2. Loop forever, waiting for Drones
    for stream in listener.incoming() {
        match stream {
            Ok( socket) => {
                println!("🐝 A Drone just connected!");

                let db = pool.clone();

              thread::spawn(move || {handle_client(socket, db);});
                
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}


fn handle_client(mut socket: TcpStream, pool: Pool<Sqlite>){
    let task_payload = "SuperSecretTaskData";

    let msg = Message::Task("task_payload".to_string());
    let json_msg = serde_json::to_string(&msg).unwrap();
    socket.write_all(json_msg.as_bytes()).unwrap();
    print!("Task {} sent to background thread\n" , task_payload);


    let mut  buffer = [0; 512];

    match socket.read(&mut buffer){
       Ok(bytes_read) => {
        if bytes_read == 0 {return;}
        let received_data = &buffer[..bytes_read];
        
        if let Ok(reply_msg) = serde_json::from_slice::<Message>(received_data){
            println!("Received hash from drone: {:?}", reply_msg);

            if let Message::TaskComplete(hash_result) = reply_msg{

            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async{
                sqlx::query("INSERT INTO completed_tasks (task_name, result_hash) VALUES(?,?)")
                .bind(task_payload)
                .bind(hash_result)
                .execute(&pool)
                .await.unwrap();
            });

            print!("Stored to database!\n");




            }
        } else {
            println!("Failed to parse message from drone");
        }

       }

         Err(e) => {
          println!("Failed to read from socket: {}", e);
         }
    } 


}
