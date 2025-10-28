#!/bin/bash
set -e

echo "=== Starting Passport NFT Agent ==="
echo ""

# Check if indexer is running
if ! curl -s http://localhost:8081/operations > /dev/null 2>&1; then
    echo "⚠️  Warning: Indexer not accessible at http://localhost:8081/operations"
    echo "   Please start the indexer first:"
    echo "   cd ~/linera-protocol/linera-indexer/example && ./run_grpc_indexer.sh"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if GraphQL service is running
if ! curl -s http://localhost:8080 > /dev/null 2>&1; then
    echo "Error: Linera service not running on port 8080"
    echo "Please start it first: linera service --port 8080"
    exit 1
fi

cd ~/linera-protocol/examples/passport-nft-agent

echo "Building agent..."
cargo build --release

echo ""
echo "Starting agent with config.toml..."
echo ""

export PASSPORT_AGENT_CONFIG="$(pwd)/config.toml"
export RUST_LOG=info
~/linera-protocol/examples/target/release/passport_oracle
