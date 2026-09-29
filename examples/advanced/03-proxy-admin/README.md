# Proxy Admin

Admin-authenticated upgrade proposals with timelock and emergency pause.

## Role in Learning Path

This is the **third step** in the [upgrade patterns learning path](../README.md#upgrade-patterns--proxy-patterns). After learning basic and factory-managed proxies, this example adds:
- Admin authorization and access control
- Timelock delays for planned upgrades
- Emergency pause mechanisms
- Staged upgrade governance
- Risk mitigation for production systems

**Prerequisites:** 
- Start with [`02-beacon-proxy`](../02-beacon-proxy/) for beacon basics
- Then [`03-beacon-proxy-factory`](../03-beacon-proxy-factory/) for factory patterns

**Next steps:**
- **[`04-upgradeable-proxy`](../04-upgradeable-proxy/)** — Alternative pattern: storage in proxy
- **[`06-beacon-management`](../06-beacon-management/)** — Versioned implementations with rollback
- **[`07-upgrade-patterns`](../07-upgrade-patterns/)** — Complete upgrade strategies

## Key Concepts

- Role-based access control for upgrades
- Timelock-enforced delays
- Emergency pause for rollback
- Proposal lifecycle management
- Production-grade safety checks

## Pattern Progression

**Basic beacon → Beacon factory → Governance → Direct upgrade → Versioning → Full patterns**

See the [advanced examples README](../README.md) for the full upgrade patterns learning path.
