# Linera Passport NFT - Buildathon Submission

## What it does

Linera Passport NFT is a soulbound reputation system that issues non-transferable identity NFTs on Linera microchains. Each user gets one passport that lives on their own microchain and tracks blockchain activity to build a verifiable reputation score.

**Key features:**
- Smart contract that mints soulbound NFT passports
- Oracle agent that reads user activity from Linera Indexer
- Dynamic scoring based on transactions, app usage, and token transfers
- Configurable achievement system
- Web frontend for minting and viewing passports
- Cross-chain reputation aggregation

## The problem it solves

**Identity and reputation are fragmented across Web3.** Users can't prove their on-chain history. DeFi protocols can't assess creditworthiness. DAOs can't weight votes by contribution. Games can't recognize achievements from other platforms.

**Traditional solutions don't scale.** Storing reputation in global state creates bottlenecks where every update competes for the same resources.

**Linera Passport NFT solves this:**
- Soulbound identity NFTs that can't be sold or transferred
- Verifiable on-chain activity tracking via Linera Indexer
- Each passport on its own microchain (no contention, infinite scalability)
- Cross-chain aggregation for unified reputation
- Portable credentials for DeFi, DAOs, games, and social apps

## Challenges I ran into

**1. Configuring oracle agent to read correct data from indexer**
The agent needed to parse different operation types (transfers, app creations, transactions) and aggregate them correctly to calculate scores. Had to filter indexer data by owner address and handle edge cases where activity data was incomplete or malformed.

**2. Applications failing to deploy due to system time desynchronization**
Linera requires synchronized system time for block validation. When local time drifted from network time, contract deployments failed with cryptic errors. Fixed by syncing system clock with NTP servers.

**3. Cross-chain data aggregation complexity**
Implementing cross-chain reputation aggregation required querying multiple microchains and merging activity data without double-counting. Had to design efficient algorithms to track which chains were already processed and handle cases where chains had overlapping operations.

## Technologies I used

- **Linera SDK** - Smart contract framework
- **Rust + WebAssembly** - Contract and oracle implementation
- **Linera Indexer** - Blockchain activity data source
- **GraphQL** - Service layer for queries and mutations
- **Axum** - HTTP server for Quick Score API
- **Tower-HTTP** - CORS middleware
- **Vanilla JavaScript** - Zero-dependency frontend

## How we built it

**Smart Contract:**
- Soulbound NFT logic with one-per-address restriction
- State management for passports, scores, and achievements
- GraphQL service for minting and querying
- Admin-only `updateAchievements` mutation

**Oracle Agent:**
- HTTP client to query Linera Indexer
- Scoring logic that reads transactions, transfers, app creations
- Two modes: Quick Score API (read-only) and Full Oracle (writes to blockchain)
- Configurable achievement rules via JSON

**Frontend:**
- Single HTML file with zero dependencies
- GraphQL integration for minting and queries
- Quick Score API integration for real-time scoring
- Auto-refresh every 15 seconds

**Architecture:**
- Each passport on separate microchain for parallel execution
- Oracle reads from indexer for efficiency
- Dual scoring: instant feedback (API) + verified state (blockchain)

## What we learned

**Microchains eliminate contention.** Each passport on its own microchain means 1 million users = 1 million parallel chains. No global state bottlenecks.

**Linera Indexer is crucial for oracles.** Instead of querying individual chains, the indexer provides unified view of all activity. This makes cross-chain aggregation practical.

**Soulbound tokens require explicit restrictions.** Non-transferable NFTs need careful contract design with ownership checks and no transfer functions.

**Configuration flexibility is essential.** JSON-based achievement rules let us add new achievements without redeploying contracts.

**Dual state design improves UX.** Quick Score API gives instant feedback while Full Oracle ensures blockchain state is eventually consistent.

## What's next for Linera Passport NFT

**Production-Ready:**
- Wallet integration in frontend
- Enhanced UI with responsive design
- Oracle reliability improvements
- Automated testing suite

**AI-Powered Scoring:**
- LLM integration for behavior analysis
- Pattern detection for fraud and Sybil attacks
- Semantic achievement detection

**Decentralized Oracle Network:**
- Multiple oracle nodes with consensus
- TEE for secure AI execution
- Stake-based participation with slashing

**Ecosystem Expansion:**
- Dynamic NFT images based on score
- Leaderboards and social features
- Third-party integrations (DeFi credit scoring, DAO voting, game achievements)
- Mobile app
- Public API for external projects

**Vision:** A universal reputation layer for Web3 where your Linera Passport is recognized across all apps, protocols, and chains.
