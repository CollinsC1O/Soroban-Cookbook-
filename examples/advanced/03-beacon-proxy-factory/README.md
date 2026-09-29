# Beacon Proxy Factory

Factory-managed beacon proxies with shared upgrades. One beacon controls many proxy instances.

## Role in Learning Path

This is the **second step** in the [upgrade patterns learning path](../README.md#upgrade-patterns--proxy-patterns). After learning basic beacons, this example shows:
- Factory pattern for deploying proxy instances
- One beacon serving many proxies
- Batch upgrades of all proxies at once
- Cost-efficient multi-proxy management
- Decoupling proxy deployment from implementation

**Prerequisites:** Start with [`02-beacon-proxy`](../02-beacon-proxy/) to understand beacon basics.

**Next steps:**
- **[`03-proxy-admin`](../03-proxy-admin/)** — Add governance and safety checks
- **[`04-upgradeable-proxy`](../04-upgradeable-proxy/)** — Alternative pattern: storage in proxy
- **[`06-beacon-management`](../06-beacon-management/)** — Versioned implementations with rollback
- **[`07-upgrade-patterns`](../07-upgrade-patterns/)** — Complete upgrade strategies

## Key Concepts

- Factory-based proxy creation
- Beacon-mediated implementation sharing
- Atomic multi-proxy upgrades
- Cost optimization for multiple instances
- Shared upgrade governance

## Pattern Progression

**Basic beacon → Beacon factory → Governance → Direct upgrade → Versioning → Full patterns**

See the [advanced examples README](../README.md) for the full upgrade patterns learning path.
