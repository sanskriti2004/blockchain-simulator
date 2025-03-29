// importing dependencies
// Importing SHA-256 hashing algorithm  
use sha2::{Digest, Sha256};  

use core::time;
// For implementing custom formatting (if needed later)  
use std::fmt;  

// For getting the current system time  
use std::time::{SystemTime, UNIX_EPOCH};  

// For handling multi-threading (if mining involves parallel processing)  
use std::thread;  

// For adding delays (useful in simulating mining difficulty)  
use std::time::Duration;  


// difficulty level
const DIFFICULTY: u32 = 4;

// struct for block
struct Block {
    previous_hash: String,
    timestamp: u64,
    index: u32,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index:u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get current time").as_secs();
    
        Block {
            previous_hash,
            timestamp,
            index,
            data,
            nonce: 0,
            hash: String::new(),
        }

    }

    fn calculate_hash(&mut self) -> String {
        let data = format!("{}{}{}{}{}", self.previous_hash, self.timestamp, self.index, self.data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let res = hasher.finalize();
        let hash_str = format!("{:x}", res);
        hash_str
        }
    
}
fn main() {  
    println!("Welcome to the MehCoin mining simulator!");  
}














fn main() {
    println!("Welcome to the MehCoin mining simulator!");
}
