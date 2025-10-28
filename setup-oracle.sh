#!/bin/bash
set -e

echo "=== Adding Admin as Oracle ==="
echo ""

# From deployment
APPLICATION_ID="6b78da405d79174f0bac8d95416ed52c1b594f0f0a6b2d2d704dac6acd09ac37"
CHAIN_ID="f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe"
ADMIN_ACCOUNT="a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74"

GRAPHQL_ENDPOINT="http://localhost:8080/chains/${CHAIN_ID}/applications/${APPLICATION_ID}"

echo "GraphQL Endpoint: $GRAPHQL_ENDPOINT"
echo "Admin Account: $ADMIN_ACCOUNT"
echo ""

# Add admin as oracle
echo "Sending addOracle mutation..."
curl -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d "{
    \"query\": \"mutation { addOracle(oracle: \\\"$ADMIN_ACCOUNT\\\") }\"
  }" | jq .

echo ""
echo "✓ Oracle setup complete"
