// Importing SHA-256 hashing algorithm
use sha2::{Digest, Sha256};

// Importing standard Rust libraries
use std::fmt; // For implementing custom formatting
use std::io;  // For handling user input
use std::time::{SystemTime, UNIX_EPOCH}; // For getting the current system time
use std::thread; // For handling multi-threading
use std::time::Duration; // For adding delays

// Importing chrono for working with timestamps
use chrono::{NaiveDateTime, Utc};

// Difficulty level (number of leading zeros required in the hash)
const DIFFICULTY: usize = 4;

// Struct representing a block in the blockchain
struct Block {
    previous_hash: String, // Hash of the previous block
    timestamp: u64,        // Time when the block was created
    index: u32,            // Block index (position in blockchain)
    data: String,          // Transaction data stored in the block
    nonce: u64,            // Number used to vary hash computation (proof of work)
    hash: String,          // Hash of the current block
}

impl Block {
    // Constructor for creating a new block
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get current time")
            .as_secs(); // Get current time in seconds

        Block {
            previous_hash,
            timestamp,
            index,
            data,
            nonce: 0,
            hash: String::new(), // Empty hash (to be computed later)
        }
    }

    // Function to calculate the SHA-256 hash of the block
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.previous_hash, self.timestamp, self.index, self.data, self.nonce
        );

        let mut hasher = Sha256::new(); // Create a SHA-256 hasher
        hasher.update(data.as_bytes()); // Hash the block data
        let res = hasher.finalize();

        format!("{:x}", res) // Convert hash to a hexadecimal string
    }

    // Proof-of-work mining function
    fn block_mining(&mut self) {
        let mut iterations = 0;
        loop {
            iterations += 1;
            self.hash = self.calculate_hash();

            // Check if hash meets difficulty target (starts with N zeros)
            if self.hash.starts_with(&"0".repeat(DIFFICULTY)) {
                println!("Block mined successfully: #{}", self.index);
                break;
            }

            // Display progress if taking too long
            if iterations > 100 {
                println!("Block mining in progress...");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }

            self.nonce += 1; // Increment nonce to generate a new hash
        }
    }
}

// Implementing display formatting for the Block struct
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert timestamp to readable date-time format
        let datetime = NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0)
            .expect("Invalid timestamp");

        write!(
            f,
            "Block #{}\nPrevious Hash: {}\nTimestamp: {}\nData: {}\nNonce: {}\nHash: {}\n",
            self.index, self.previous_hash, datetime, self.data, self.nonce, self.hash
        )
    }
}

// Struct for Blockchain (a collection of blocks)
struct Blockchain {
    chain: Vec<Block>, // Vector to store the blocks
}

impl Blockchain {
    // Initialize a new blockchain with a genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    // Function to add a new block to the blockchain
    fn add_block(&mut self, mut new_block: Block) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = prev_hash;
        new_block.block_mining();
        self.chain.push(new_block);
    }

    // Function to get total number of blocks
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!("Welcome to the MehCoin mining simulator!");

    // Taking miner's name as input
    println!("Enter your miner name: ");
    let mut miner_name = String::new();
    io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");
    miner_name = miner_name.trim().to_string();

    // List of traders for transactions
    let trader_names = vec!["Jason", "Woj", "Arky", "Ron", "Lacy", "Max", "Silky", "Sunny"];

    let mut mehcoin = Blockchain::new(); // Create a new blockchain

    println!("Let's start mining!");
    let mut sender = miner_name.clone();

    // Mining multiple blocks (simulating transactions)
    for i in 0..trader_names.len() {
        println!("Mining block #{}", i + 1);

        let receiver = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        }; // Determine receiver of transaction

        let transaction = format!("{} sent MehCoin to {}", sender, receiver);
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        mehcoin.add_block(new_block);
        println!("Transaction: {}", transaction);

        sender = receiver.clone();
        println!();
    }

    // Display blockchain statistics
    let total_blocks = mehcoin.get_total_blocks();
    println!("Total blocks mined: {}", total_blocks);

    let mehcoin_per_block = 137;
    let mehcoin_traded = total_blocks * mehcoin_per_block;
    println!("Total MehCoin traded: {}", mehcoin_traded);

    // Get and format the end timestamp
    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_secs();

    let end_datetime = NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0)
        .expect("Invalid timestamp");

    println!(
        "Simulation ended at: {}",
        end_datetime.format("%Y-%m-%d %H:%M:%S")
    );
    println!("Thank you for using the MehCoin mining simulator!");
}
