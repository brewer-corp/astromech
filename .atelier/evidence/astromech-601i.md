---
created_at: "2026-10-17T17:42:18.374875646+00:00"
id: "astromech-601i"
evidence_type: "test"
commit: "d883b7ccd4ec7ebc6746f092262c1ca092144eeb"
result: "pass"
captured_at: "2026-10-17T17:41:41.065568708+00:00"
command: "sh -c 'python3 scripts/check-guide.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-mf5e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mf5e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "sh -c 'python3 scripts/check-guide.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
updated_at: "2026-10-17T17:42:18.376013121+00:00"
---

## Summary

sh -c 'python3 scripts/check-guide.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'

## Command

```console
sh -c 'python3 scripts/check-guide.py && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 10794
Truncated: yes

```text
checked 22 guide pages: 0 problem(s)

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-docs/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-docs/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792258911678 START "/home/supadava/projects/astromech-wt-docs/app"
1792258911680 COMPLETED 388 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@0.1.5 build /home/supadava/projects/astromech-wt-docs/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 332 modules transformed.
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
dist/assets/ExtLinks-CTtgu2nZ.css                                        0.49 kB │ gzip:  0.27 kB
dist/assets/View-B6pN-6rI.css                                            0.55 kB │ gzip:  0.31 kB
dist/assets/SystemChip-D3GM7gCR.css                                      0.61 kB │ gzip:  0.31 kB
dist/assets/View-BZO8uiKa.css                                            0.87 kB │ gzip:  0.35 kB
dist/assets/View-DI0And5n.css                                            0.93 kB │ gzip:  0.37 kB
dist/assets/PresetPicker-D8iBmlpT.css                                    1.05 kB │ gzip:  0.41 kB
dist/assets/View-W1ScxR78.css                                            1.10 kB │ gzip:  0.42 kB
dist/assets/View-ggjitJdN.css                                            1.43 kB │ gzip:  0.48 kB
dist/assets/View-CEILl0y0.css                                            1.56 kB │ gzip:  0.55 kB
dist/assets/View-DZ9WtIcy.css                                            1.67 kB │ gzip:  0.55 kB
dist/assets/View-Bc6MhH3O.css                                            1.81 kB │ gzip:  0.57 kB
dist/assets/View-B--zVyFt.css                                            1.84 kB │ gzip:  0.54 kB
dist/assets/View-CK1qDg6g.css                                            1.98 kB │ gzip:  0.59 kB
dist/assets/View-DbqLMeb1.css                                            2.31 kB │ gzip:  0.68 kB
dist/assets/View-D5pn8kgE.css                                            2.32 kB │ gzip:  0.67 kB
dist/assets/SearchKit-DRP9ZseB.css
```

## Stderr

Bytes: 0
Truncated: no

```text
```

