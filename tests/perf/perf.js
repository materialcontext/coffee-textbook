// Performance test for the built book.
//
// Serves dist/ over HTTP and loads every page in Chromium under three
// profiles: an unthrottled modern machine, a 2006-class PC on DSL
// (CPU slowed 20x, 1.5 Mbit/s, 50 ms), and the same PC on a 56k modem.
// Records bytes, requests, first paint, load, main-thread work and the
// time each calculator takes to run, then checks the results against
// budgets. Throttling approximates old hardware; it does not replace
// testing on a real old machine.
//
// Run: node tests/perf/perf.js [dist] [--report report.md]
'use strict';
var http = require('http');
var fs = require('fs');
var path = require('path');
var zlib = require('zlib');
var chromium = require('playwright').chromium;

var dist = path.resolve(process.argv[2] && process.argv[2].indexOf('--') !== 0 ? process.argv[2] : 'dist');
var reportArg = process.argv.indexOf('--report');
var reportPath = reportArg > 0 ? process.argv[reportArg + 1] : null;

var PROFILES = [
  { name: 'modern', cpu: 1, net: null },
  { name: 'old PC, DSL', cpu: 20, net: { down: 1500, up: 256, latency: 50 } },
  // The modem profiles model the hosted web edition: the server compresses with gzip,
  // which every browser since Internet Explorer 4 accepts.
  { name: 'old PC, 56k modem', cpu: 20, net: { down: 56, up: 33, latency: 150 }, gzip: true },
  // Second page of a visit: book.css, dark.css and book.js already in the browser cache.
  { name: 'old PC, 56k, cached', cpu: 20, net: { down: 56, up: 33, latency: 150 }, gzip: true, warm: true }
];

// Budgets, in milliseconds or bytes. First paint is what a reader waits for;
// "ready" is when calculators and self-checks respond. The response-time
// limits follow Nielsen's thresholds: 0.1 s feels instant, 1 s keeps the
// reader's flow of thought.
var BUDGET = {
  pageBytes: 128 * 1024,          // one chapter page with its CSS, JS and figures, uncompressed
  pageRequests: 12,
  firstPaint: { 'modern': 300, 'old PC, DSL': 1500, 'old PC, 56k modem': 4000, 'old PC, 56k, cached': 4000 },
  ready: { 'modern': 500, 'old PC, DSL': 3000, 'old PC, 56k modem': 10000, 'old PC, 56k, cached': 6000 },
  calcMs: 100,                    // longest calculator or self-check run on the slowed CPU
  longTaskMs: 1000                // longest single main-thread task (parse and layout) on the slowed CPU
};

var TYPES = { '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'application/javascript',
  '.gif': 'image/gif', '.png': 'image/png', '.txt': 'text/plain; charset=utf-8', '.xhtml': 'application/xhtml+xml' };

// A static server like a typical host: cacheable for an hour, gzip on request
// for text files when the test asks for it (the modem profiles send x-perf-gzip).
function serve(root) {
  return http.createServer(function (req, res) {
    var p = path.join(root, decodeURIComponent(req.url.split('?')[0]));
    if (p.indexOf(root) !== 0 || !fs.existsSync(p) || fs.statSync(p).isDirectory()) { res.writeHead(404); res.end(); return; }
    var body = fs.readFileSync(p), type = TYPES[path.extname(p)] || 'application/octet-stream';
    var head = { 'Content-Type': type, 'Cache-Control': 'max-age=3600' };
    var gz = (req.headers['x-perf-gzip'] === '1') && /text|javascript/.test(type) && /gzip/.test(req.headers['accept-encoding'] || '');
    if (gz) { body = zlib.gzipSync(body, { level: 6 }); head['Content-Encoding'] = 'gzip'; }
    res.writeHead(200, head);
    res.end(body);
  });
}

function median(a) { var s = a.slice().sort(function (x, y) { return x - y; }); return s.length ? s[Math.floor(s.length / 2)] : 0; }
function max(a) { return a.length ? Math.max.apply(null, a) : 0; }

async function measure(browser, base, page, profile) {
  var ctx = await browser.newContext(profile.gzip ? { extraHTTPHeaders: { 'x-perf-gzip': '1' } } : {});
  var tab = await ctx.newPage();
  var cdp = await ctx.newCDPSession(tab);
  await cdp.send('Performance.enable');
  if (profile.cpu > 1) await cdp.send('Emulation.setCPUThrottlingRate', { rate: profile.cpu });
  if (profile.net) {
    await cdp.send('Network.enable');
    await cdp.send('Network.emulateNetworkConditions', {
      offline: false, latency: profile.net.latency,
      downloadThroughput: profile.net.down * 1024 / 8, uploadThroughput: profile.net.up * 1024 / 8
    });
  }
  await tab.addInitScript(function () {
    window.__long = 0;
    try {
      new PerformanceObserver(function (l) {
        l.getEntries().forEach(function (e) { if (e.duration > window.__long) window.__long = e.duration; });
      }).observe({ type: 'longtask', buffered: true });
    } catch (e) {}
  });
  if (profile.warm) await tab.goto(base + 'index.html', { waitUntil: 'load', timeout: 180000 });
  var bytes = 0, requests = 0;
  tab.on('requestfinished', async function (r) {
    requests++;
    try { var s = await r.sizes(); bytes += s.responseBodySize; } catch (e) {}
  });
  await tab.goto(base + page, { waitUntil: 'load', timeout: 180000 });
  await tab.waitForTimeout(50);
  var t = await tab.evaluate(function () {
    var nav = performance.getEntriesByType('navigation')[0];
    var fcp = performance.getEntriesByType('paint').filter(function (p) { return p.name === 'first-contentful-paint'; })[0];
    return { firstPaint: fcp ? fcp.startTime : null, dcl: nav.domContentLoadedEventEnd, load: nav.loadEventEnd,
      ready: (window.UB && UB.readyAt) ? UB.readyAt - performance.timeOrigin : nav.loadEventEnd, longTask: window.__long };
  });
  // Time every calculator and self-check, one by one.
  var calc = await tab.evaluate(function () {
    var forms = document.getElementsByTagName('form'), worst = 0, n = 0;
    for (var i = 0; i < forms.length; i++) {
      if (!forms[i].onsubmit) continue;
      var t0 = performance.now();
      forms[i].onsubmit();
      var dt = performance.now() - t0;
      if (dt > worst) worst = dt;
      n++;
    }
    return { worst: worst, n: n };
  });
  var m = await cdp.send('Performance.getMetrics');
  var metric = function (k) { var x = m.metrics.filter(function (e) { return e.name === k; })[0]; return x ? x.value : 0; };
  await ctx.close();
  return { page: page, profile: profile.name, gzip: !!profile.gzip, bytes: bytes, requests: requests, firstPaint: t.firstPaint, dcl: t.dcl,
    load: t.load, ready: t.ready, longTask: t.longTask, calcWorst: calc.worst, forms: calc.n,
    scriptMs: metric('ScriptDuration') * 1000, layoutMs: (metric('LayoutDuration') + metric('RecalcStyleDuration')) * 1000,
    heapKB: metric('JSHeapUsedSize') / 1024 };
}

function sizes(root) {
  var out = { files: 0, raw: 0, gzip: 0, html: [], largest: null };
  fs.readdirSync(root).forEach(function (f) {
    var p = path.join(root, f);
    if (fs.statSync(p).isDirectory()) return;
    var b = fs.readFileSync(p);
    var g = zlib.gzipSync(b, { level: 9 }).length;
    out.files++; out.raw += b.length; out.gzip += g;
    if (/\.html$/.test(f)) out.html.push({ f: f, raw: b.length, gzip: g });
  });
  out.html.sort(function (a, b) { return b.raw - a.raw; });
  return out;
}

(async function main() {
  var server = serve(dist).listen(0);
  var base = 'http://127.0.0.1:' + server.address().port + '/';
  var pages = fs.readdirSync(dist).filter(function (f) { return /\.html$/.test(f) && f !== 'print.html'; });
  var browser = await chromium.launch();
  var results = [], failures = [];
  for (var pi = 0; pi < PROFILES.length; pi++) {
    var prof = PROFILES[pi];
    // The modem profile is slow to run, so it covers the contents page and the three largest chapters.
    var list = prof.net && prof.net.down < 100
      ? ['index.html'].concat(sizes(dist).html.filter(function (h) { return !/solutions|print|terms|glossary/.test(h.f); }).slice(0, 3).map(function (h) { return h.f; }))
      : pages;
    for (var i = 0; i < list.length; i++) results.push(await measure(browser, base, list[i], prof));
  }
  await browser.close();
  server.close();

  var lines = [], sz = sizes(dist);
  lines.push('# Performance report', '');
  lines.push('Built book: ' + sz.files + ' files, ' + (sz.raw / 1024).toFixed(0) + ' KB uncompressed, ' + (sz.gzip / 1024).toFixed(0) + ' KB if served gzip-compressed.');
  lines.push('Largest page: ' + sz.html[0].f + ', ' + (sz.html[0].raw / 1024).toFixed(1) + ' KB (' + (sz.html[0].gzip / 1024).toFixed(1) + ' KB gzip).', '');
  lines.push('Bytes are as transferred; the modem profiles are served gzip-compressed.', '');
  lines.push('| Profile | Pages | Median bytes | Max requests | Median first paint (ms) | Max first paint (ms) | Median ready (ms) | Max ready (ms) | Worst calculator (ms) | Longest task (ms) | Max JS heap (KB) |');
  lines.push('|---|---|---|---|---|---|---|---|---|---|---|');
  PROFILES.forEach(function (p) {
    var r = results.filter(function (x) { return x.profile === p.name; });
    var col = function (k) { return r.map(function (x) { return x[k] || 0; }); };
    lines.push('| ' + [p.name, r.length, (median(col('bytes')) / 1024).toFixed(1) + ' KB', max(col('requests')),
      median(col('firstPaint')).toFixed(0), max(col('firstPaint')).toFixed(0), median(col('ready')).toFixed(0), max(col('ready')).toFixed(0),
      max(col('calcWorst')).toFixed(1), max(col('longTask')).toFixed(0), max(col('heapKB')).toFixed(0)].join(' | ') + ' |');
  });
  lines.push('');

  results.forEach(function (r) {
    var tag = r.profile + ' ' + r.page;
    if (!r.gzip && r.bytes > BUDGET.pageBytes) failures.push(tag + ': ' + r.bytes + ' bytes > ' + BUDGET.pageBytes);
    if (r.requests > BUDGET.pageRequests) failures.push(tag + ': ' + r.requests + ' requests > ' + BUDGET.pageRequests);
    if (r.firstPaint > BUDGET.firstPaint[r.profile]) failures.push(tag + ': first paint ' + r.firstPaint.toFixed(0) + ' ms > ' + BUDGET.firstPaint[r.profile]);
    if (BUDGET.ready[r.profile] && r.ready > BUDGET.ready[r.profile]) failures.push(tag + ': ready ' + r.ready.toFixed(0) + ' ms > ' + BUDGET.ready[r.profile]);
    if (r.profile !== 'modern' && r.calcWorst > BUDGET.calcMs) failures.push(tag + ': calculator ' + r.calcWorst.toFixed(1) + ' ms > ' + BUDGET.calcMs);
    if (r.profile !== 'modern' && r.longTask > BUDGET.longTaskMs) failures.push(tag + ': long task ' + r.longTask.toFixed(0) + ' ms > ' + BUDGET.longTaskMs);
  });
  lines.push('Budgets: page ' + BUDGET.pageBytes / 1024 + ' KB and ' + BUDGET.pageRequests + ' requests; first paint ' +
    JSON.stringify(BUDGET.firstPaint) + ' ms; ready ' + JSON.stringify(BUDGET.ready) + ' ms; calculator ' + BUDGET.calcMs +
    ' ms and longest task ' + BUDGET.longTaskMs + ' ms on the slowed CPU.', '');
  lines.push(failures.length ? '## Over budget\n\n' + failures.map(function (f) { return '- ' + f; }).join('\n') : 'All pages within budget.');
  var text = lines.join('\n') + '\n';
  if (reportPath) fs.writeFileSync(reportPath, text);
  fs.writeFileSync(path.join(__dirname, 'last-results.json'), JSON.stringify(results, null, 1));
  console.log(text);
  process.exit(failures.length ? 1 : 0);
})().catch(function (e) { console.error(e); process.exit(2); });
