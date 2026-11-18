# Work in progress, November 18, 2026

Not a completed behaviorally verified port.

- All 527 generated modules compile. Full executable builds.
- Runtime unit tests: 9 passed.
- Latest direct knowledge probe correctly selects `gas-shut-off` for the gas sample after fixing loop-else semantics.
- Seeded full CLI gas and self-harm cases both return safety replies in Rust. Neither exactly matches Python: Python falls through its failing safety hook. Recorded cases: 2, exact matches: 0. This is not a passing parity suite.
- Full conversation sample suite is not finished.
- Release build is not yet verified.
- No domain initialization failures are reported by current traces, but this is not exhaustive behavior coverage.
- Runtime is slow: roughly 44-57 seconds per fresh-process safety sample before adding a dynamic regex compilation cache. Cache verification is pending.
- Full generated code includes finite eager generator approximations, not general lazy Python generators. String/float random seeding is approximate.
- Some unresolved calls in upstream modules.memory_store remain generator warnings. Their upstream definitions also need investigation before deciding whether these are upstream defects or translation gaps.

Next: verify regex cache, run the differential suite, investigate non-safety differences, build release, and report measured coverage and remaining gaps.
