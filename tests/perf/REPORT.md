# Performance report

Built book: 46 files, 1662 KB uncompressed, 473 KB if served gzip-compressed.
Largest page: print.html, 727.2 KB (185.5 KB gzip).

Bytes are as transferred; the modem profiles are served gzip-compressed.

| Profile | Pages | Median bytes | Max requests | Median first paint (ms) | Max first paint (ms) | Median ready (ms) | Max ready (ms) | Worst calculator (ms) | Longest task (ms) | Max JS heap (KB) |
|---|---|---|---|---|---|---|---|---|---|---|
| modern | 39 | 50.1 KB | 7 | 56 | 112 | 36 | 67 | 2.2 | 0 | 1815 |
| old PC, DSL | 39 | 50.1 KB | 7 | 700 | 1108 | 700 | 1884 | 34.7 | 730 | 1818 |
| old PC, 56k modem | 4 | 35.7 KB | 7 | 1844 | 1992 | 5944 | 7616 | 34.5 | 495 | 1306 |
| old PC, 56k, cached | 4 | 21.9 KB | 7 | 492 | 532 | 3595 | 4926 | 16.5 | 203 | 1729 |

Budgets: page 128 KB and 12 requests; first paint {"modern":300,"old PC, DSL":1500,"old PC, 56k modem":4000,"old PC, 56k, cached":4000} ms; ready {"modern":500,"old PC, DSL":3000,"old PC, 56k modem":10000,"old PC, 56k, cached":6000} ms; calculator 100 ms and longest task 1000 ms on the slowed CPU.

All pages within budget.
