# Project Title

Patent Filing – A Soroban Smart Contract for Timestamped Patent Registration on Stellar

## Project Vision

This project demonstrates a **decentralized patent filing system** built on Stellar using Soroban smart contracts. It provides:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage (patent records with timestamps)
- How to handle user authentication and authorization in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to enable inventors to file and verify patents with immutable timestamps on the blockchain.

---

## Description

A Soroban smart contract dApp that allows inventors to file patents on Stellar Testnet with timestamps. Each patent record contains the inventor's address, patent ID, title, description, and filing timestamp. The system supports patent transfers and inventor verification.

---

## Features

### 1. Patent Filing
- File a new patent with unique ID
- Stores inventor address, title, description, and timestamp
- Timestamp from Stellar ledger (immutable)

### 2. Patent Retrieval
- Get complete patent details by patent ID
- Returns (inventor, title, description, filing_timestamp)

### 3. Patent Transfer
- Transfer patent ownership to a new inventor
- Only current inventor can authorize transfer
- Maintains original filing timestamp

### 4. Inventor Verification
- Verify if an address is the inventor of a patent
- Returns boolean result

### 5. On-chain Transparency
- All patent records stored permanently on blockchain
- Anyone can query patent information and verify inventors

---

## Contract Functions

- `init()` - Initialize the contract
- `file_patent(inventor, patent_id, title, description)` - File a new patent
- `get_patent(patent_id)` - Get patent details
- `transfer_patent(patent_id, new_inventor)` - Transfer patent ownership
- `verify_inventor(patent_id, address)` - Verify inventor

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CBY45JEGLLZ4TKUUZTZXUCCV6VLQHUUHIU6ACQC7BQDQUIXCRYVTLNRK](https://stellar.expert/explorer/testnet/tx/e3bf363906a35ca48941ae13b7f9dc566a2e8b3436089f05bb4a8349291005c1)

![screenshot](https://i.ibb.co/qYBWvnc4/image.png)

---


## Future Scopes

### 1. Patent Search
- Add functionality to search patents by title keywords or inventor

### 2. Patent Metadata
- Support for additional metadata like category, citations, or related patents

### 3. Admin Interface
- Add a frontend dashboard to browse and manage patents

### 4. Royalty System
- Implement royalty payments when patents are licensed or sold

### 5. Multi-signature Transfer
- Require multiple inventor signatures for high-value patent transfers

### 6. IPFS Integration
- Store large patent documents on IPFS with hash stored on-chain

---

## Profile

- **Name:** :phohuyhieu6
