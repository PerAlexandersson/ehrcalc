# Development checkpoint

## Ordering and audit remediation — 2026-11-20

Completed scope: constraint limits and polynomiality diagnostics, streaming
LR state budgets, sampling/ordering performance, CLI/MCP contracts and docs.
Automatic exact-subset ordering stops at 16 variables; explicit requests
support 22. Later strict probes are capped at two additional observations,
with feasibility screening through scale 256. Single factors reuse samples
and plans; split scoring uses one fixed order.

Formatting, strict workspace/all-target Clippy and CI tests pass: 378 passed,
0 failed and one optional timing test ignored in each of debug and release.
CLI documentation drift and real MCP stdio checks pass. Generic x86-64
release binaries were built. Exact arithmetic/counting comparisons pass.

The bounded 26-case automatic-policy comparison changed completion from
7 to 8 grids and retained 302 to 316 exact samples;
all 298 overlapping samples agree. Different sampling splits and
bounded pilots affect this combined comparison; forecasts remain heuristic.
Plan-only timings improved from 9.34 to 0.35 seconds for chain-100 and from
8.22 to 0.48 seconds for Birkhoff B8 on the measured machine.

Necessary polynomiality warnings do not certify an all-index polynomial.
State caps bound layer memory rather than total transition time. Empty or
censored strict probes may still require range-based fallback. Signed subset
sums and parallel constraint counting remain future work.
