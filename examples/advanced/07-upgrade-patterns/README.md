# Upgrade Patterns

Direct WASM upgrade, versioned storage migration, and init guards. Highest-level patterns for safe contract evolution.

## Role in Learning Path

This is the **final step** in the [upgrade patterns learning path](../README.md#upgrade-patterns--proxy-patterns). After mastering proxy patterns, this example shows:
- Direct WASM contract upgrades
- Versioned storage migrations (v1 to v2)
- Storage compatibility checks
- Init guard patterns to prevent re-initialization
- Safe state evolution across versions

**Prerequisites:** Understand all proxy and versioning patterns:
- [`02-beacon-proxy`](../02-beacon-proxy/) — Basic beacon concept
- [`03-beacon-proxy-factory`](../03-beacon-proxy-factory/) — Factory patterns
- [`03-proxy-admin`](../03-proxy-admin/) — Governance patterns
- [`04-upgradeable-proxy`](../04-upgradeable-proxy/) — Direct upgrade alternative
- [`06-beacon-management`](../06-beacon-management/) — Versioned implementations

## Key Concepts

- Direct WASM code replacement
- Batched storage migrations
- Version number tracking
- Dual-read patterns for backwards compatibility
- Init guard enforcement
- Safe state schema evolution

## Pattern Progression

**Basic beacon → Beacon factory → Governance → Direct upgrade → Versioning → Full patterns**

## When to Use

- Large-scale deployments where all contracts upgrade atomically
- State migrations that require coordinated changes
- Removing migration code after all instances are upgraded
- Complex storage restructuring

See the [advanced examples README](../README.md) for the full upgrade patterns learning path.
