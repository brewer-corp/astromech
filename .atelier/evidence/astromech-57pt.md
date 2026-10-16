---
created_at: "2026-10-16T22:51:08.187788656+00:00"
id: "astromech-57pt"
evidence_type: "check"
commit: "63e039957666524f4c33bd07fe8bed2051df8e39"
result: "pass"
captured_at: "2026-10-16T22:50:47.835395202+00:00"
command: "bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-m30a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-m30a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging trade: app install, lint, svelte-check, build, vitest"
updated_at: "2026-10-16T22:51:08.188155490+00:00"
---

## Summary

After merging trade: app install, lint, svelte-check, build, vitest

## Command

```console
bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 8566
Truncated: yes

```text
Lockfile is up to date, resolution step is skipped
Already up to date

Done in 647ms using pnpm v10.34.6

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-feature-astromech-m30a/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-feature-astromech-m30a/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792191058876 START "/home/supadava/projects/astromech-wt-feature-astromech-m30a/app"
1792191058877 COMPLETED 310 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@0.1.5 build /home/supadava/projects/astromech-wt-feature-astromech-m30a/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 263 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                                                          0.39 kB │ gzip:  0.26 kB
dist/assets/saira-semi-condensed-vietnamese-600-normal-CtOKQqyp.woff     5.85 kB
dist/assets/saira-semi-condensed-vietnamese-400-normal-DZeVTnup.woff     5.89 kB
dist/assets/saira-semi-condensed-vietnamese-500-normal-Dvrr_M0-.woff     5.90 kB
dist/assets/saira-semi-condensed-vietnamese-600-normal-BleKkWhQ.woff2    7.06 kB
dist/assets/saira-semi-condensed-vietnamese-400-normal-BjlVRgV2.woff2    7.08 kB
dist/assets/saira-semi-condensed-vietnamese-500-normal-DaSOOpQd.woff2    7.14 kB
dist/assets/share-tech-mono-latin-400-normal-15MgdvEM.woff              10.24 kB
dist/assets/share-tech-mono-latin-400-normal-CCqdbj5z.woff2             13.50 kB
dist/assets/saira-semi-condensed-latin-ext-400-normal-FBtzH8lb.woff     13.54 kB
dist/assets/saira-semi-condensed-latin-ext-600-normal-gxH8H1Hk.woff     13.56 kB
dist/assets/saira-semi-condensed-latin-ext-500-normal-CBunCqud.woff     13.70 kB
dist/assets/saira-semi-condensed-latin-ext-400-normal-z9rK9nq6.woff2    14.67 kB
dist/assets/saira-semi-condensed-latin-ext-600-normal-cD8iogYK.woff2    14.94 kB
dist/assets/saira-semi-condensed-latin-ext-500-normal-DcBBbpUL.woff2    15.00 kB
dist/assets/saira-semi-condensed-latin-600-normal-CyCQx_HP.woff         15.71 kB
dist/assets/saira-semi-condensed-latin-500-normal-DNw5c0PQ.woff         15.78 kB
dist/assets/saira-semi-condensed-latin-400-normal-GbyiKxUx.woff         15.86 kB
dist/assets/saira-semi-condensed-latin-600-normal-BlCmTINa.woff2        18.22 kB
dist/assets/saira-semi-condensed-latin-500-normal-OkPZo89y.woff2        18.26 kB
dist/assets/saira-semi-condensed-latin-400-normal-DBFPQfhT.woff2        18.33 kB
dist/assets/View-CKZXWWMe.css                                            0.40 kB │ gzip:  0.26 kB
dist/assets/View-GfvHuV02.css                                            0.40 kB │ gzip:  0.26 kB
dist/assets/View-B6pN-6rI.css                                            0.55 kB │ gzip:  0.31 kB
dist/assets/SystemChip-D3GM7gCR.css                                      0.61 kB │ gzip:  0.31 kB
dist/assets/View-DI0And5n.css                                            0.93 kB │ gzip:  0.37 kB
dist/assets/PresetPicker-D8iBmlpT.css                                    1.05 kB │ gzip:  0.41 kB
dist/assets/View-W1ScxR78.css                                            1.10 kB │ gzip:  0.42 kB
dist/assets/View-B1E7xiYe.css                                            1.43 kB │ gzip:  0.52 kB
dist/assets/View-ggjitJdN.css                                            1.43 kB │ gzip:  0.48 kB
dist/assets/View-CEILl0y0.css                                            1.56 kB │ gzip:  0.55 kB
dist/assets/View-BjzciPvw.css                                            1.74 kB │ gzip:  0.58 kB
dist/assets/View-BBLqak4E.css                                            1.93 kB │ gzip:  0.58 kB
dist/assets/View-CSBfDQgw.css                                            2.13 kB │ gzip:  0.65 kB
dist/assets/View-D5pn8kgE.css                                            2.32 kB │ gzip:  0.67 kB
dist/assets/SearchKit-DRP9ZseB.css                                       2.40 kB │ gzip:  0.71 kB
dist/assets/View-B0t8nEPl.css                                            2.55 kB │ gzip:  0.73
```

## Stderr

Bytes: 0
Truncated: no

```text
```

