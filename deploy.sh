#!/bin/bash
set -e

echo "=== Passport NFT - Deploy (Publish & Create) ==="
echo ""

CONTRACT_WASM="$HOME/linera-protocol/examples/target/wasm32-unknown-unknown/release/passport_nft_contract.wasm"
SERVICE_WASM="$HOME/linera-protocol/examples/target/wasm32-unknown-unknown/release/passport_nft_service.wasm"

if [ ! -f "$CONTRACT_WASM" ]; then
    echo "Error: Contract WASM not found"
    exit 1
fi

if [ ! -f "$SERVICE_WASM" ]; then
    echo "Error: Service WASM not found"
    exit 1
fi

echo "✓ Contract WASM: $(du -h $CONTRACT_WASM | cut -f1)"
echo "✓ Service WASM:  $(du -h $SERVICE_WASM | cut -f1)"
echo ""

echo "Deploying to Linera (publish + create)..."
echo ""

linera publish-and-create "$CONTRACT_WASM" "$SERVICE_WASM"

echo ""
echo "================================================"
echo "✅ APPLICATION DEPLOYED!"
echo "⚠️  SAVE THE APPLICATION_ID AND CHAIN_ID"
echo "================================================"
