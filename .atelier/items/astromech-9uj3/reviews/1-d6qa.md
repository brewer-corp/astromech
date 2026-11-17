---
schema: atelier.review
schema_version: 3
round: 1
commit: 58b8c6828dc5e22c687b80b297e1f06a6f890c7e
reviewers:
- thread:delegated-task:command%3Amcp%3A576694aa-c3f9-4179-842e-71eb9f9e8eab%3Adelegate-task%3A3dafe6e2-4fd9-4c00-973e-cf647c47f5ff
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T13:48:50.415690028Z
---

## Summary

Complexity: medium. Four independent failures span dependency resolution, Cargo features, frontend imports and the Atelier workflow, but the fixes are small and separable.

Value: high. Passing CI is necessary to detect regressions, and this restores compilation of the Windows window code.

Risk of merging: medium. SQLite, zip and JSON parsing ship in the app; the changes are covered by both CI jobs and can be reverted.

The value justifies the change. I found no blocking defect or substantially smaller replacement for these fixes. The chrono exceptions are an explicit stopgap, limited to informational unmaintained advisories, with replacement tracked in astromech-y374 on main.

Reviewed main...58b8c6828dc5e22c687b80b297e1f06a6f890c7e, including the complete lockfile diff, renamed module and all its imports, lint implementation and tests, workflow pin, float feature and fixture changes. Checked jsonschema's dependency manifest to confirm it enables float_roundtrip. Read the ignored advisories from the refreshed RustSec database and confirmed their classification and lack of a patched version.

Queried the main run histories and read the failure logs for 43196334048, 43200713843, 44027233143 and 44083188995. They support the Report's first failing runs and the SQLite advisory, case-sensitive import, unsupported mission status and three gravity-value discrepancies respectively. The relevant source changes support the named introducing commits. The Windows tests succeeded in 43200713843 despite its world cleanup warnings.

Independently verified successful workflow-dispatch runs [44093067446](https://github.com/brewer-corp/astromech/actions/runs/44093067446) and [44093072409](https://github.com/brewer-corp/astromech/actions/runs/44093072409), both at d5276fdd3da8312874c36757e2eed4fd4a51c19d. The reviewed head changes only the item record after that commit. Both Linux and Windows CI jobs passed, including Windows build and clippy with the window feature, which compile astromech-7o1g's host.rs change. The Linux job also passed the full workspace tests, frontend checks, bridge check and journeys.

Locally ran cargo deny check; pnpm lint; the new lint-rule suite (7 tests); and the preview_fixtures suite both with cargo test -p astromech-app --test preview_fixtures --locked and cargo test --workspace --test preview_fixtures --locked (3 tests each). All passed. Atelier check accepts the records. git diff --check reports only an extra blank line at the end of the item report, with no correctness impact.

I did not rerun the full workspace or frontend suite locally, or run the Windows app. The third Done when line is not yet demonstrated: the successful runs are on the item branch, not its eventual main merge and subsequent push. This review approves the code for merge; it does not certify that post-merge requirement. The implementer must finish the two main runs and record their IDs and push count as the Report already promises. The remaining Done when claims are supported, including the explanation for leaving caused_by empty.

## Finding 1: Windows world cleanup leaves temporary directories behind
Recommendation: defer

This predates the change and does not explain the red CI jobs, so it need not block these fixes. It is nevertheless a real cleanup defect rather than harmless output. In crates/astromech-core/src/world.rs:752, world.dir.close() runs while the core's database and log handles can remain open. On failure it prints the error and returns success. Windows run 43200713843 records repeated sharing violations from this path even though the tests pass. The undeleted astromech-world-* directories remain on a persistent Windows development machine. Track a separate fix that releases the owning handles before removing the directory and verifies removal after the child exits; do not hide it with retries or suppressing the warning.
