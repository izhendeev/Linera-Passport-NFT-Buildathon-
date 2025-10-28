# Linera Passport NFT - Web Frontend

Beautiful, modern frontend for the Linera Passport NFT system with AI scoring capabilities.

## Features

- 🎨 **Frosted Glass UI** - iOS-inspired design with 3D parallax effects
- 👛 **Wallet Integration** - Connect Linera wallet to view your passport
- 🏆 **Live Leaderboard** - Real-time rankings from blockchain data
- ✨ **AI Scoring** - AI-powered reputation analysis button
- 📊 **GraphQL Integration** - Real-time data from Linera blockchain
- 🔄 **Auto-refresh** - Passport data updates every 5 seconds

## Quick Start

### Prerequisites

- Node.js 18+ installed
- Linera service running on `localhost:8080`
- Passport NFT contract deployed
- GraphQL endpoint reachable (e.g. open `http://localhost:8080/chains/{CHAIN_ID}/applications/{APP_ID}` in GraphiQL)

### Installation

```bash
# Install dependencies
npm install --legacy-peer-deps

# Set environment variables
cp .env.local.example .env.local
# Edit .env.local with your APPLICATION_ID and CHAIN_ID

# Start development server
npm run dev
```

Frontend will be available at http://localhost:3000

## Environment Variables

Create `.env.local` with:

```env
NEXT_PUBLIC_GRAPHQL_ENDPOINT=http://localhost:8080/chains/{CHAIN_ID}/applications/{APP_ID}
NEXT_PUBLIC_CHAIN_ID={your-chain-id}
NEXT_PUBLIC_APPLICATION_ID={your-application-id}
```

## Usage

1. **Connect Wallet** - Click "Connect Wallet" button (simulated for demo)
2. **Mint Passport** - If you don't have a passport, click "Mint Passport"
3. **View Data** - Your passport card shows token ID, score, achievements
4. **Update** - Click "Update Passport" to refresh data from Oracle Agent
5. **AI Score** - Click "AI Score" button for enhanced reputation analysis

## Architecture

```
┌─────────────────────────────────────┐
│  Next.js 16 + React 19              │
│  Tailwind CSS 4 + shadcn/ui         │
│  Michroma Font (Google Fonts)       │
└──────────────┬──────────────────────┘
               ↓
┌──────────────────────────────────────┐
│  Apollo Client (GraphQL)             │
│  - Auto-refresh every 5s             │
│  - Real-time passport data           │
│  - Leaderboard from all passports    │
└──────────────┬───────────────────────┘
               ↓
┌──────────────────────────────────────┐
│  Linera Service (port 8080)          │
│  GraphQL endpoint                    │
│  /chains/{chain}/applications/{app}  │
└──────────────────────────────────────┘
```

## Components

- **PassportCard** - 3D animated card with parallax tilt
- **WalletButton** - Wallet connection dropdown
- **Leaderboard** - Live rankings sidebar
- **FloatingParticles** - Animated background particles
- **Button** - Gradient buttons with hover effects

## GraphQL Queries

The frontend communicates exclusively with the Linera GraphQL service exposed by `linera service`
(`http://localhost:8080`). All reads and writes (minting, updating achievements, leaderboard data)
use the same GraphQL operations you can try from GraphiQL.

### Get All Passports
```graphql
query GetAllPassports {
  allPassports {
    tokenId { id }
    owner
    ownerChain
    score
    achievements
  }
}
```

### Mint Passport
```graphql
mutation MintPassport {
  mint
}
```

## Styling

Uses **Michroma** font for futuristic look:
- Titles: Bold, large sizes
- Labels: Light weight, uppercase, tracking-widest
- Details: Regular weight

Color scheme:
- Background: stone-300 → stone-200 → stone-100 gradient
- Card: Frosted glass with red/blue tints
- Primary button: stone-700 → stone-900
- AI button: purple-600 → indigo-700

## Development

```bash
# Run dev server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint code
npm run lint
```

## Troubleshooting

### GraphQL Connection Error
- Ensure Linera service is running: `linera service --port 8080`
- Check `.env.local` has correct endpoint URLs

### Passport Not Showing
- Verify wallet address matches `owner` in contract
- Check GraphQL endpoint returns data: `curl http://localhost:8080/chains/.../applications/.../graphql -d '{"query": "{allPassports{owner}}"}'`

### Leaderboard Empty
- Ensure multiple passports exist in contract
- Check `allPassports` query returns >1 result

## Tech Stack

- **Framework**: Next.js 16 (App Router)
- **React**: 19.2.0
- **Styling**: Tailwind CSS 4 + shadcn/ui
- **GraphQL**: Apollo Client 3.8
- **Icons**: Lucide React
- **Font**: Michroma (Google Fonts)

## License

Apache-2.0 (same as Linera Protocol)
