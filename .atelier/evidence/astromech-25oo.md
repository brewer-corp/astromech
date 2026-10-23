---
created_at: "2026-10-23T01:18:54.396085110+00:00"
id: "astromech-25oo"
evidence_type: "test"
commit: "087f55f793516ea913b0c1dd0a06af520c9e99cd"
result: "pass"
captured_at: "2026-10-23T01:18:28.039985837+00:00"
command: "bash -c 'cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-5k55"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5k55"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Frontend: lint, svelte-check, build and the full vitest suite (PlotBar search, Enter picks the highlighted suggestion, plots from the current system, no From; Route page without the Plot panel; Resume)"
updated_at: "2026-10-23T01:18:54.397289315+00:00"
---

## Summary

Frontend: lint, svelte-check, build and the full vitest suite (PlotBar search, Enter picks the highlighted suggestion, plots from the current system, no From; Route page without the Plot panel; Resume)

## Command

```console
bash -c 'cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 10940
Truncated: yes

```text

> astromech-app@1.0.0-alpha.2 lint /home/supadava/projects/astromech/app
> eslint .


> astromech-app@1.0.0-alpha.2 check /home/supadava/projects/astromech/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792718317217 START "/home/supadava/projects/astromech/app"
1792718317219 COMPLETED 434 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@1.0.0-alpha.2 build /home/supadava/projects/astromech/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 372 modules transformed.
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
dist/assets/View-CZnIvwer.css                                            0.05 kB │ gzip:  0.07 kB
dist/assets/View-wUChb8L1.css                                            0.06 kB │ gzip:  0.08 kB
dist/assets/View-DnHRU7B1.css                                            0.12 kB │ gzip:  0.12 kB
dist/assets/SystemChip-Br2USrj8.css                                      0.53 kB │ gzip:  0.29 kB
dist/assets/View-BZO8uiKa.css                                            0.87 kB │ gzip:  0.35 kB
dist/assets/View-W1ScxR78.css                                            1.10 kB │ gzip:  0.42 kB
dist/assets/FindWant-t3Z0phcI.css                                        1.10 kB │ gzip:  0.44 kB
dist/assets/View-BkTuNZAK.css                                            1.21 kB │ gzip:  0.46 kB
dist/assets/View-DlZgCdLo.css                                            1.37 kB │ gzip:  0.46 kB
dist/assets/PresetPicker-C6B5zMYd.css                                    1.38 kB │ gzip:  0.49 kB
dist/assets/View-CEILl0y0.css                                            1.56 kB │ gzip:  0.55 kB
dist/assets/View-Bc6MhH3O.css                                            1.81 kB │ gzip:  0.57 kB
dist/assets/View-WK-2USlY.css                                            1.82 kB │ gzip:  0.56 kB
dist/assets/View-B--zVyFt.css                                            1.84 kB │ gzip:  0.54 kB
dist/assets/View-3r58C7hY.css                                            2.25 kB │ gzip:  0.63 kB
dist/assets/View-DbqLMeb1.css                                            2.31 kB │ gzip:  0.68 kB
dist/assets/View-CMnSxT-P.css                                            2.63 kB │ gzip:  0.75 kB
dist/assets/View-CvqiQX0c.css                                            3.02 kB �
```

## Stderr

Bytes: 349
Truncated: no

```text
[plugin vite:reporter] 
(!) /home/supadava/projects/astromech/app/src/modules/route/View.svelte is dynamically imported by /home/supadava/projects/astromech/app/src/modules/route/manifest.ts but also statically imported by /home/supadava/projects/astromech/app/src/modules/route/Rail.svelte, dynamic import will not move module into another chunk.
```

