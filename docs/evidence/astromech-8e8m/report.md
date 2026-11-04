# astromech-8e8m: push-event check of pushed commits, local emulation

The Atelier job on a push event, emulated in a scratch clone of the local repository (never
pushed): `actions/checkout` of `GITHUB_SHA` detached, then the `run:` scripts of the
`atelier doctor` and `atelier land (pushed commits)` steps, read from
`.github/workflows/atelier.yml` and run with `bash -e`, with `BEFORE` standing in for
`github.event.before`. The land step runs whether or not doctor passed (`!cancelled()`).
Atelier was built from the pinned `ATELIER_REV` (the installed binary is 227 commits newer and
its `check_base_commits` differs). In the emulation `refs/remotes/origin/main` starts at
1d2219d7 (origin/main on 2026-11-04), which is why doctor prints an "ahead of its upstream"
warning on some scenarios; on GitHub it starts at the pushed commit.

## Harness

`ci.sh <clone>` (env `BEFORE`, `GITHUB_SHA`):

```bash
set -u
WF=/home/supadava/projects/astromech-wt9/.github/workflows/atelier.yml
S=$(dirname "$0")
cd "$1"
git checkout --quiet --detach "$GITHUB_SHA"
echo "--- step: atelier doctor"; bash -e <(python3 -I "$S/steps.py" "$WF" "atelier doctor") 2>&1 | tail -5; echo "exit ${PIPESTATUS[0]}"
echo "--- step: atelier land (pushed commits)"; BEFORE="$BEFORE" bash -e <(python3 -I "$S/steps.py" "$WF" "atelier land (pushed commits)") 2>&1; echo "exit $?"
```

`steps.py` prints the `run:` of the named step of `jobs.doctor.steps`. `scenarios.sh` builds the commits:

```bash
S=$1; export PATH=$S/bin:$PATH
rm -rf $S/astro2; git clone -q /home/supadava/projects/astromech $S/astro2; cd $S/astro2
B=1d2219d740be577747a8e224c289df5f6f292754
G="git -c user.name=t -c user.email=t@x"
echo "atelier: $(atelier --version)  (ATELIER_REV in workflow: $(grep -o 'ATELIER_REV: .*' /home/supadava/projects/astromech-wt9/.github/workflows/atelier.yml))"
# (a) leaf validation issue astromech-1b43 marked done in one commit on main
git checkout -q --detach $B
sed -i 's/^status: "todo"/status: "done"/' .atelier/issues/astromech-1b43.md
printf '\n## Close Reason\n\nChecked on Windows.\n' >> .atelier/issues/astromech-1b43.md
$G commit -qam "atelier: close astromech-1b43"; A=$(git rev-parse HEAD)
# (c) records-only planning commit: a new issue made with atelier issue create on main
git checkout -q -B main $B
atelier issue create "Test planning issue for the CI push check" --issue-type task --body $'## Goal\n\nA planning record.\n\n## Done when\n\nNever; scratch test.' -q >/dev/null
git add -A .atelier; $G commit -qm "atelier: plan a test issue"; C=$(git rev-parse HEAD)
# (d) code commit on main
git checkout -q --detach $B; echo "// test" >> README.md; $G commit -qam "Edit README on main"; D=$(git rev-parse HEAD)
git checkout -q --detach $B; git branch -q -D main
run() { echo; echo "=== $1"; echo "BEFORE=$2 GITHUB_SHA=$3"; git log --oneline --no-decorate -1 $3 | sed 's/^/tip: /'; git update-ref refs/remotes/origin/main $B; BEFORE=$2 GITHUB_SHA=$3 ci.sh $S/astro2 2>&1; }
run "(a) one commit marks leaf astromech-1b43 done without review" $B $A
run "(b) the real reviewed range eb74ee23..origin/main (atelier merge integrations)" $(git rev-parse eb74ee23) $B
run "(c) records-only planning commit" $B $C
run "(d) code commit straight on main" $B $D
run "(e) new main: before is all zeros" 0000000000000000000000000000000000000000 $C
git merge-base --is-ancestor $A $C || echo && echo "(f) note: before $A is not an ancestor of tip $C"
run "(f) force push replacing (a) with (c)" $A $C
run "(g) force push whose before cannot be fetched" 1111111111111111111111111111111111111111 $C
```

## Transcript

```text
atelier: atelier 0.2.0+1f5fd1c2  (ATELIER_REV in workflow: ATELIER_REV: 1f5fd1c25a84740a637366832f2bdeb9233b46bb)

=== (a) one commit marks leaf astromech-1b43 done without review
BEFORE=1d2219d740be577747a8e224c289df5f6f292754 GITHUB_SHA=b9ff72513a6a2488e4d1b9bdc7ab321c5ec9739c
tip: b9ff7251 atelier: close astromech-1b43
--- step: atelier doctor
HEAD is now at b9ff7251 atelier: close astromech-1b43
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
Error: land_base_done_claim: commit b9ff7251 on main marks astromech-1b43 done outside review; a done claim is committed on the work branch with the work it describes. Undo it on main with `git revert b9ff7251`, or edit astromech-1b43 back to its earlier status and commit, then run `atelier start astromech-1b43` and make the claim on the work branch. To retire work that will not be done, supersede it instead of closing it.
land: refused — 1 check (land_base_done_claim)
exit 1

=== (b) the real reviewed range eb74ee23..origin/main (atelier merge integrations)
BEFORE=eb74ee23f720934b96074ec500900fb5f3843c12 GITHUB_SHA=1d2219d740be577747a8e224c289df5f6f292754
tip: 1d2219d7 Merge branch 'main' into task/astromech-az6g
--- step: atelier doctor
HEAD is now at 1d2219d7 Merge branch 'main' into task/astromech-az6g
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
Warning: main is ahead of its upstream with 47 commits that no owner made: 1d2219d7 Merge branch 'main' into task/astromech-az6g (Sumeet Padavala); 1cc512dd Merge branch 'main' into task/astromech-7c7c (Sumeet Padavala); 86bfe532 atelier: az6g review dispositions (R1-R3 deferred to astromech-7ew2, R4 no action) (Sumeet Padavala); d45ec256 Record approval astromech-ehud (Sumeet Padavala); 4d2b04a2 Record finding astromech-lpn8 (Sumeet Padavala); and 42 more. Pushing main publishes them; confirm they are meant to go out.
No work owner on the integration base.
Base commits: 12 unpushed record-only commits outside review.
Landing checks passed. No commits or records created.
land: passed
exit 0

=== (c) records-only planning commit
BEFORE=1d2219d740be577747a8e224c289df5f6f292754 GITHUB_SHA=bbcd986c0cf419c162d19ab79cbb3ec29688f8bf
tip: bbcd986c atelier: plan a test issue
--- step: atelier doctor
HEAD is now at bbcd986c atelier: plan a test issue
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on the integration base.
Base commits: 1 unpushed record-only commit outside review.
Landing checks passed. No commits or records created.
land: passed
exit 0

=== (d) code commit straight on main
BEFORE=1d2219d740be577747a8e224c289df5f6f292754 GITHUB_SHA=58f7e002752097359a71aa6b77cea9d3991c5ec8
tip: 58f7e002 Edit README on main
--- step: atelier doctor
HEAD is now at 58f7e002 Edit README on main
Warning: main is ahead of its upstream with 1 commit that no owner made: 58f7e002 Edit README on main (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
Error: land_base_unreviewed: commit 58f7e002 on main changes README.md without recorded review; only records and mission strategies may be committed on main directly. Move it to a work branch: `git switch -c <issue_type>/<id>`, `git branch -f main main@{upstream}`, then `atelier start <id>` there; get it reviewed and integrate it with `atelier merge`.
land: refused — 1 check (land_base_unreviewed)
exit 1

=== (e) new main: before is all zeros
BEFORE=0000000000000000000000000000000000000000 GITHUB_SHA=bbcd986c0cf419c162d19ab79cbb3ec29688f8bf
tip: bbcd986c atelier: plan a test issue
--- step: atelier doctor
HEAD is now at bbcd986c atelier: plan a test issue
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
main is new: no earlier commit to check from.
exit 0

(f) note: before b9ff72513a6a2488e4d1b9bdc7ab321c5ec9739c is not an ancestor of tip bbcd986c0cf419c162d19ab79cbb3ec29688f8bf

=== (f) force push replacing (a) with (c)
BEFORE=b9ff72513a6a2488e4d1b9bdc7ab321c5ec9739c GITHUB_SHA=bbcd986c0cf419c162d19ab79cbb3ec29688f8bf
tip: bbcd986c atelier: plan a test issue
--- step: atelier doctor
HEAD is now at bbcd986c atelier: plan a test issue
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on the integration base.
Base commits: 1 unpushed record-only commit outside review.
Landing checks passed. No commits or records created.
land: passed
exit 0

=== (g) force push whose before cannot be fetched
BEFORE=1111111111111111111111111111111111111111 GITHUB_SHA=bbcd986c0cf419c162d19ab79cbb3ec29688f8bf
tip: bbcd986c atelier: plan a test issue
--- step: atelier doctor
HEAD is now at bbcd986c atelier: plan a test issue
Warning: main is ahead of its upstream with 1 commit that no owner made: bbcd986c atelier: plan a test issue (t). Pushing main publishes them; confirm they are meant to go out.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
exit 0
--- step: atelier land (pushed commits)
fatal: git upload-pack: not our ref 1111111111111111111111111111111111111111
fatal: remote error: upload-pack: not our ref 1111111111111111111111111111111111111111
exit 128
```
