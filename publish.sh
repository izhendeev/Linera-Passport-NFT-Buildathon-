#!/bin/bash
set -e

echo "=== Passport NFT - Publish Module (Bytecode) ==="
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

if ! command -v linera &> /dev/null; then
    echo "Error: linera not found in PATH"
    exit 1
fi

echo "Publishing module to Linera..."
echo ""

linera publish-module "$CONTRACT_WASM" "$SERVICE_WASM"

echo ""
echo "================================================"
echo "⚠️  IMPORTANT: SAVE THE MODULE_ID (BYTECODE_ID)"
echo "================================================"
