---
created_at: "2026-10-17T00:28:51.948139818+00:00"
id: "astromech-yqub"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:28:27.672214689+00:00"
command: "bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-970a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-970a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App gates: pnpm install --frozen-lockfile, lint, check, build, test"
updated_at: "2026-10-17T00:28:51.948585952+00:00"
---

## Summary

App gates: pnpm install --frozen-lockfile, lint, check, build, test

## Command

```console
bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 10049
Truncated: yes

```text
Lockfile is up to date, resolution step is skipped
Already up to date

Done in 2.2s using pnpm v10.34.6

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-task-astromech-970a/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-task-astromech-970a/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792196922158 START "/home/supadava/projects/astromech-wt-task-astromech-970a/app"
1792196922159 COMPLETED 346 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@0.1.5 build /home/supadava/projects/astromech-wt-task-astromech-970a/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 295 modules transformed.
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
dist/assets/View-DruMCGqz.css                                            1.22 kB │ gzip:  0.45 kB
dist/assets/View-B1E7xiYe.css                                            1.43 kB │ gzip:  0.52 kB
dist/assets/View-ggjitJdN.css                                            1.43 kB │ gzip:  0.48 kB
dist/assets/View-CEILl0y0.css                                            1.56 kB │ gzip:  0.55 kB
dist/assets/View-DZ9WtIcy.css                                            1.67 kB │ gzip:  0.55 kB
dist/assets/View-BjzciPvw.css                                            1.74 kB │ gzip:  0.58 kB
dist/assets/View-CCvET0VO.css                                            1.92 kB │ gzip:  0.61 kB
dist/assets/View-BBLqak4E.css                                            1.93 kB │ gzip:  0.58 kB
dist/assets/View-CSBfDQgw.css                                            2.13 kB │ gzip:  0.65 kB
dist/asset
```

## Stderr

Bytes: 0
Truncated: no

```text
```

