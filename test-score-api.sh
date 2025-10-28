#!/bin/bash

# ========================================
# Quick Score API - Test Script
# ========================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Quick Score API - Test${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Load config
if [ -f ".env.deployment" ]; then
    source .env.deployment
    OWNER="$ADMIN_ACCOUNT"
else
    OWNER="${1:-User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74}"
fi

echo -e "${YELLOW}Testing Quick Score API...${NC}"
echo -e "${BLUE}Owner: $OWNER${NC}\n"

# Test API
SCORE_API_URL="http://localhost:8001/quick-score?owner=$OWNER"

echo -e "${YELLOW}Request:${NC}"
echo -e "${BLUE}$SCORE_API_URL${NC}\n"

echo -e "${YELLOW}Response:${NC}"

if curl -s "$SCORE_API_URL" 2>/dev/null; then
    echo -e "\n${GREEN}✓ Quick Score API is working!${NC}\n"
else
    echo -e "\n${RED}✗ Quick Score API failed!${NC}"
    echo -e "${YELLOW}Make sure the API is running:${NC}"
    echo -e "  ${BLUE}cd examples/passport-nft-agent${NC}"
    echo -e "  ${BLUE}cargo run --bin quick_score_api --release${NC}\n"
    exit 1
fi

# Pretty print with jq if available
if command -v jq > /dev/null; then
    echo -e "${YELLOW}Formatted response:${NC}"
    curl -s "$SCORE_API_URL" | jq .
fi