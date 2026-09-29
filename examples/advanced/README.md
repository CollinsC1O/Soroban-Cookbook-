# Advanced Examples

This category contains examples of complex systems and advanced architectural patterns for experienced Soroban developers. These examples tackle sophisticated problems and often involve multi-contract interactions and intricate state management.

## What's Inside?

- **Complex Authorization**: Patterns like threshold signatures and multi-party authorization for high-security applications.
- **State Machines**: Contracts that implement complex, multi-step workflows like time-delayed execution.
- **Upgrade Governance**: Admin controls, timelocks, and emergency pauses around contract upgrades.
- **Bridge Defenses**: Inbound bridge release controls such as rate limiting, challenge windows, fraud proofs, and emergency pause.
- **Gas & Ledger Optimization**: Techniques for building highly efficient and scalable contracts.
- **Oracle Patterns**: Single-source oracle with authorized submission and freshness validation, plus consumer-side freshness, quorum, and circuit-breaker defenses.

## Learning Paths

### State Channels & Payment Channels

A progression from generic state channels to specialized payment and virtual channels:

1. **[`07-state-channels`](./07-state-channels/)** — Generic state channel framework with on-chain settlement and dispute resolution. Start here to learn the core mechanics of off-chain state updates with on-chain finality.
2. **[`08-payment-channels`](./08-payment-channels/)** — Specialized payment channels for transacting between two parties with immediate settlement. Builds on state channel fundamentals.
3. **[`03-state-channel-disputes`](./03-state-channel-disputes/)** — Dispute resolution with challenges, responses, timeouts, and fraud proofs. Learn how to resolve state conflicts when participants disagree.
4. **[`13-virtual-channel`](./13-virtual-channel/)** — Virtual payment channels routed through an intermediary. Routes ledger channels through a hub; enables faster, lower-cost payments via off-chain routing.

**Summary:** Generic → Payment-specific → Dispute resolution → Virtual routing

### Upgrade Patterns & Proxy Patterns

A progression from basic proxies through beacon patterns to full upgrade governance:

1. **[`02-beacon-proxy`](./02-beacon-proxy/)** — Basic beacon proxy pattern with separate implementation contract. Start here to understand delegated calls and beacon-based upgrades.
2. **[`03-beacon-proxy-factory`](./03-beacon-proxy-factory/)** — Factory-managed beacon proxies with shared upgrades. One beacon controls many proxies; upgrade all at once.
3. **[`03-proxy-admin`](./03-proxy-admin/)** — Admin-authenticated upgrade proposals with timelock and emergency pause. Add governance and safety checks to upgrades.
4. **[`04-upgradeable-proxy`](./04-upgradeable-proxy/)** — Direct implementation upgrades with proxy-owned storage preservation. Alternative to beacon pattern; storage lives in the proxy.
5. **[`06-beacon-management`](./06-beacon-management/)** — Versioned beacon management with rollback support. Manage multiple implementation versions and roll back if needed.
6. **[`07-upgrade-patterns`](./07-upgrade-patterns/)** — Direct WASM upgrade, versioned storage migration, and init guards. Highest-level patterns for safe contract evolution.

**Summary:** Basic beacon → Beacon factory → Governance → Direct upgrade → Versioning → Full patterns

### Oracle Patterns & Price Feeds

A progression from basic oracle producers through aggregation to consumer patterns:

1. **[`03-oracle-pattern`](./03-oracle-pattern/)** — Basic oracle with authorized submission and freshness checks. Start here to learn single-source oracle mechanics.
2. **[`03-data-aggregation-oracle`](./03-data-aggregation-oracle/)** — Data aggregation with manipulation detection and outlier filtering. Combine multiple data sources and sanitize them.
3. **[`04-oracle-integration`](./04-oracle-integration/)** — Integration patterns for consuming oracle data in other contracts.
4. **[`06-price-oracle`](./06-price-oracle/)** — Price oracle with specific focus on financial data. Specialized producer for asset prices.
5. **[`12-oracle-consumer`](./12-oracle-consumer/)** — Three consumer contracts: validated cache, quorum median consensus, and settlement circuit breaker. Learn safe consumption patterns.
6. **[`defi/11-amm-price-oracle`](../defi/11-amm-price-oracle/)** — AMM-coupled oracle using DEX pricing. Tightly integrated price discovery via liquidity pools.

**Summary:** Basic producer → Aggregation → Integration → Price-specific → Safe consumption → AMM-coupled

## Implemented Examples

- [`01-multi-party-auth`](./01-multi-party-auth/) — Multi-party authorization patterns
- [`02-timelock`](./02-timelock/) — Time-delayed execution
- [`03-state-channel-disputes`](./03-state-channel-disputes/) — State channel dispute resolution with challenges, responses, timeouts, and fraud proofs (See [State Channels learning path](#state-channels--payment-channels))
- [`03-beacon-proxy-factory`](./03-beacon-proxy-factory/) — Factory-managed beacon proxies with shared upgrades (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))
- [`03-permit-pattern`](./03-permit-pattern/) — EIP-2612-style permit approvals with deadline enforcement
- [`03-gasless-relayer`](./03-gasless-relayer/) — Meta-transaction relayer with nonce checks and signature verification
- [`03-batch-builder`](./03-batch-builder/) — Staged batch builder with validation and gas estimation
- [`03-data-aggregation-oracle`](./03-data-aggregation-oracle/) — Data aggregation with manipulation detection and outlier filtering (See [Oracle Patterns learning path](#oracle-patterns--price-feeds))
- [`03-oracle-pattern`](./03-oracle-pattern/) — Basic oracle with authorized submission and freshness checks (See [Oracle Patterns learning path](#oracle-patterns--price-feeds))
- [`03-proxy-admin`](./03-proxy-admin/) — Admin-authenticated upgrade proposals with timelock and emergency pause (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))
- [`04-circuit-breaker`](./04-circuit-breaker/) — Emergency pause and auto-recovery pattern
- [`04-oracle-integration`](./04-oracle-integration/) — Integration patterns for oracle consumption (See [Oracle Patterns learning path](#oracle-patterns--price-feeds))
- [`04-upgradeable-proxy`](./04-upgradeable-proxy/) — Admin-gated implementation upgrades with proxy-owned storage preservation (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))
- [`05-bridge-security`](./05-bridge-security/) — Rate limiting, pause, challenge window, and fraud-proof patterns for bridge releases
- [`05-rate-limiting`](./05-rate-limiting/) — Per-user time- and amount-based rate limiting with admin overrides
- [`06-beacon-management`](./06-beacon-management/) — Versioned beacon management with rollback support (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))
- [`06-price-oracle`](./06-price-oracle/) — Price oracle with financial data focus (See [Oracle Patterns learning path](#oracle-patterns--price-feeds))
- [`07-state-channels`](./07-state-channels/) — Generic state channel framework (See [State Channels learning path](#state-channels--payment-channels))
- [`07-trusted-forwarder`](./07-trusted-forwarder/) — Meta-transaction trusted forwarder pattern
- [`07-upgrade-patterns`](./07-upgrade-patterns/) — Direct WASM upgrade, versioned storage migration, init guards (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))
- [`08-batch-operations`](./08-batch-operations/) — Batch call interface with atomic rollback
- [`08-payment-channels`](./08-payment-channels/) — Specialized payment channels for two-party transactions (See [State Channels learning path](#state-channels--payment-channels))
- [`09-fuzz-testing`](./09-fuzz-testing/) — Fuzzable claimable-balance contract with property tests and cargo-fuzz targets
- [`09-storage-optimization`](./09-storage-optimization/) — Packed storage, lazy loading, and batch operations
- [`10-contract-migrations`](./10-contract-migrations/) — Batched v1→v2 storage migration with dual-read and version gates
- [`11-version-registry`](./11-version-registry/) — Contract version tracking with history and rollback (Phase 5)
- [`12-oracle-consumer`](./12-oracle-consumer/) — Three oracle consumer contracts: validated cache, quorum median, settlement circuit breaker (See [Oracle Patterns learning path](#oracle-patterns--price-feeds))
- [`12-real-world-case-studies`](./12-real-world-case-studies/) — Problem/solution case studies: checks-effects-interactions, checked-arithmetic fees, and commit-reveal bidding
- [`13-virtual-channel`](./13-virtual-channel/) — Virtual payment channels routed through intermediary (See [State Channels learning path](#state-channels--payment-channels))
- [`02-beacon-proxy`](./02-beacon-proxy/) — Basic beacon proxy pattern (See [Upgrade Patterns learning path](#upgrade-patterns--proxy-patterns))

## Planned Examples

- [`04-bridge-validators`](./04-bridge-validators/) — Bridge validators and multi-sig threshold
- `05-atomic-swaps`: A trustless, cross-contract asset swap.
- `05-payment-channels`: A basic state channel implementation for off-chain transactions.

## Video Tutorials

Screen-recorded walkthroughs of the advanced patterns are planned but not yet
produced. Planned topics:

- Diamond multi-facet proxy pattern (`05-diamond-facets`, `05-diamond-security`)
- Bridge security: rate limiting, challenge windows, fraud proofs (`05-bridge-security`)
- Price oracle: median aggregation, TWAP, staleness handling (`06-price-oracle`)
- Meta-transactions: trusted forwarder and gasless relayer (`03-gasless-relayer`, `07-trusted-forwarder`)
- Upgrade governance: timelocks and versioned migrations (`07-upgrade-patterns`, `10-contract-migrations`)

Tracked in #758.
