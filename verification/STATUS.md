# Verification status, November 18, 2026

Experimental full-corpus translation. Not a claim of full behavioral equivalence or production readiness.

## Source coverage and build

- Upstream: huggingface/pybot, commit `7d63c7fbebb44c7e466e5c8b5c1f341ab767929e`.
- 527 non-root Python modules, 636,550 source lines, translated into generated Rust crates. The remaining root initializer contains only a one-line documentation string.
- `module-manifest.json` maps every translated module to its source hash and generated Rust path. Coverage means the source is represented, not that every rule has been exercised.
- All generated crates compile and the release executable builds on Linux x86_64. The supplied binary needs glibc 2.35 or newer.
- 18 tests pass: 14 compatibility-library tests, two runtime memory tests, and two generated-translation/priority tests. The latter check recursive phrase matching and the safety band's registry priority and arbiter precedence.
- Upstream LICENSE is retained byte for byte. NOTICE records attribution and modifications. No GitHub Actions workflows are included.

## Differential sample

Both result files below record the same unstripped executable SHA-256:

`bf98fc48613880ed03b5c39ce2e1c0769a2ba2ab7ebf72108aa0dbe06a569520`

The final rebuild after reverting a failed cache experiment produced that exact hash again.

| Sample | Conversations | Prompts | Exact conversations | Exact replies |
| --- | ---: | ---: | ---: | ---: |
| `release-differential.json` | 12 | 24 | 8 | 20 |
| `extended-differential.json` | 11 | 14 | 11 | 14 |
| Total | 23 | 38 | 19 | 34 |

All runs use integer seed 42. Python baselines were recorded from the source revision above. Reused baselines are marked and their source revision was checked before reuse. Replies are compared as complete output lines, including punctuation. Results include actual inputs, outputs, module diagnostics, executable hashes and timeout details.

The four non-exact conversations are:

- Three safety cases: gas, self-harm and chest pain/breathing difficulty. Rust emits safety advice; the Python safety hook raises a regex error in these runs and produces different final replies. These count as differences, not passing parity tests. This does not establish the safety or medical correctness of either implementation.
- One three-turn arithmetic/geography conversation exceeds a 110-second Rust process limit after producing the correct answers for 2+2 and 12*8. Its last reply is missing at the limit. Paris matches in a separate single-turn extended case.

## Known limitations

- Runtime is slow: sampled fresh processes commonly take about 30 to 58 seconds, with some conversations exceeding 110 seconds. This is not a low-latency replacement for Python.
- General lazy Python generators are approximated by finite eager evaluation. String/float random seeding is approximate; integer seeding is the tested path. Python language and regex behavior are not universally implemented.
- Seventeen generator warnings refer to nine names also absent from the corresponding imported Python module at this source revision. See `translation-warnings.json`.
- The compatibility library is purpose-built for this corpus, not a general Python implementation. Unexercised paths can still differ or fail. Do not rely on this experimental port for medical, crisis, food-safety or other safety-critical advice.
- `capture-cache-experiment.json` and `timing-check.json` retain failed performance experiments. They are not the release sample or additional passing tests.

The full corpus, a buildable executable and bounded verification evidence are delivered. Exhaustive behavioral parity and acceptable runtime performance remain open engineering work.
