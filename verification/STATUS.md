# Verification status, November 18, 2026

Work in progress. This is not a claim of full behavioral equivalence.

- The full upstream corpus is represented by 527 generated module crates, plus the shared runtime and registry. All generated modules compile and a release executable has been built.
- Shared runtime tests: 13 passed at the latest test checkpoint.
- Initial seeded CLI differential sample: 12 conversations, 24 prompts, seed 42. Five conversations and 16 individual replies exactly matched Python. See `release-differential.json` for the actual inputs, replies and diagnostics.
- Exact conversations in that sample: greetings, arithmetic/geography, emotion, thanks, and pet-name recall.
- Differences remain in personal-name and preference recall, rice instructions, joke follow-up, and safety routing. An imported-priority registry fix is being rebuilt and has not yet been reflected in those results.
- Rust emits safety advice for the sampled gas, self-harm and medical-emergency inputs. Python's safety hook errors in these runs and its final replies differ. Matching its fallback is not a reason to weaken the Rust safety path.
- The cooking stack overflow was traced to an alias capturing the wrong duplicate function definition and has been fixed. Both cooking sample turns now complete.
- Dynamic regular expressions are cached. Finite generators use eager evaluation rather than general lazy Python generator semantics. String/float random seeding remains approximate; integer-seed parity is the tested path.
- Module coverage and compilation do not establish rule-by-rule behavioral parity. Only the reported sample has been compared end to end.

Remaining work: rebuild the imported-priority fix, retest affected conversations, investigate remaining translation differences, refresh the verification evidence, and package the executable with a final bounded status report.
