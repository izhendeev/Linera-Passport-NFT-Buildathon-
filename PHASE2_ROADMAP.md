# Phase 2: Decentralized AI Oracle Network

## Vision

Transform the current **centralized oracle** into a **decentralized AI-powered oracle network** with TEE (Trusted Execution Environment) integration, making Passport NFT the first production-grade decentralized reputation system on Linera.

## Current State (Phase 1)

```
Single Oracle Agent → Linera Indexer → Calculate Score → Submit to Contract
```

**Limitations**:
- Single point of failure
- Trust in one oracle operator
- No AI-powered insights
- Manual scoring rules

## Target State (Phase 2)

```
┌───────────────────────────────────────────────────────┐
│              Oracle TEE Network                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐            │
│  │ Oracle 1 │  │ Oracle 2 │  │ Oracle 3 │            │
│  │ (AI+TEE) │  │ (AI+TEE) │  │ (AI+TEE) │            │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘            │
└───────┼─────────────┼─────────────┼───────────────────┘
        │             │             │
        └─────────────┴─────────────┘
                      ↓
        ┌─────────────────────────┐
        │  Consensus Layer        │
        │  (66%+ agreement)       │
        └─────────────┬───────────┘
                      ↓
        ┌─────────────────────────┐
        │  Passport NFT Contract  │
        │  (Multi-oracle support) │
        └─────────────────────────┘
```

## Architecture Components

### 1. Multi-Oracle Infrastructure

#### Oracle Registration Contract
```rust
pub struct OracleRegistry {
    /// All registered oracles
    pub oracles: MapView<OracleId, OracleNode>,
    /// Minimum stake required
    pub min_stake: Amount,
    /// Active oracle set
    pub active_oracles: SetView<OracleId>,
}

pub struct OracleNode {
    pub id: OracleId,
    pub operator: AccountOwner,
    pub stake: Amount,
    pub reputation_score: u64,
    pub responses_submitted: u64,
    pub slashed_count: u32,
    pub registration_block: BlockHeight,
}
```

#### Operations
- `RegisterOracle { stake: Amount }` - Join oracle network
- `StakeMore { amount: Amount }` - Increase stake
- `Unstake { amount: Amount }` - Withdraw stake (7-day delay)
- `SlashOracle { oracle_id: OracleId, reason: String }` - Penalize misbehavior

### 2. Consensus Mechanism

#### Oracle Response Collection
```rust
pub struct OracleResponseSet {
    pub token_id: TokenId,
    pub responses: Vec<OracleResponse>,
    pub submission_deadline: Timestamp,
}

pub struct OracleResponse {
    pub oracle_id: OracleId,
    pub score: u64,
    pub achievements: Vec<String>,
    pub signature: OracleSignature,
    pub submitted_at: Timestamp,
}
```

#### Consensus Algorithm
```rust
impl OracleResponseSet {
    /// Check if consensus reached (66%+ agreement)
    pub fn check_consensus(&self) -> Option<ConsensusResult> {
        let threshold = (self.responses.len() * 2) / 3;

        // Group by score
        let mut score_counts: HashMap<u64, Vec<OracleId>> = HashMap::new();
        for response in &self.responses {
            score_counts.entry(response.score)
                .or_insert_with(Vec::new)
                .push(response.oracle_id);
        }

        // Find majority
        for (score, oracles) in score_counts {
            if oracles.len() >= threshold {
                return Some(ConsensusResult {
                    agreed_score: score,
                    supporting_oracles: oracles,
                    total_oracles: self.responses.len(),
                });
            }
        }

        None
    }

    /// Identify outlier oracles for potential slashing
    pub fn find_outliers(&self, consensus: &ConsensusResult) -> Vec<OracleId> {
        self.responses
            .iter()
            .filter(|r| r.score != consensus.agreed_score)
            .map(|r| r.oracle_id)
            .collect()
    }
}
```

### 3. TEE (Trusted Execution Environment) Integration

#### Intel SGX Architecture
```
┌─────────────────────────────────────────┐
│        TEE Enclave (Intel SGX)          │
│  ┌───────────────────────────────────┐  │
│  │  AI Oracle Logic                  │  │
│  │  - Load user activity             │  │
│  │  - Run LLM inference              │  │
│  │  - Calculate score                │  │
│  │  - Sign result                    │  │
│  └───────────────────────────────────┘  │
│                                          │
│  ┌───────────────────────────────────┐  │
│  │  Private Keys (sealed)            │  │
│  │  - Oracle signing key             │  │
│  │  - OpenAI API key                 │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
         ↓ Attestation Quote
┌─────────────────────────────────────────┐
│     Linera Validators                    │
│  Verify SGX quote + signature           │
└─────────────────────────────────────────┘
```

#### Implementation
```rust
// TEE attestation structure
pub struct TeeAttestation {
    pub quote: Vec<u8>,          // SGX quote
    pub enclave_hash: Vec<u8>,   // Measurement
    pub public_key: PublicKey,   // Oracle's public key
}

// Response with TEE proof
pub struct TeeOracleResponse {
    pub response: OracleResponse,
    pub attestation: TeeAttestation,
    pub proof: TeeProof,
}

impl TeeOracleResponse {
    /// Verify that response came from genuine SGX enclave
    pub fn verify(&self) -> Result<bool> {
        // 1. Verify SGX quote signature
        verify_sgx_quote(&self.attestation.quote)?;

        // 2. Check enclave measurement matches approved oracle binary
        if self.attestation.enclave_hash != APPROVED_ORACLE_HASH {
            return Ok(false);
        }

        // 3. Verify response signature
        self.attestation.public_key.verify(
            &self.response.serialize(),
            &self.response.signature
        )
    }
}
```

### 4. AI-Powered Scoring

#### Current: Rule-Based
```json
{
  "active_user": {
    "condition": { "total_transactions": { "min_count": 10 } },
    "points": 25
  }
}
```

#### Phase 2: AI-Enhanced
```rust
pub struct AiScoringEngine {
    model: OpenAiClient,
    context_window: usize,
}

impl AiScoringEngine {
    /// Analyze user behavior with LLM
    pub async fn analyze_user(&self, activity: &[ActivityEvent]) -> AiInsights {
        let prompt = format!(
            "Analyze this blockchain user's activity and provide insights:

            Total transactions: {}
            Apps used: {:?}
            Transfer volume: {}
            Time span: {} days

            Identify:
            1. User type (DeFi trader, NFT collector, developer, etc.)
            2. Skill level (beginner, intermediate, expert)
            3. Reputation score (0-1000)
            4. Potential fraud indicators
            5. Custom achievements deserved",
            activity.len(),
            extract_apps(activity),
            calculate_volume(activity),
            calculate_days(activity)
        );

        let response = self.model.complete(&prompt).await?;
        parse_ai_insights(response)
    }

    /// Fraud detection with AI
    pub async fn detect_fraud(&self, activity: &[ActivityEvent]) -> FraudScore {
        // Detect patterns:
        // - Sybil attacks (coordinated accounts)
        // - Wash trading (circular transfers)
        // - Bot-like behavior (regular intervals)

        let features = extract_fraud_features(activity);
        self.model.classify_fraud(features).await
    }
}
```

#### AI Achievements Examples
```rust
pub enum AiAchievement {
    /// "This user is a sophisticated DeFi trader"
    DefiExpert { confidence: f64, evidence: String },

    /// "Early adopter of 5+ protocols"
    EarlyAdopter { protocols: Vec<String> },

    /// "Consistent daily activity for 30+ days"
    ConsistentUser { streak_days: u32 },

    /// "Bridge between multiple communities"
    CommunityConnector { chains: Vec<ChainId> },
}
```

## Implementation Timeline

### Month 1: Multi-Oracle Foundation
**Week 1-2**: Oracle Registry Contract
- [ ] Design oracle registration data structures
- [ ] Implement stake/unstake logic
- [ ] Add slashing mechanism
- [ ] Write tests for edge cases

**Week 3-4**: Response Collection & Consensus
- [ ] Build response aggregation system
- [ ] Implement 66%+ consensus algorithm
- [ ] Add timeout handling for slow oracles
- [ ] Test with 3-10 simulated oracles

### Month 2: TEE Integration
**Week 5-6**: Intel SGX Setup
- [ ] Set up SGX development environment
- [ ] Port oracle logic to SGX enclave
- [ ] Implement quote generation
- [ ] Test attestation verification

**Week 7-8**: Linera Integration
- [ ] Add SGX attestation verification to contract
- [ ] Store approved enclave measurements
- [ ] Implement proof validation on-chain
- [ ] Security audit of TEE integration

### Month 3: AI Scoring
**Week 9-10**: LLM Integration
- [ ] Fine-tune GPT-4 on blockchain reputation data
- [ ] Build prompt engineering for user analysis
- [ ] Implement fraud detection ML model
- [ ] Run in TEE with OpenAI API

**Week 11-12**: Testing & Launch
- [ ] End-to-end testing with real Linera data
- [ ] Performance optimization (latency < 5s)
- [ ] Deploy to testnet
- [ ] Launch mainnet oracle network

## Economic Model

### Oracle Incentives
```rust
pub struct OracleEconomics {
    /// Base reward per correct response
    pub base_reward: Amount,

    /// Bonus for high-reputation oracles
    pub reputation_multiplier: f64,

    /// Penalty for incorrect responses
    pub slash_percentage: u8,  // 10%

    /// Minimum stake required
    pub min_stake: Amount,  // 1000 LINERA
}
```

**Example**:
- Oracle stakes **1000 LINERA**
- Submits **100 correct responses** → earns **50 LINERA** (0.5 per response)
- Submits **1 wrong response** → loses **100 LINERA** (10% slash)
- Net after 100 responses: **+50 LINERA** (if all correct) or **-50 LINERA** (if 1 wrong)

**Result**: Strong incentive to be accurate!

### User Benefits
- **Higher accuracy**: Consensus from 5+ oracles vs. single oracle
- **Fraud resistance**: AI detects manipulation
- **Richer insights**: "DeFi Expert" vs. "10 transactions"
- **Trustless**: TEE proves computation was correct

## Technical Challenges & Solutions

### Challenge 1: Oracle Coordination
**Problem**: How do oracles know when to submit scores?
**Solution**: Event-driven architecture
```rust
// Contract emits event when passport needs update
pub enum PassportEvent {
    UpdateRequested { token_id: TokenId, deadline: Timestamp },
    ConsensusReached { token_id: TokenId, score: u64 },
}
```

### Challenge 2: TEE Performance
**Problem**: SGX adds latency (2-5s overhead)
**Solution**:
- Batch processing (score 10 passports per enclave call)
- Parallel oracles (5 oracles = 5x throughput)
- Cache frequently accessed data

### Challenge 3: AI Cost
**Problem**: OpenAI API costs $0.01 per request × 1000 users = $10/batch
**Solution**:
- Users pay small fee (0.1 LINERA) for AI analysis
- Oracles split fee (incentivizes participation)
- Use smaller local models for simple cases (GPT-4 only for complex patterns)

### Challenge 4: Consensus Deadlock
**Problem**: What if oracles never reach 66%?
**Solution**: Fallback mechanisms
```rust
pub enum ConsensusOutcome {
    Agreed(Score),           // 66%+ agreement
    Disputed,                // No majority after 24h → use median
    NoResponses,             // 0 responses → keep old score
}
```

## Research Contributions

1. **First decentralized AI oracle on Linera** (novel architecture)
2. **TEE + Blockchain consensus** (hybrid trust model)
3. **Cross-chain reputation aggregation** (microchains use case)
4. **AI fraud detection** for Web3 identity (research paper material)

## Success Metrics

### Phase 2 Launch (Month 3)
- [ ] **5+ independent oracle operators**
- [ ] **99%+ consensus accuracy** (compared to ground truth)
- [ ] **< 10s latency** (from event to score update)
- [ ] **0 successful fraud cases** detected by AI

### 6 Months Post-Launch
- [ ] **50+ oracle nodes** globally distributed
- [ ] **10,000+ passports** using AI scoring
- [ ] **Research paper** published on decentralized AI oracles
- [ ] **Other projects** adopting this oracle pattern

## Open Questions (for Discussion)

1. **Oracle Selection**: Random subset (5 of 50 oracles) or all oracles?
2. **Reward Distribution**: Flat rate or stake-weighted?
3. **Dispute Resolution**: Who decides if oracle was "wrong"? (Arbitration DAO?)
4. **Privacy**: Should raw activity data be encrypted in TEE?
5. **Upgradability**: How to update oracle logic without re-registration?

## References

- [Intel SGX Documentation](https://www.intel.com/content/www/us/en/developer/tools/software-guard-extensions/overview.html)
- [Chainlink Decentralized Oracle Networks](https://chain.link/education/blockchain-oracles)
- [OpenAI API for Blockchain Analysis](https://platform.openai.com/docs)
- [Linera Microchains Whitepaper](https://linera.io/whitepaper)

---

**Status**: Phase 1 Complete ✅ | Phase 2 Planning 📋 | ETA: 3 months

**Contact**: For partnership or oracle operator inquiries, reach out to the team!
