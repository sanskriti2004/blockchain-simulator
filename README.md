
# Blockchain Mining Simulator

A **Rust-based blockchain mining simulator** implementing **SHA-256 hashing, proof-of-work (PoW), and a simplified blockchain structure**. This project demonstrates core blockchain principles, including **decentralized ledger management, cryptographic hashing, and transaction validation**.

## Features

* **Proof-of-Work (PoW) Mining**: Implements a mining algorithm that requires solving computational puzzles.
* **Blockchain Structure**: Blocks are linked via cryptographic hashes, ensuring immutability.
* **SHA-256 Hashing**: Used for secure block validation and transaction integrity.
* **Multi-threaded Simulation**: Simulates mining difficulty and delays to mimic real-world blockchain behavior.
* **Chrono Timestamping**: Each block is time-stamped for accurate tracking.
* **Automated Transaction Processing**: Simulated transactions between predefined traders.

## Tech Stack

* **Rust** (Safe, fast, and memory-efficient programming)
* **SHA-256 (sha2 crate)** (Cryptographic hashing)
* **Chrono** (Time-based functionalities)
* **Multi-threading (std::thread)** (Simulating mining complexity)

## Installation & Usage

1. Clone the repository:

   ```sh
   git clone https://github.com/sanskriti2004/blockchain-simulator.git
   cd blockchain-simulator
   ```
2. Install Rust (if not already installed):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Run the simulator:

   ```sh
   cargo run
   ```

## Demo

When you run the program, you'll:

* Enter your miner name.
* Witness blocks being mined with real-time hash calculations.
* See transactions processed between traders.
* Get the total number of blocks mined and cryptocurrency traded.

### Sample Output:

```
Welcome to the crypto mining simulator!
Enter your miner name: Sans
Mining block #1...
Block mined successfully: #1
Transaction: Sans sent cryptocurrency to Jason

Mining block #2...
Block mined successfully: #2
Transaction: Jason sent cryptocurrency to Woj

Total blocks mined: 8
Total cryptocurrency traded: 1096
Simulation ended at: 2025-03-29 14:45:00
Thank you for using the crypto mining simulator!
```

## Why This Project?

This simulator is a **great demonstration of blockchain fundamentals** in a lightweight, easy-to-understand manner, making it perfect for **learning, portfolio showcasing, or further blockchain experimentation**.

## Contributing

Feel free to fork this repository and submit pull requests if you want to improve the project!

---
