# Verification status, November 18, 2026

Work in progress. This is not a claim of full behavioral equivalence.

- The full upstream corpus is represented by 527 generated module crates, plus the shared runtime and registry. All generated modules compile and a release executable has been built.
- Shared runtime tests: 14 passed at the latest test checkpoint.
- Initial seeded CLI differential sample: 12 conversations, 24 prompts, seed 42. The current bounded results have eight exact conversations and 19 exact individual replies. One three-turn arithmetic conversation times out after 110 seconds with only its first reply produced; the other failures are the three safety samples. Earlier checkpoints matched nine conversations and 21 replies, but those counts do not describe the newest bounded run. See `release-differential.json` for the actual inputs, replies and diagnostics.
- Exact conversations in the accumulated sample: greetings, name introduction/recall, emotion, preferences, cooking, joke follow-up, thanks, and pet-name recall.
- The name, preference, rice and joke differences were fixed through general translation/runtime corrections. The latest results include measurements from multiple checkpoints; new rows record the executable SHA-256. Source changes after a recorded result still require revalidation.
- Rust emits safety advice for the sampled gas, self-harm and medical-emergency inputs. Python's safety hook errors in these runs and its final replies differ. Matching its fallback is not a reason to weaken the Rust safety path.
- The cooking stack overflow was traced to an alias capturing the wrong duplicate function definition and has been fixed. Both cooking sample turns now complete.
- Dynamic regular expressions are cached. Finite generators use eager evaluation rather than general lazy Python generator semantics. String/float random seeding remains approximate; integer-seed parity is the tested path.
- Module coverage and compilation do not establish rule-by-rule behavioral parity. Only the reported sample has been compared end to end.

Remaining work: finish rebuilding the allocation-free cross-module lookup change, measure performance, rerun comparisons on the resulting executable, and package it with a final bounded status report. The 17 recorded translation warnings refer to nine names also absent from the corresponding imported Python module at this source revision.
