# Linera Passport NFT

**Soulbound Reputation System on Linera Microchains**

Decentralized identity and reputation platform that issues non-transferable NFT passports and tracks user activity across Linera blockchain.

## Overview

Each passport is a soulbound NFT that lives on the owner's microchain. User activity is tracked via an oracle agent that reads data from Linera Indexer and calculates reputation scores based on configurable achievement rules.

**Key Features:**
- Soulbound NFTs (one per address, non-transferable)
- Oracle agent that reads blockchain data from Linera Indexer
- Dynamic scoring based on real activity
- Configurable achievement system
- Cross-chain reputation aggregation

## Oracle Agent

The oracle agent reads user activity data from Linera Indexer and calculates reputation scores.

**Two operational modes:**

1. **Quick Score API** - Read-only HTTP API that instantly calculates scores from indexer data
2. **Full Oracle** - Polls indexer and writes verified scores to blockchain via GraphQL mutations

**What the agent reads from Linera Indexer:**
- Total transactions
- Transfer volume
- Application creations
- Application usage
- Cross-chain activity

## Quick Start

### 1. Deploy Contract

```bash
# Build the contract
cargo build --release --target wasm32-unknown-unknown

# Publish and create application
linera publish-and-create \
  target/wasm32-unknown-unknown/release/passport_nft_contract.wasm \
  target/wasm32-unknown-unknown/release/passport_nft_service.wasm \
  --json-parameters '{"admin": "User:YOUR_ADMIN_ADDRESS"}'

# Save the application ID from the output
```

### 2. Start Services

```bash
# Terminal 1: Linera Service
linera service --port 8080

# Terminal 2: Linera Indexer
linera-indexer --port 8000

# Terminal 3: Oracle Agent (Quick Score API)
cd ../passport-nft-agent
cargo run --bin quick_score_api --release

# Terminal 4: Frontend
cd ../passport-nft
python3 -m http.server 3000
```

### 3. Open Frontend

```bash
http://localhost:3000/frontend.html?owner=User:YOUR_ADDRESS
```

## Scoring System

**Base Score**: 1 point per 10 transactions

**Achievements:**
- Early Adopter: +50 points (first 1000 users)
- Active User: +25 points (10+ transactions)
- Power User: +75 points (50+ transactions)
- Whale: +100 points (1000+ tokens transferred)
- Developer: +100 points (3+ apps used)
- App Creator: +150 points (deployed an app)

## Configuration

Update `passport-nft-agent/config.toml`:

```toml
application_id = "YOUR_APP_ID"
graphql_endpoint = "http://localhost:8080/chains/CHAIN_ID/applications/APP_ID"
indexer_endpoint = "http://localhost:8000/operations"
wallet_path = "/home/user/.config/linera/wallet.json"
```

## Project Structure

```
passport-nft/
├── src/              # Smart contract
├── frontend.html     # Web UI
└── Cargo.toml

passport-nft-agent/
├── src/bin/
│   ├── passport_oracle.rs    # Full oracle (writes to blockchain)
│   └── quick_score_api.rs    # API (reads from indexer)
├── config/
│   └── achievements.json     # Achievement rules
└── config.toml
```

## Why Microchains?

- Each passport on separate microchain = parallel execution
- No global state bottlenecks
- Linear scalability
- Cross-chain aggregation

## Development Status

- ✅ Smart contract with soulbound NFTs
- ✅ Oracle agent reading from Linera Indexer
- ✅ Configurable achievement system
- ✅ Basic web frontend
- 🚧 Enhanced frontend (in progress)
- 🚧 Oracle improvements (in progress)

## Resources

- [Linera Docs](https://docs.linera.io)
- [Buildathon](https://linera.io/buildathon)

## License

Apache-2.0