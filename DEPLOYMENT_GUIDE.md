# Passport NFT - Deployment Guide

## Deployment Information

**Application ID**: `6b78da405d79174f0bac8d95416ed52c1b594f0f0a6b2d2d704dac6acd09ac37`  
**Chain ID**: `f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe`  
**Admin Account**: `User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74`

## Quick Start

### 1. Start Linera Service (Required)

```bash
linera service --port 8080
```

**Your Application GraphQL Endpoint**:
```
http://localhost:8080/chains/f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe/applications/6b78da405d79174f0bac8d95416ed52c1b594f0f0a6b2d2d704dac6acd09ac37
```

### 2. Start Frontend

```bash
cd ~/linera-protocol/examples/passport-nft/web-frontend
npm install  # first time
npm run dev
```

Access at: **http://localhost:3000**

### 3. Start Oracle Agent (Optional - for auto-scoring)

```bash
cd ~/linera-protocol/examples/passport-nft-agent
bash run-agent.sh
```

### 4. Start Indexer (Optional - required by oracle)

```bash
cd ~/linera-protocol/linera-indexer/example
./run_grpc_indexer.sh
```

## Sharing Your Application

**GraphQL Endpoint (full URL)**:
```
http://localhost:8080/chains/f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe/applications/6b78da405d79174f0bac8d95416ed52c1b594f0f0a6b2d2d704dac6acd09ac37
```

**Application ID**: `6b78da405d79174f0bac8d95416ed52c1b594f0f0a6b2d2d704dac6acd09ac37`  
**Chain ID**: `f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe`

## Configuration Files Created

- `examples/passport-nft/.env.deployment` - deployment info
- `examples/passport-nft/web-frontend/.env.local` - frontend config
- `examples/passport-nft-agent/config.toml` - oracle config
- `examples/passport-nft-agent/config/achievements.json` - achievement rules

## Notes

- Admin is already set as oracle (done during deployment)
- Oracle format: `0x<hash>` (not `User:<hash>`)
- Each owner can only have 1 passport (soulbound)
