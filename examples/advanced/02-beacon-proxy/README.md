# Beacon Proxy Pattern

Basic beacon proxy pattern with separate implementation contract using delegated calls.

## Role in Learning Path

This is the **foundation example** in the [upgrade patterns learning path](../README.md#upgrade-patterns--proxy-patterns). Start here to understand:
- How delegated call works
- Proxy contract pattern
- Separate implementation storage
- Beacon-based implementation reference
- Basic upgrade mechanics

After learning this foundation, proceed to:
- **[`03-beacon-proxy-factory`](../03-beacon-proxy-factory/)** — Manage many proxies via one beacon
- **[`03-proxy-admin`](../03-proxy-admin/)** — Add governance and safety checks
- **[`04-upgradeable-proxy`](../04-upgradeable-proxy/)** — Alternative pattern: storage in proxy
- **[`06-beacon-management`](../06-beacon-management/)** — Versioned implementations with rollback
- **[`07-upgrade-patterns`](../07-upgrade-patterns/)** — Complete upgrade strategies

## Key Concepts

- Delegated call semantics (msg.sender preservation)
- Implementation contract separation
- Beacon contract pointing to current implementation
- Storage layout isolation
- Simple upgrade path

## Pattern Progression

**Basic beacon → Beacon factory → Governance → Direct upgrade → Versioning → Full patterns**

See the [advanced examples README](../README.md) for the full upgrade patterns learning path.
