# Biometric Blockchain System

A Rust-based blockchain implementation designed for storing and validating biometric data with a web interface for interaction.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Architecture](#architecture)
- [How It Works](#how-it-works)
  - [Backend](#backend)
  - [Frontend](#frontend)
- [Data Structures](#data-structures)
  - [Block Structure](#block-structure)
  - [Blockchain Structure](#blockchain-structure)
- [Algorithms](#algorithms)
  - [Proof of Work](#proof-of-work)
  - [Hashing](#hashing)
  - [Blockchain Validation](#blockchain-validation)
- [API Endpoints](#api-endpoints)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running the Application](#running-the-application)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Overview

This project demonstrates a simple blockchain implementation in Rust, specifically designed for storing biometric data hashes. It provides both a command-line interface and a web interface for interacting with the blockchain. The system implements a Proof of Work (PoW) consensus mechanism to secure the blockchain.

## Features

- **Blockchain Implementation**: Custom blockchain with blocks linked through cryptographic hashes
- **Proof of Work**: Mining mechanism to secure the blockchain
- **Biometric Data Storage**: Secure storage of biometric data hashes
- **Web Interface**: User-friendly web interface for interacting with the blockchain
- **REST API**: HTTP endpoints for programmatic access
- **Data Persistence**: Save and load blockchain from JSON files
- **Validation**: Built-in blockchain validation mechanism
- **CLI Interface**: Command-line interface for direct interaction

## Technology Stack

- **Language**: Rust (2024 edition)
- **Web Framework**: Actix-web v4
- **Serialization**: Serde v1.0
- **Hashing**: SHA-256 (sha2 v0.10)
- **Time Handling**: Chrono v0.4
- **UUID Generation**: uuid v1.0
- **Frontend**: Vanilla JavaScript, HTML, CSS (no external frameworks)

## Architecture

```
+------------------+     +------------------+     +------------------+
|   Web Browser    |<--->|  Actix-web API   |<--->|  Blockchain Core |
+------------------+     +------------------+     +------------------+
                                                       |
                                               +------------------+
                                               |   Data Storage   |
                                               +------------------+
```

The system follows a layered architecture:
1. **Frontend Layer**: Web interface running in the browser
2. **API Layer**: Actix-web REST API handling HTTP requests
3. **Business Logic Layer**: Blockchain core implementation
4. **Data Layer**: File-based persistence (JSON)

## How It Works

### Backend

The backend is built with Rust and Actix-web, providing a robust and efficient blockchain implementation:

1. **Initialization**:
   - On startup, the application attempts to load an existing blockchain from `biometric_chain.json`
   - If no existing blockchain is found, it creates a new one with a genesis block
   - The user can choose between the web interface or command-line interface

2. **Block Creation**:
   - When adding biometric data, a new block is created with:
     - Unique UUID identifier
     - Index (position in the chain)
     - Timestamp of creation
     - Hash of the biometric data (SHA-256)
     - Previous block's hash
     - Nonce for Proof of Work
     - Current block's hash (SHA-256 of all block data)

3. **Mining Process**:
   - Before adding a block to the chain, it must be "mined"
   - Mining involves finding a nonce that produces a hash with a specific number of leading zeros
   - This process requires computational work, hence "Proof of Work"

4. **Persistence**:
   - The blockchain can be saved to a JSON file
   - On startup, the application attempts to load from this file

### Frontend

The frontend is a single-page application embedded in the Rust binary:

1. **User Interface**:
   - Clean, responsive design with no external dependencies
   - Real-time display of blockchain information
   - Form for adding new biometric data
   - Action buttons for blockchain operations

2. **JavaScript Functionality**:
   - AJAX calls to backend API endpoints
   - Dynamic updating of blockchain information
   - User feedback through success/error messages
   - Automatic refresh of blockchain stats

## Data Structures

### Block Structure

```rust
pub struct Block {
    pub id: String,                  // UUID identifier
    pub index: u64,                  // Position in the chain
    pub timestamp: DateTime<Utc>,    // Creation timestamp
    pub biometric_data_hash: String, // SHA-256 hash of biometric data
    pub previous_hash: String,       // Hash of the previous block
    pub nonce: u64,                  // Nonce for Proof of Work
    pub hash: String,                // SHA-256 hash of all block data
}
```

### Blockchain Structure

```rust
pub struct Blockchain {
    pub chain: Vec<Block>,  // Vector of blocks
    pub difficulty: usize,  // Mining difficulty (number of leading zeros)
}
```

## Algorithms

### Proof of Work

The Proof of Work algorithm requires finding a nonce that, when combined with the block data and hashed, produces a hash with a specific number of leading zeros:

1. Start with nonce = 0
2. Calculate hash of block data + nonce
3. Check if hash has required number of leading zeros
4. If not, increment nonce and repeat
5. If yes, block is mined

This process makes it computationally expensive to add blocks, preventing tampering.

### Hashing

All hashing in the system uses SHA-256:

1. **Block Hash**: Hash of concatenated block fields:
   ```
   hash = SHA256(id + index + timestamp + biometric_data_hash + previous_hash + nonce)
   ```

2. **Biometric Data Hash**: Hash of the raw biometric data:
   ```
   biometric_data_hash = SHA256(raw_biometric_data)
   ```

### Blockchain Validation

The validation algorithm ensures the integrity of the blockchain:

1. For each block (starting from index 1):
   - Verify current block's hash matches its calculated hash
   - Verify current block's previous_hash matches the previous block's hash
2. If all checks pass, the blockchain is valid

## API Endpoints

| Method | Endpoint           | Description                     |
|--------|--------------------|---------------------------------|
| POST   | /api/add_block     | Add a new biometric data block  |
| GET    | /api/info          | Get blockchain information      |
| GET    | /api/blockchain    | Get the entire blockchain       |
| GET    | /api/validate      | Validate the blockchain         |
| POST   | /api/save          | Save blockchain to file         |
| GET    | /                  | Serve the web interface         |

## Getting Started

### Prerequisites

- Rust toolchain (edition 2024)
- Cargo package manager

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust_blockchain
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Running the Application

1. Run the application:
   ```bash
   cargo run
   ```

2. When prompted, choose the web interface (option 1)

3. Open your browser and navigate to `http://localhost:8080`

## Usage

### Web Interface

1. Access the web interface at `http://localhost:8080`
2. Enter biometric data (e.g., fingerprint hash) in the text area
3. Click "Add Block" to mine and add a new block
4. Use the action buttons to validate, save, or view the blockchain

### Command Line Interface

1. When starting the application, choose option 2 for CLI
2. Use the menu to:
   - Add biometric blocks
   - View the blockchain
   - Validate the blockchain
   - View the latest block
   - Save the blockchain

## Project Structure

```
src/
├── block.rs          // Block data structure and methods
├── blockchain.rs     // Blockchain data structure and methods
├── handlers.rs       // HTTP request handlers (API endpoints)
├── main.rs           // Application entry point and CLI
└── static/           // Static files (if any)
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
