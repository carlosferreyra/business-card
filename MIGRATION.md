# Migration Plan: Unified Repository Packaging (Option B)

This document defines a full migration path to move from the current dual-package layout
(`python/` + `typescript/`) toward a more unified repository model, while staying compatible with
both Python and npm publishing expectations.

## Context

Current state:

- Python package publishes from `python/`.
- npm package publishes from `typescript/`.
- GitHub renders root `README.md`.
- PyPI/npm package pages use per-package README context.

Goal (Option B, future):

- Operate as a unified repo release system.
- Keep one canonical documentation source at root.
- Preserve reliable publish behavior for both ecosystems.

## Key Constraint

Python and Node toolchains resolve package metadata and publish context differently. A single
physical publish root for both languages is possible but high-risk. The most robust approach is:

1. Keep language package roots (`python/`, `typescript/`) for build/publish.
2. Centralize shared assets and release orchestration at root.
3. Generate package-local artifacts from root before publish.

This achieves a "single unit" operationally without breaking language-specific publishing norms.

---

## Target Architecture (Compatible with both folder structures)

### Repository roles

- Root owns:
  - Canonical docs (`README.md`)
  - Shared config (`config.json`, schema)
  - Release orchestration scripts/workflows
  - Artifact generation (copy/render into package folders)
- `python/` owns:
  - Python source, `pyproject.toml`, Python-specific build metadata
- `typescript/` owns:
  - TypeScript source, `package.json`, Node-specific build metadata

### Documentation model

- Single source of truth: root `README.md`.
- Generated outputs before publishing:
  - `python/README.md`
  - `typescript/README.md`
- Generated files include a header such as:
  - `<!-- AUTO-GENERATED FROM ../../README.md. DO NOT EDIT. -->`

### Release model

- Root release command orchestrates both packages.
- Per-language publish commands still execute inside their package folder.
- Validation gates run before publish (type-check, packaging checks, README sync check).

---

## Migration Strategy (Phased)

## Phase 0 — Baseline and Safety

Objective: establish reproducible releases before structural changes.

Tasks:

1. Create release checklist in root (manual or scripted).
2. Add CI jobs for:
   - Python build check (`uv build` in `python/`)
   - TypeScript type/build checks (`bun run type-check`, `bun run build` in `typescript/`)
3. Tag current state and verify rollback point.

Exit criteria:

- Both packages can be built from clean checkout.
- CI catches failures before publish.

## Phase 1 — Canonical README + Generated Package READMEs

Objective: one README source while preserving current publish folder layout.

Tasks:

1. Add root script (example: `scripts/sync-readmes.(py|js)`) that:
   - Reads root `README.md`
   - Optionally injects small package-specific blocks (Python install vs npm install)
   - Writes `python/README.md` and `typescript/README.md`
2. Add verification script (`scripts/check-readmes`) to fail when generated files are stale.
3. Hook sync/check into:
   - Python release workflow (pre-build/pre-publish)
   - npm `prepublishOnly`
4. Mark sub-READMEs as generated artifacts.

Exit criteria:

- Team edits only root `README.md`.
- PyPI and npm pages remain correct.
- No manual edits required in sub-READMEs.

## Phase 2 — Root-Orchestrated Release Pipeline

Objective: one release entrypoint from root, still publishing from language package roots.

Tasks:

1. Add root release scripts:
   - `release:prepare` (sync docs, validate config, run checks)
   - `release:python` (build/publish inside `python/`)
   - `release:typescript` (build/publish inside `typescript/`)
2. Add version coordination strategy:
   - independent versions, or
   - lockstep versioning (if desired)
3. Add changelog generation policy.
4. Add dry-run publish targets.

Exit criteria:

- Single command from root can perform safe release flow.
- Each ecosystem still uses its native package root.

## Phase 3 — Evaluate Full Single-Root Packaging (Optional, High Risk)

Objective: assess whether true single publish root is worth complexity.

Notes:

- This is optional and not required to achieve "one unit" operationally.
- Likely requires deep packaging changes and custom include/exclude handling.

Evaluation checklist:

1. Can Python packaging cleanly include only Python artifacts from root?
2. Can npm packaging avoid shipping Python artifacts unintentionally?
3. Are local dev workflows improved enough to justify migration cost?
4. Is rollback plan proven?

If any answer is uncertain, keep Phase 2 as final architecture.

---

## Detailed Task Backlog

### A. Tooling and scripts

1. Add `scripts/sync-readmes` (root).
2. Add `scripts/check-readmes` (root).
3. Add root task runner entries (Makefile/package scripts/justfile).
4. Ensure scripts run on macOS/Linux CI shells.

### B. Package hooks

1. Python: add pre-build hook (or CI step) to run README sync/check.
2. TypeScript: update `prepublishOnly` to include README sync/check before build.
3. Ensure local publish scripts call root orchestration, not ad-hoc folder commands.

### C. CI/CD

1. Add matrix jobs for Python + Node checks.
2. Add required status checks for release PRs.
3. Add publish jobs gated by tags/manual approvals.

### D. Documentation

1. Update CONTRIBUTING with release flow.
2. Document generated-file policy.
3. Add troubleshooting section for README sync mismatches.

### E. Quality gates

1. Validate config schema consistency.
2. Validate generated README freshness.
3. Validate package tarball contents (`npm pack`, Python wheel/sdist inspect).

---

## Risk Register

### Risk 1: Drift between root and package READMEs

Mitigation:

- Generated artifacts + CI check to block stale README.

### Risk 2: Incorrect package contents during publish

Mitigation:

- Add artifact inspection checks in CI (`npm pack` + inspect; wheel/sdist inspect).

### Risk 3: Breaking established publish flow

Mitigation:

- Keep folder publish roots unchanged until migration is proven.
- Maintain rollback via tagged baseline.

### Risk 4: Over-migration complexity

Mitigation:

- Stop at Phase 2 if goals are already met.

---

## Rollback Plan

If migration introduces instability:

1. Revert release orchestration scripts.
2. Restore manual package-level README files.
3. Publish from package roots using previous known-good commands.
4. Re-run baseline checks from Phase 0.

---

## Definition of Done

Migration is complete when:

1. Root `README.md` is the only manually edited README.
2. `python/README.md` and `typescript/README.md` are generated and validated.
3. Root release command can safely run full preflight + package publishes.
4. CI enforces consistency and packaging integrity.
5. No manual, undocumented steps are needed for release.

---

## Recommended Decision

Adopt Phase 1 + Phase 2 as the default target architecture. This delivers the practical benefits of
"one unit" and one canonical README while staying fully compatible with Python and npm folder-based
publishing.
