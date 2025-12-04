use serde::{Deserialize, Serialize};

// This is the "Message" enum.
// It lists EVERYTHING the Queen and Drone are allowed to say to each other.
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HelloDrone,           // Queen says "Hi"
    HelloQueen,           // Drone says "Hi"
    Task(String),         // Queen says "Do this task: [text]"
    TaskComplete(String), // Drone says "I finished: [text]"
}