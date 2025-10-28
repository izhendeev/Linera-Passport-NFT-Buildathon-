#!/bin/bash

# ========================================
# Passport NFT Frontend - Start Script
# ========================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AGENT_DIR="$SCRIPT_DIR/../passport-nft-agent"
FRONTEND_FILE="$SCRIPT_DIR/frontend.html"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Passport NFT Frontend - Startup${NC}"
echo -e "${BLUE}========================================${NC}\n"

# ========================================
# 1. Check if Linera Service is running
# ========================================
echo -e "${YELLOW}[1/4] Checking Linera Service...${NC}"

if curl -s http://localhost:8080 > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Linera Service is running on port 8080${NC}\n"
else
    echo -e "${RED}✗ Linera Service is not running!${NC}"
    echo -e "${YELLOW}Start it in a separate terminal:${NC}"
    echo -e "  ${BLUE}linera service --port 8080${NC}\n"
    exit 1
fi

# ========================================
# 2. Check if Quick Score API is running
# ========================================
echo -e "${YELLOW}[2/4] Checking Quick Score API...${NC}"

if curl -s http://localhost:8001 > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Quick Score API is running on port 8001${NC}\n"
else
    echo -e "${YELLOW}⚠ Quick Score API is not running${NC}"
    echo -e "${YELLOW}Starting Quick Score API...${NC}"

    if [ -d "$AGENT_DIR" ]; then
        cd "$AGENT_DIR"

        # Check if binary exists
        if [ -f "target/release/quick_score_api" ]; then
            echo -e "${GREEN}Using pre-built binary${NC}"
            nohup ./target/release/quick_score_api > /tmp/quick_score_api.log 2>&1 &
        else
            echo -e "${YELLOW}Building and starting Quick Score API (this may take a minute)...${NC}"
            cargo build --bin quick_score_api --release > /dev/null 2>&1
            nohup ./target/release/quick_score_api > /tmp/quick_score_api.log 2>&1 &
        fi

        SCORE_API_PID=$!
        echo -e "${GREEN}✓ Quick Score API started (PID: $SCORE_API_PID)${NC}"
        echo -e "${BLUE}  Log: tail -f /tmp/quick_score_api.log${NC}\n"

        # Wait for API to start
        sleep 2

        cd "$SCRIPT_DIR"
    else
        echo -e "${RED}✗ Agent directory not found: $AGENT_DIR${NC}"
        echo -e "${YELLOW}Start it manually:${NC}"
        echo -e "  ${BLUE}cd examples/passport-nft-agent${NC}"
        echo -e "  ${BLUE}cargo run --bin quick_score_api --release${NC}\n"
    fi
fi

# ========================================
# 3. Load configuration
# ========================================
echo -e "${YELLOW}[3/4] Loading configuration...${NC}"

if [ -f "$SCRIPT_DIR/.env.deployment" ]; then
    source "$SCRIPT_DIR/.env.deployment"
    echo -e "${GREEN}✓ Configuration loaded${NC}"
    echo -e "${BLUE}  Application ID: $APPLICATION_ID${NC}"
    echo -e "${BLUE}  Chain ID: $CHAIN_ID${NC}"
    echo -e "${BLUE}  Admin Account: $ADMIN_ACCOUNT${NC}\n"
else
    echo -e "${YELLOW}⚠ .env.deployment not found${NC}"
    APPLICATION_ID="b139121af898c9bbb6dca05a7efde3ef396eeefe271650bb5659692613d4d463"
    CHAIN_ID="f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe"
    echo -e "${BLUE}  Using default values${NC}\n"
fi

# ========================================
# 4. Start HTTP server for frontend
# ========================================
echo -e "${YELLOW}[4/4] Starting HTTP server for frontend...${NC}"

PORT=3000

# Check if port is already in use
if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo -e "${YELLOW}⚠ Port $PORT is already in use${NC}"
    PORT=3001
    echo -e "${BLUE}  Trying port $PORT instead${NC}"
fi

cd "$SCRIPT_DIR"

# Start Python HTTP server in background
nohup python3 -m http.server $PORT > /tmp/passport_frontend.log 2>&1 &
HTTP_SERVER_PID=$!

echo -e "${GREEN}✓ HTTP server started on port $PORT (PID: $HTTP_SERVER_PID)${NC}\n"

# Wait for server to start
sleep 1

# ========================================
# 5. Generate URL
# ========================================
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ All services are running!${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Get owner address from wallet
OWNER=""
WALLET_FILE="$HOME/.config/linera/wallet.json"

if [ -f "$WALLET_FILE" ]; then
    OWNER=$(cat "$WALLET_FILE" | grep -o '"owner":"[^"]*"' | head -1 | cut -d'"' -f4)
fi

if [ -z "$OWNER" ]; then
    OWNER="YOUR_OWNER_ADDRESS"
fi

FRONTEND_URL="http://localhost:$PORT/frontend.html?owner=$OWNER&chainId=$CHAIN_ID&app=$APPLICATION_ID"

echo -e "${GREEN}Open this URL in your browser:${NC}\n"
echo -e "${BLUE}$FRONTEND_URL${NC}\n"

echo -e "${YELLOW}If the owner address is wrong, replace it with yours:${NC}"
echo -e "${BLUE}http://localhost:$PORT/frontend.html?owner=YOUR_ADDRESS&chainId=$CHAIN_ID&app=$APPLICATION_ID${NC}\n"

# Try to open browser automatically
if command -v xdg-open > /dev/null; then
    echo -e "${YELLOW}Opening browser...${NC}"
    xdg-open "$FRONTEND_URL" 2>/dev/null &
elif command -v firefox > /dev/null; then
    echo -e "${YELLOW}Opening Firefox...${NC}"
    firefox "$FRONTEND_URL" 2>/dev/null &
elif command -v google-chrome > /dev/null; then
    echo -e "${YELLOW}Opening Chrome...${NC}"
    google-chrome "$FRONTEND_URL" 2>/dev/null &
fi

echo -e "\n${YELLOW}To stop the services:${NC}"
echo -e "  ${BLUE}kill $HTTP_SERVER_PID${NC} (HTTP server)"
if [ ! -z "$SCORE_API_PID" ]; then
    echo -e "  ${BLUE}kill $SCORE_API_PID${NC} (Quick Score API)"
fi
echo -e "  ${BLUE}pkill -f 'linera service'${NC} (Linera Service - if needed)"

echo -e "\n${YELLOW}Logs:${NC}"
echo -e "  ${BLUE}tail -f /tmp/passport_frontend.log${NC} (HTTP server)"
echo -e "  ${BLUE}tail -f /tmp/quick_score_api.log${NC} (Quick Score API)"

echo -e "\n${GREEN}Happy building! 🚀${NC}\n"
