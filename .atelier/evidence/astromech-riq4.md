---
created_at: "2026-10-16T19:48:00.968210252+00:00"
id: "astromech-riq4"
evidence_type: "test"
commit: "bdcef370136c610768e12ec0233e1b97f9f53b66"
result: "pass"
captured_at: "2026-10-16T19:47:54.730585853+00:00"
command: "sh -c 'cd app && pnpm lint && pnpm check && pnpm build'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ngyt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ngyt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App: lint, svelte-check and build pass with the toast component"
updated_at: "2026-10-16T19:48:00.968457493+00:00"
---

## Summary

App: lint, svelte-check and build pass with the toast component

## Command

```console
sh -c 'cd app && pnpm lint && pnpm check && pnpm build'
```

Exit status: 0

## Stdout

Bytes: 6313
Truncated: yes

```text

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-feature-astromech-cmhv/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-feature-astromech-cmhv/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792180079142 START "/home/supadava/projects/astromech-wt-feature-astromech-cmhv/app"
1792180079143 COMPLETED 175 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@0.1.5 build /home/supadava/projects/astromech-wt-feature-astromech-cmhv/app
> vite build

vite v7.3.7 building client environment for production...
transforming...
✓ 190 modules transformed.
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
dist/assets/Nearby-CUiqJFqe.css                                          0.60 kB │ gzip:  0.27 kB
dist/assets/View-C3JxCiuI.css                                            0.61 kB │ gzip:  0.29 kB
dist/assets/About-BXs6Ftu4.css                                           0.71 kB │ gzip:  0.28 kB
dist/assets/View-DI0And5n.css                                            0.93 kB │ gzip:  0.37 kB
dist/assets/View-DSY_eBj1.css                                            0.96 kB │ gzip:  0.40 kB
dist/assets/Sheet-BY9pAWKS.css                                           1.25 kB │ gzip:  0.45 kB
dist/assets/View-BiZ53tpP.css                                            1.41 kB │ gzip:  0.53 kB
dist/assets/PresetPicker-B0pNwD8L.css                                    1.76 kB │ gzip:  0.59 kB
dist/assets/View-D6r_mUHA.css                                            2.19 kB │ gzip:  0.63 kB
dist/assets/View-D5pn8kgE.css                                            2.32 kB │ gzip:  0.67 kB
dist/assets/View-BqXNMEU3.css                                            2.68 kB │ gzip:  0.76 kB
dist/assets/KeybindsView-DgzzNmXd.css                                    3.67 kB │ gzip:  0.96 kB
dist/assets/View-DZbhUOTX.css                                            4.00 kB │ gzip:  1.07 kB
dist/assets/index-DLiPyWfH.css                                           6.96 kB │ gzip:  1.66 kB
dist/assets/Mockups-Be_rqh5x.css                                        11.84 kB │ gzip:  2.40 kB
dist/assets/snippet-oCPs1SxU.js                                          0.17 kB │ gzip:  0.16 kB
dist/assets/style-ypu7smkU.js                                            0.40 kB │ gzip:  0.28 kB
di
```

## Stderr

Bytes: 0
Truncated: no

```text
```

