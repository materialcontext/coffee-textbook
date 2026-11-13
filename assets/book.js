/* book.js: ECMAScript 3 only. No libraries, no JSON, no addEventListener,
   no Array.prototype.indexOf/forEach, no textContent, no trailing commas.
   Runs in Internet Explorer 6, Firefox 1.5, Opera 9, Safari 2 and later.
   Pure calculations live in UB.calc so they can be tested outside a browser. */

var UB = {};

/* ---------- number helpers ---------- */

UB.num = function (s) {
  if (s === null || s === undefined) { return NaN; }
  s = String(s).replace(/^\s+|\s+$/g, '').replace(',', '.');
  if (s === '') { return NaN; }
  return Number(s);
};

/* Fixed decimals without relying on Number.prototype.toFixed rounding. */
UB.fix = function (x, d) {
  if (typeof x !== 'number' || isNaN(x) || !isFinite(x)) { return '-'; }
  var neg = x < 0, m = Math.pow(10, d), v, ip, fp;
  v = Math.round(Math.abs(x) * m * (1 + 1e-12)); /* absorbs binary representation error, e.g. 1.005 */
  ip = Math.floor(v / m);
  fp = String(v - ip * m);
  while (fp.length < d) { fp = '0' + fp; }
  return (neg && v !== 0 ? '-' : '') + ip + (d > 0 ? '.' + fp : '');
};

/* ---------- pure calculations ---------- */

UB.calc = {};

/* method: 'perc' or 'imm'. liquid: beverage mass (perc) or water mass (imm). tds in percent. */
UB.calc.extraction = function (method, dose, liquid, tds) {
  var t = tds / 100, dissolved;
  if (!(dose > 0) || !(liquid > 0) || !(tds >= 0) || !(t < 1)) { return null; }
  dissolved = method === 'imm' ? t * liquid / (1 - t) : t * liquid;
  return { ey: 100 * dissolved / dose, dissolved: dissolved, ratio: liquid / dose };
};

/* TDS (percent) that a brew of this method, dose and liquid would show at extraction yield ey (percent). */
UB.calc.tdsOnLine = function (method, dose, liquid, ey) {
  var s = (ey / 100) * dose;
  return method === 'imm' ? 100 * s / (liquid + s) : 100 * s / liquid;
};

UB.calc.recipe = function (ratio, water, lrr) {
  if (!(ratio > 0) || !(water > 0) || !(lrr >= 0)) { return null; }
  var dose = water / ratio;
  return { dose: dose, beverage: water - lrr * dose };
};

UB.calc.waterIons = function (ca, mg, na, hco3) {
  return {
    hardness: 2.497 * ca + 4.118 * mg,
    caHardness: 2.497 * ca,
    alkalinity: 0.820 * hco3,
    na: na
  };
};

UB.calc.waterRecipe = function (epsom, cacl, soda) {
  var mg = epsom * 1000 * 24.305 / 246.47,
      ca = cacl * 1000 * 40.078 / 147.01,
      na = soda * 1000 * 22.990 / 84.007,
      hco3 = soda * 1000 * 61.017 / 84.007,
      so4 = epsom * 1000 * 96.06 / 246.47,
      cl = cacl * 1000 * 70.906 / 147.01,
      ions = UB.calc.waterIons(ca, mg, na, hco3);
  ions.mg = mg; ions.ca = ca; ions.hco3 = hco3; ions.so4 = so4; ions.cl = cl;
  return ions;
};

/* P(X >= k), X ~ Binomial(n, p). */
UB.calc.binomTail = function (n, k, p) {
  var j, term, sum = 0;
  if (k <= 0) { return 1; }
  if (k > n) { return 0; }
  term = Math.pow(1 - p, n);            /* P(X = 0) */
  for (j = 0; j <= n; j++) {
    if (j >= k) { sum += term; }
    term = term * (n - j) / (j + 1) * p / (1 - p);
  }
  return sum > 1 ? 1 : sum;
};

/* Smallest k with P(X >= k) <= alpha, in one pass: O(n) instead of O(n^2). */
UB.calc.critical = function (n, p, alpha) {
  var pmf = [], term = Math.pow(1 - p, n), j, tail = 0;
  for (j = 0; j <= n; j++) {
    pmf[j] = term;
    term = term * (n - j) / (j + 1) * p / (1 - p);
  }
  for (j = n; j >= 0; j--) {
    tail += pmf[j];
    if (tail > alpha) { return j + 1; }
  }
  return 0;
};

UB.calc.triangle = function (n, k) {
  return { p: UB.calc.binomTail(n, k, 1 / 3), needed: UB.calc.critical(n, 1 / 3, 0.05) };
};

UB.calc.weibull = function (t, tau, beta) {
  return 1 - Math.exp(-Math.pow(t / tau, beta));
};

UB.calc.weibullTime = function (f, tau, beta) {
  return tau * Math.pow(-Math.log(1 - f), 1 / beta);
};

UB.calc.roastLog = function (green, roasted, fc, drop, t1, t2, dt) {
  return {
    loss: 100 * (1 - roasted / green),
    dev: drop - fc,
    dtr: 100 * (drop - fc) / drop,
    ror: (t2 - t1) / dt
  };
};

/* Consecutive first-order reactions A -> B -> C under a linear temperature ramp.
   Rate constants follow Arrhenius, referenced to 190 C. Each step uses the exact
   solution for constant k, which stays stable for any step size. */
UB.calc.kinetics = function (t0, rate, tend, ea1, ea2, k1ref, k2ref, steps) {
  var R = 8.314e-3, Tref = 463.15, dur = (tend - t0) / rate, dt, i, t, T, k1, k2,
      a = 1, b = 0, e1, e2, rows = [], peak = { b: 0, t: 0, T: t0 }, every;
  if (!(rate > 0) || !(tend > t0)) { return null; }
  steps = steps || 400;
  dt = dur / steps;
  every = Math.max(1, Math.round(steps / 20));
  for (i = 0; i <= steps; i++) {
    t = i * dt;
    T = t0 + rate * t;
    if (i % every === 0 || i === steps) { rows[rows.length] = { t: t, T: T, a: a, b: b }; }
    if (b > peak.b) { peak = { b: b, t: t, T: T }; }
    if (i === steps) { break; }
    T = t0 + rate * (t + dt / 2) + 273.15;
    k1 = k1ref * Math.exp(-(ea1 / R) * (1 / T - 1 / Tref));
    k2 = k2ref * Math.exp(-(ea2 / R) * (1 / T - 1 / Tref));
    e1 = Math.exp(-k1 * dt);
    e2 = Math.exp(-k2 * dt);
    if (Math.abs(k2 - k1) < 1e-12) {
      b = b * e2 + a * k1 * dt * e1;
    } else {
      b = b * e2 + a * k1 / (k2 - k1) * (e1 - e2);
    }
    a = a * e1;
  }
  return { rows: rows, peak: peak, end: { a: a, b: b }, duration: dur };
};

/* ---------- DOM helpers ---------- */

UB.hasClass = function (el, c) {
  return (' ' + el.className + ' ').indexOf(' ' + c + ' ') >= 0;
};

UB.val = function (f, name) {
  var el = f.elements[name];
  if (!el) { return ''; }
  if (el.options) { return el.options[el.selectedIndex].value; }
  return el.value;
};

UB.n = function (f, name) { return UB.num(UB.val(f, name)); };

UB.row = function (label, value) {
  return '<tr><th>' + label + '</th><td>' + value + '</td></tr>';
};

UB.table = function (rows) { return '<table>' + rows + '</table>'; };

UB.bad = function () { return '<p class="wrong">Check the inputs: every field needs a positive number.</p>'; };

/* ---------- widgets: each returns HTML for the output area ---------- */

UB.KINDS = {};

UB.KINDS.extraction = function (f) {
  var m = UB.val(f, 'method'), dose = UB.n(f, 'dose'), liq = UB.n(f, 'liquid'),
      tds = UB.n(f, 'tds'), r = UB.calc.extraction(m, dose, liq, tds),
      g = UB.val(f, 'grid').split(','), tlo = Number(g[0]), thi = Number(g[1]), ts = Number(g[2]),
      elo = Number(g[3]), ehi = Number(g[4]), es = Number(g[5]),
      nr = Math.round((thi - tlo) / ts), nc = Math.round((ehi - elo) / es), i, j, cell, row, ey;
  for (i = 0; i <= nr; i++) {
    for (j = 0; j <= nc; j++) {
      cell = document.getElementById(f.id + '-c-' + i + '-' + j);
      if (cell) { cell.className = ''; }
    }
  }
  if (!r) { return UB.bad(); }
  for (j = 0; j <= nc; j++) {
    ey = elo + j * es;
    row = Math.round((thi - UB.calc.tdsOnLine(m, dose, liq, ey)) / ts);
    cell = document.getElementById(f.id + '-c-' + row + '-' + j);
    if (cell) { cell.className = 'line'; }
  }
  row = Math.round((thi - tds) / ts);
  j = Math.round((r.ey - elo) / es);
  cell = document.getElementById(f.id + '-c-' + row + '-' + j);
  if (cell) { cell.className = 'hit'; }
  return UB.table(
    UB.row('Extraction yield', UB.fix(r.ey, 1) + ' %') +
    UB.row('Dissolved solids in cup', UB.fix(r.dissolved, 2) + ' g') +
    UB.row(m === 'imm' ? 'Brew ratio (water : coffee)' : 'Beverage ratio (beverage : coffee)', UB.fix(r.ratio, 1) + ' : 1')
  ) + (cell ? '' : '<p class="small">This brew lies outside the chart.</p>');
};

UB.KINDS.recipe = function (f) {
  var r = UB.calc.recipe(UB.n(f, 'ratio'), UB.n(f, 'water'), UB.n(f, 'lrr'));
  if (!r) { return UB.bad(); }
  return UB.table(UB.row('Dose', UB.fix(r.dose, 1) + ' g') + UB.row('Expected beverage', UB.fix(r.beverage, 0) + ' g'));
};

UB.waterTable = function (w) {
  return UB.table(
    UB.row('Total hardness (as CaCO<sub>3</sub>)', UB.fix(w.hardness, 0) + ' mg/L') +
    UB.row('Calcium hardness (as CaCO<sub>3</sub>)', UB.fix(w.caHardness, 0) + ' mg/L <span class="small">(SCA 2009 target 68, range 17 to 85)</span>') +
    UB.row('Alkalinity (as CaCO<sub>3</sub>)', UB.fix(w.alkalinity, 0) + ' mg/L <span class="small">(SCA 2009 target about 40)</span>') +
    UB.row('Hardness : alkalinity', w.alkalinity > 0 ? UB.fix(w.hardness / w.alkalinity, 2) : '-')
  );
};

UB.KINDS['water-ions'] = function (f) {
  var w = UB.calc.waterIons(UB.n(f, 'ca'), UB.n(f, 'mg'), UB.n(f, 'na'), UB.n(f, 'hco3'));
  if (isNaN(w.hardness) || isNaN(w.alkalinity)) { return UB.bad(); }
  return UB.waterTable(w);
};

UB.KINDS['water-recipe'] = function (f) {
  var w = UB.calc.waterRecipe(UB.n(f, 'epsom'), UB.n(f, 'cacl'), UB.n(f, 'soda'));
  if (isNaN(w.hardness) || isNaN(w.alkalinity)) { return UB.bad(); }
  return UB.table(
    UB.row('Mg<sup>2+</sup>', UB.fix(w.mg, 1) + ' mg/L') +
    UB.row('Ca<sup>2+</sup>', UB.fix(w.ca, 1) + ' mg/L') +
    UB.row('Na<sup>+</sup>', UB.fix(w.na, 1) + ' mg/L') +
    UB.row('HCO<sub>3</sub><sup>&minus;</sup>', UB.fix(w.hco3, 1) + ' mg/L') +
    UB.row('SO<sub>4</sub><sup>2&minus;</sup>', UB.fix(w.so4, 1) + ' mg/L') +
    UB.row('Cl<sup>&minus;</sup>', UB.fix(w.cl, 1) + ' mg/L')
  ) + UB.waterTable(w);
};

UB.KINDS.triangle = function (f) {
  var n = Math.round(UB.n(f, 'n')), k = Math.round(UB.n(f, 'k')), r;
  if (!(n > 0) || !(k >= 0) || k > n) { return UB.bad(); }
  r = UB.calc.triangle(n, k);
  return UB.table(
    UB.row('p-value, P(X &ge; ' + k + ')', UB.fix(r.p, 4)) +
    UB.row('Significant at 0.05?', r.p <= 0.05 ? 'yes' : 'no') +
    UB.row('Correct answers needed out of ' + n, r.needed <= n ? String(r.needed) : 'not reachable')
  );
};

UB.KINDS.degassing = function (f) {
  var tau = UB.n(f, 'tau'), beta = UB.n(f, 'beta'), minf = UB.n(f, 'minf'),
      times = [6, 24, 72, 168, 336, 720], rows = '', i, fr;
  if (!(tau > 0) || !(beta > 0) || !(minf >= 0)) { return UB.bad(); }
  for (i = 0; i < times.length; i++) {
    fr = UB.calc.weibull(times[i], tau, beta);
    rows += '<tr><td>' + times[i] + ' h (' + UB.fix(times[i] / 24, 1) + ' d)</td><td>' +
      UB.fix(100 * fr, 0) + ' %</td><td>' + UB.fix(minf * fr, 2) + ' %</td><td><div class="bar" style="width:' +
      Math.round(120 * fr) + 'px">&nbsp;</div></td></tr>';
  }
  return '<table><tr><th>Time after roast</th><th>Released</th><th>Mass lost</th><th></th></tr>' + rows + '</table>' +
    UB.table(UB.row('Half released at', UB.fix(UB.calc.weibullTime(0.5, tau, beta) / 24, 1) + ' days') +
      UB.row('90% released at', UB.fix(UB.calc.weibullTime(0.9, tau, beta) / 24, 1) + ' days'));
};

UB.KINDS.roastlog = function (f) {
  var r = UB.calc.roastLog(UB.n(f, 'green'), UB.n(f, 'roasted'), UB.n(f, 'fc'), UB.n(f, 'drop'),
    UB.n(f, 't1'), UB.n(f, 't2'), UB.n(f, 'dt'));
  if (isNaN(r.loss) || isNaN(r.dtr) || isNaN(r.ror)) { return UB.bad(); }
  return UB.table(
    UB.row('Mass loss', UB.fix(r.loss, 1) + ' %') +
    UB.row('Development time', UB.fix(r.dev, 2) + ' min') +
    UB.row('Development time ratio', UB.fix(r.dtr, 1) + ' %') +
    UB.row('Rate of rise', UB.fix(r.ror, 1) + ' &deg;C/min')
  );
};

UB.KINDS.kinetics = function (f) {
  var r = UB.calc.kinetics(UB.n(f, 't0'), UB.n(f, 'rate'), UB.n(f, 'tend'), UB.n(f, 'ea1'),
        UB.n(f, 'ea2'), UB.n(f, 'k1ref'), UB.n(f, 'k2ref'), 400), rows = '', i, x;
  if (!r) { return UB.bad(); }
  for (i = 0; i < r.rows.length; i++) {
    x = r.rows[i];
    rows += '<tr><td>' + UB.fix(x.t, 1) + '</td><td>' + UB.fix(x.T, 0) + '</td><td>' + UB.fix(x.b, 3) +
      '</td><td><div class="bar" style="width:' + Math.round(150 * x.b) + 'px">&nbsp;</div></td></tr>';
  }
  return UB.table(
    UB.row('Roast length', UB.fix(r.duration, 1) + ' min') +
    UB.row('Maximum B', UB.fix(r.peak.b, 3) + ' at ' + UB.fix(r.peak.t, 1) + ' min, ' + UB.fix(r.peak.T, 0) + ' &deg;C') +
    UB.row('B at end of roast', UB.fix(r.end.b, 3)) +
    UB.row('A remaining', UB.fix(r.end.a, 3))
  ) + '<table><tr><th>min</th><th>&deg;C</th><th>B</th><th></th></tr>' + rows + '</table>';
};

/* ---------- statistics and numerics ---------- */

/* Parse a textarea into rows of numbers; non-numeric leading words are kept as row.label. */
UB.rows = function (text) {
  var lines = String(text).split(/\r?\n/), out = [], i, parts, nums, j, v, label;
  for (i = 0; i < lines.length; i++) {
    parts = lines[i].replace(/^\s+|\s+$/g, '').split(/[\s,;]+/);
    if (parts.length === 1 && parts[0] === '') { continue; }
    nums = []; label = '';
    for (j = 0; j < parts.length; j++) {
      v = UB.num(parts[j]);
      if (isNaN(v)) { label = label ? label + ' ' + parts[j] : parts[j]; } else { nums[nums.length] = v; }
    }
    out[out.length] = { label: label, v: nums };
  }
  return out;
};

UB.calc.linfit = function (xs, ys) {
  var n = xs.length, mx = 0, my = 0, sxx = 0, sxy = 0, syy = 0, i, b, a, ss = 0, e;
  if (n < 2) { return null; }
  for (i = 0; i < n; i++) { mx += xs[i]; my += ys[i]; }
  mx /= n; my /= n;
  for (i = 0; i < n; i++) {
    sxx += (xs[i] - mx) * (xs[i] - mx); sxy += (xs[i] - mx) * (ys[i] - my); syy += (ys[i] - my) * (ys[i] - my);
  }
  if (sxx === 0) { return null; }
  b = sxy / sxx; a = my - b * mx;
  for (i = 0; i < n; i++) { e = ys[i] - a - b * xs[i]; ss += e * e; }
  return { a: a, b: b, r2: syy > 0 ? 1 - ss / syy : 1, s: n > 2 ? Math.sqrt(ss / (n - 2)) : NaN, n: n };
};

/* Standard normal cdf (Abramowitz and Stegun 26.2.17) and quantile (Acklam). */
UB.calc.ncdf = function (z) {
  var t = 1 / (1 + 0.2316419 * Math.abs(z)), d = 0.3989422804014327 * Math.exp(-z * z / 2),
      p = d * t * (0.319381530 + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
  return z >= 0 ? 1 - p : p;
};

UB.calc.nquant = function (p) {
  var a = [-39.69683028665376, 220.9460984245205, -275.9285104469687, 138.3577518672690, -30.66479806614716, 2.506628277459239],
      b = [-54.47609879822406, 161.5858368580409, -155.6989798598866, 66.80131188771972, -13.28068155288572],
      c = [-0.007784894002430293, -0.3223964580411365, -2.400758277161838, -2.549732539343734, 4.374664141464968, 2.938163982698783],
      d = [0.007784695709041462, 0.3224671290700398, 2.445134137142996, 3.754408661907416],
      q, r;
  if (p < 0.02425) {
    q = Math.sqrt(-2 * Math.log(p));
    return (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1);
  }
  if (p > 1 - 0.02425) { return -UB.calc.nquant(1 - p); }
  q = p - 0.5; r = q * q;
  return (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1);
};

UB.calc.lgamma = function (x) {
  var g = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5],
      y = x, tmp = x + 5.5, ser = 1.000000000190015, j;
  tmp -= (x + 0.5) * Math.log(tmp);
  for (j = 0; j < 6; j++) { y += 1; ser += g[j] / y; }
  return -tmp + Math.log(2.5066282746310005 * ser / x);
};

/* Regularised incomplete beta by continued fraction (Numerical Recipes betacf). */
UB.calc.ibeta = function (x, a, b) {
  var bt, cf;
  if (x <= 0) { return 0; }
  if (x >= 1) { return 1; }
  bt = Math.exp(UB.calc.lgamma(a + b) - UB.calc.lgamma(a) - UB.calc.lgamma(b) + a * Math.log(x) + b * Math.log(1 - x));
  cf = function (x, a, b) {
    var m, m2, aa, c = 1, d = 1 - (a + b) * x / (a + 1), h, del, qab = a + b, qap = a + 1, qam = a - 1;
    if (Math.abs(d) < 1e-30) { d = 1e-30; }
    d = 1 / d; h = d;
    for (m = 1; m <= 200; m++) {
      m2 = 2 * m;
      aa = m * (b - m) * x / ((qam + m2) * (a + m2));
      d = 1 + aa * d; if (Math.abs(d) < 1e-30) { d = 1e-30; }
      c = 1 + aa / c; if (Math.abs(c) < 1e-30) { c = 1e-30; }
      d = 1 / d; h *= d * c;
      aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
      d = 1 + aa * d; if (Math.abs(d) < 1e-30) { d = 1e-30; }
      c = 1 + aa / c; if (Math.abs(c) < 1e-30) { c = 1e-30; }
      d = 1 / d; del = d * c; h *= del;
      if (Math.abs(del - 1) < 3e-12) { break; }
    }
    return h;
  };
  if (x < (a + 1) / (a + b + 2)) { return bt * cf(x, a, b) / a; }
  return 1 - bt * cf(1 - x, b, a) / b;
};

/* Upper-tail p-value of F with (d1, d2) degrees of freedom. */
UB.calc.fp = function (f, d1, d2) {
  return UB.calc.ibeta(d2 / (d2 + d1 * f), d2 / 2, d1 / 2);
};

UB.calc.anova = function (groups) {
  var k = groups.length, n = 0, grand = 0, ssb = 0, ssw = 0, i, j, m, g, f;
  for (i = 0; i < k; i++) { for (j = 0; j < groups[i].length; j++) { grand += groups[i][j]; n++; } }
  if (k < 2 || n <= k) { return null; }
  grand /= n;
  for (i = 0; i < k; i++) {
    g = groups[i]; m = 0;
    for (j = 0; j < g.length; j++) { m += g[j]; }
    m /= g.length;
    ssb += g.length * (m - grand) * (m - grand);
    for (j = 0; j < g.length; j++) { ssw += (g[j] - m) * (g[j] - m); }
  }
  f = (ssb / (k - 1)) / (ssw / (n - k));
  return { f: f, df1: k - 1, df2: n - k, p: UB.calc.fp(f, k - 1, n - k), ssb: ssb, ssw: ssw };
};

UB.calc.triPower = function (n, pd, alpha) {
  var crit = UB.calc.critical(n, 1 / 3, alpha), pc = 1 / 3 + 2 / 3 * pd;
  return { crit: crit, pc: pc, power: crit <= n ? UB.calc.binomTail(n, crit, pc) : 0 };
};

UB.calc.sampleSize = function (d, alpha, power) {
  var z = UB.calc.nquant(1 - alpha / 2) + UB.calc.nquant(power);
  return 2 * (z / d) * (z / d);
};

UB.calc.eyUnc = function (dose, ddose, bev, dbev, tds, dtds) {
  var ey = tds * bev / dose, rel = Math.sqrt(Math.pow(dtds / tds, 2) + Math.pow(dbev / bev, 2) + Math.pow(ddose / dose, 2));
  return { ey: ey, rel: rel, abs: ey * rel };
};

UB.calc.equilibrium = function (k, emax, r) {
  var ke = k * emax;
  return { ey: 100 * ke, tds: 100 * ke / (r + ke) };
};

/* Time constant (s) of the slowest diffusion mode in a sphere of diameter d (micrometres). */
UB.calc.diffTau = function (dUm, D) {
  var a = dUm * 1e-6 / 2;
  return a * a / (Math.PI * Math.PI * D);
};

UB.calc.darcy = function (dpUm, eps, diaMm, lenMm, muMpas, pbar) {
  var d = dpUm * 1e-6, k = d * d * eps * eps * eps / (180 * (1 - eps) * (1 - eps)),
      area = Math.PI * Math.pow(diaMm * 1e-3 / 2, 2), q = k * area * pbar * 1e5 / (muMpas * 1e-3 * lenMm * 1e-3);
  return { k: k, q: q * 1e6 };
};

UB.calc.speciation = function (ph, alk, pka1, pka2) {
  var h = Math.pow(10, -ph), k1 = Math.pow(10, -pka1), k2 = Math.pow(10, -pka2), den = h * h + h * k1 + k1 * k2,
      a0 = h * h / den, a1 = h * k1 / den, a2 = k1 * k2 / den,
      eqPerL = alk / 50.04 / 1000, ct = eqPerL / (a1 + 2 * a2);
  return { a0: a0, a1: a1, a2: a2, ct: ct, hco3: ct * a1 * 61.017 * 1000, co2: ct * a0 * 44.01 * 1000 };
};

UB.calc.blend = function (tapalk, taphard, target, vol) {
  var f = target / tapalk;
  if (!(f >= 0) || f > 1) { return null; }
  return { f: f, tap: f * vol, ro: (1 - f) * vol, hard: f * taphard };
};

UB.calc.sieve = function (rows) {
  var tot = 0, i, r, out = [], cum = 0, ssa = 0, nsum = 0, d50 = NaN, prev = 0, mid, frac, rho = 1200;
  for (i = 0; i < rows.length; i++) { tot += rows[i][2]; }
  if (!(tot > 0)) { return null; }
  for (i = 0; i < rows.length; i++) {
    r = rows[i]; mid = (r[0] + r[1]) / 2; if (mid <= 0) { mid = r[1] / 2; }
    frac = r[2] / tot; nsum += frac / Math.pow(mid, 3);
    ssa += frac * 6 / (rho * mid * 1e-6);
    prev = cum; cum += frac;
    if (isNaN(d50) && cum >= 0.5) { d50 = r[0] + (r[1] - r[0]) * (0.5 - prev) / frac; }
    out[out.length] = { lo: r[0], hi: r[1], frac: frac, cum: cum, nf: frac / Math.pow(mid, 3) };
  }
  for (i = 0; i < out.length; i++) { out[i].nf /= nsum; }
  return { bins: out, d50: d50, ssa: ssa };
};

UB.calc.weibullFit = function (pts, total) {
  var xs = [], ys = [], i, f, fit;
  for (i = 0; i < pts.length; i++) {
    f = pts[i][1] / total;
    if (pts[i][0] > 0 && f > 0 && f < 1) { xs[xs.length] = Math.log(pts[i][0]); ys[ys.length] = Math.log(-Math.log(1 - f)); }
  }
  fit = UB.calc.linfit(xs, ys);
  if (!fit) { return null; }
  return { beta: fit.b, tau: Math.exp(-fit.a / fit.b), r2: fit.r2 };
};

UB.calc.arrhenius = function (k1, t1, k2, t2, t3) {
  var R = 8.314, T1 = t1 + 273.15, T2 = t2 + 273.15, T3 = t3 + 273.15,
      ea = R * Math.log(k2 / k1) / (1 / T1 - 1 / T2);
  return { ea: ea / 1000, k3: k1 * Math.exp(-(ea / R) * (1 / T3 - 1 / T1)) };
};

UB.calc.moisture = function (wbPct, mass) {
  var w = wbPct / 100;
  return { db: 100 * w / (1 - w), water: w * mass, dry: (1 - w) * mass };
};

/* Lumped roast model. Returns rows every 30 s until drop or 25 min. */
UB.calc.roastSim = function (mass, w0Pct, tair, ha, cp, tend, het) {
  var dt = 1, t = 0, T = 25, dry = mass * (1 - w0Pct / 100), water = mass * w0Pct / 100,
      x = 0, rows = [], Lv = 2257, cw = 4.18, kd, kr, heat, evap, react, prevT = T, ror = 0;
  while (t <= 1500) {
    if (t % 30 === 0) { rows[rows.length] = { t: t, T: T, w: 100 * water / (dry + water), x: x, ror: ror }; }
    if (T >= tend) { rows[rows.length] = { t: t, T: T, w: 100 * water / (dry + water), x: x, ror: ror, drop: true }; break; }
    kd = T > 60 ? 0.0008 * Math.exp(0.035 * (T - 100)) : 0;
    evap = Math.min(water, kd * water * dt);
    kr = T > 150 ? 0.00015 * Math.exp(0.03 * (T - 150)) : 0;
    react = Math.min(1 - x, kr * (1 - x) * dt);
    heat = ha * mass * (tair - T) * dt - Lv * evap + het * dry * react;
    T += heat / (dry * cp + water * cw);
    water -= evap; x += react; t += dt;
    if (t % 30 === 0) { ror = (T - prevT) * 2; prevT = T; }
  }
  return rows;
};

UB.calc.ror = function (pts, win) {
  var out = [], i, j, lo, hi, xs, ys, fit, h = Math.max(1, Math.floor(win / 2));
  for (i = 0; i < pts.length; i++) {
    lo = Math.max(0, i - h); hi = Math.min(pts.length - 1, i + h); xs = []; ys = [];
    for (j = lo; j <= hi; j++) { xs[xs.length] = pts[j][0] / 60; ys[ys.length] = pts[j][1]; }
    fit = UB.calc.linfit(xs, ys);
    out[out.length] = { t: pts[i][0], T: pts[i][1], ror: fit ? fit.b : NaN };
  }
  return out;
};

UB.calc.parallel = function (t0, rate, tend, ea1, ea2, ea3) {
  var R = 8.314e-3, Tref = 463.15, steps = 600, dur = (tend - t0) / rate, dt = dur / steps,
      s = 1, a = 0, g = 1, l = 0, i, T, k1, k2, k3, e1, e2, rows = [], every = 30;
  if (!(rate > 0) || !(tend > t0)) { return null; }
  for (i = 0; i <= steps; i++) {
    if (i % every === 0) { rows[rows.length] = { t: i * dt, T: t0 + rate * i * dt, a: a, l: l }; }
    if (i === steps) { break; }
    T = t0 + rate * (i + 0.5) * dt + 273.15;
    k1 = 0.3 * Math.exp(-(ea1 / R) * (1 / T - 1 / Tref));
    k2 = 0.1 * Math.exp(-(ea2 / R) * (1 / T - 1 / Tref));
    k3 = 0.05 * Math.exp(-(ea3 / R) * (1 / T - 1 / Tref));
    e1 = Math.exp(-k1 * dt); e2 = Math.exp(-k2 * dt);
    a = Math.abs(k2 - k1) < 1e-12 ? a * e2 + s * k1 * dt * e1 : a * e2 + s * k1 / (k2 - k1) * (e1 - e2);
    s *= e1;
    l += g * (1 - Math.exp(-k3 * dt)); g *= Math.exp(-k3 * dt);
  }
  return { rows: rows, a: a, l: l, ratio: l > 0 ? a / l : NaN, dur: dur };
};

/* ---------- renderers for the new calculators ---------- */

UB.txt = function (f, name) { return UB.rows(UB.val(f, name)); };

UB.KINDS.calibration = function (f) {
  var r = UB.txt(f, 'data'), xs = [], ys = [], i, fit;
  for (i = 0; i < r.length; i++) { if (r[i].v.length >= 2) { xs[xs.length] = r[i].v[0]; ys[ys.length] = r[i].v[1]; } }
  fit = UB.calc.linfit(xs, ys);
  if (!fit) { return UB.bad(); }
  return UB.table(UB.row('Intercept a', UB.fix(fit.a, 4)) + UB.row('Slope b', UB.fix(fit.b, 4)) +
    UB.row('R<sup>2</sup>', UB.fix(fit.r2, 5)) + UB.row('Residual SD', UB.fix(fit.s, 4)) + UB.row('Points', String(fit.n)));
};

UB.KINDS['ey-uncertainty'] = function (f) {
  var r = UB.calc.eyUnc(UB.n(f, 'dose'), UB.n(f, 'ddose'), UB.n(f, 'bev'), UB.n(f, 'dbev'), UB.n(f, 'tds'), UB.n(f, 'dtds'));
  if (isNaN(r.ey)) { return UB.bad(); }
  return UB.table(UB.row('Extraction yield', UB.fix(r.ey, 2) + ' &plusmn; ' + UB.fix(r.abs, 2) + ' %') + UB.row('Relative uncertainty', UB.fix(100 * r.rel, 2) + ' %'));
};

UB.KINDS['triangle-power'] = function (f) {
  var n = Math.round(UB.n(f, 'n')), r = UB.calc.triPower(n, UB.n(f, 'pd'), UB.n(f, 'alpha'));
  if (!(n > 0)) { return UB.bad(); }
  return UB.table(UB.row('Correct needed for significance', r.crit <= n ? String(r.crit) : 'not reachable') +
    UB.row('Probability correct if p<sub>d</sub> is true', UB.fix(r.pc, 3)) + UB.row('Power', UB.fix(r.power, 3)));
};

UB.KINDS.oav = function (f) {
  var r = UB.txt(f, 'data'), list = [], i, j, tmp, rows = '';
  for (i = 0; i < r.length; i++) { if (r[i].v.length >= 2 && r[i].v[1] > 0) { list[list.length] = { n: r[i].label || ('row ' + (i + 1)), o: r[i].v[0] / r[i].v[1] }; } }
  for (i = 1; i < list.length; i++) { tmp = list[i]; j = i - 1; while (j >= 0 && list[j].o < tmp.o) { list[j + 1] = list[j]; j--; } list[j + 1] = tmp; }
  for (i = 0; i < list.length; i++) { rows += '<tr><td>' + list[i].n + '</td><td>' + UB.fix(list[i].o, 0) + '</td><td>' + UB.fix(Math.log(list[i].o) / Math.LN10, 1) + '</td></tr>'; }
  return list.length ? '<table><tr><th>Compound</th><th>OAV</th><th>log<sub>10</sub> OAV</th></tr>' + rows + '</table>' : UB.bad();
};

UB.KINDS.anova = function (f) {
  var r = UB.txt(f, 'data'), g = [], i, a;
  for (i = 0; i < r.length; i++) { if (r[i].v.length) { g[g.length] = r[i].v; } }
  a = UB.calc.anova(g);
  if (!a) { return UB.bad(); }
  return UB.table(UB.row('F', UB.fix(a.f, 3)) + UB.row('Degrees of freedom', a.df1 + ', ' + a.df2) + UB.row('p-value', UB.fix(a.p, 4)) +
    UB.row('Significant at 0.05?', a.p <= 0.05 ? 'yes' : 'no'));
};

UB.KINDS.power = function (f) {
  var n = UB.calc.sampleSize(UB.n(f, 'd'), UB.n(f, 'alpha'), UB.n(f, 'power'));
  if (isNaN(n)) { return UB.bad(); }
  return UB.table(UB.row('Per group (normal approximation)', UB.fix(n, 1)) + UB.row('Round up to', String(Math.ceil(n) + 1)));
};

UB.KINDS.equilibrium = function (f) {
  var k = UB.n(f, 'k'), e = UB.n(f, 'emax'), lo = UB.n(f, 'rlo'), hi = UB.n(f, 'rhi'), r, rows = '', i, step;
  if (!(k > 0) || !(e > 0) || !(hi > lo) || !(lo > 0)) { return UB.bad(); }
  step = (hi - lo) / 6;
  for (i = 0; i <= 6; i++) { r = UB.calc.equilibrium(k, e, lo + i * step); rows += '<tr><td>' + UB.fix(lo + i * step, 1) + '</td><td>' + UB.fix(r.tds, 2) + '</td><td>' + UB.fix(r.ey, 1) + '</td></tr>'; }
  return '<table><tr><th>Ratio</th><th>TDS %</th><th>EY %</th></tr>' + rows + '</table>';
};

UB.KINDS.diffusion = function (f) {
  var d = UB.n(f, 'd'), D = UB.n(f, 'diff'), sizes = [50, 100, 200, 400, 800, 1200], rows = '', i, tau;
  if (!(d > 0) || !(D > 0)) { return UB.bad(); }
  tau = UB.calc.diffTau(d, D);
  for (i = 0; i < sizes.length; i++) { rows += '<tr><td>' + sizes[i] + '</td><td>' + UB.fix(UB.calc.diffTau(sizes[i], D), 1) + '</td></tr>'; }
  return UB.table(UB.row('&tau; for ' + d + ' &micro;m', UB.fix(tau, 1) + ' s (' + UB.fix(tau / 60, 2) + ' min)')) +
    '<table><tr><th>d (&micro;m)</th><th>&tau; (s)</th></tr>' + rows + '</table>';
};

UB.KINDS.darcy = function (f) {
  var r = UB.calc.darcy(UB.n(f, 'dp'), UB.n(f, 'eps'), UB.n(f, 'dia'), UB.n(f, 'len'), UB.n(f, 'mu'), UB.n(f, 'pbar'));
  if (isNaN(r.q)) { return UB.bad(); }
  return UB.table(UB.row('Permeability k', r.k.toExponential(2) + ' m<sup>2</sup>') + UB.row('Flow rate Q', UB.fix(r.q, 2) + ' mL/s'));
};

UB.KINDS.speciation = function (f) {
  var r = UB.calc.speciation(UB.n(f, 'ph'), UB.n(f, 'alk'), UB.n(f, 'pka1'), UB.n(f, 'pka2'));
  if (isNaN(r.a1)) { return UB.bad(); }
  return UB.table(UB.row('CO<sub>2</sub>(aq) + H<sub>2</sub>CO<sub>3</sub>', UB.fix(100 * r.a0, 1) + ' %') + UB.row('HCO<sub>3</sub><sup>&minus;</sup>', UB.fix(100 * r.a1, 2) + ' %') +
    UB.row('CO<sub>3</sub><sup>2&minus;</sup>', UB.fix(100 * r.a2, 3) + ' %') + UB.row('Bicarbonate', UB.fix(r.hco3, 1) + ' mg/L') + UB.row('Dissolved CO<sub>2</sub>', UB.fix(r.co2, 1) + ' mg/L'));
};

UB.KINDS.blend = function (f) {
  var r = UB.calc.blend(UB.n(f, 'tapalk'), UB.n(f, 'taphard'), UB.n(f, 'target'), UB.n(f, 'vol'));
  if (!r) { return '<p class="wrong">Target must be between zero and the tap alkalinity.</p>'; }
  return UB.table(UB.row('Tap water', UB.fix(r.tap, 2) + ' L (' + UB.fix(100 * r.f, 0) + ' %)') + UB.row('RO water', UB.fix(r.ro, 2) + ' L') + UB.row('Hardness of blend', UB.fix(r.hard, 0) + ' mg/L CaCO<sub>3</sub>'));
};

UB.KINDS.sieve = function (f) {
  var r = UB.txt(f, 'data'), rows = [], i, s, h = '';
  for (i = 0; i < r.length; i++) { if (r[i].v.length >= 3) { rows[rows.length] = r[i].v; } }
  s = UB.calc.sieve(rows);
  if (!s) { return UB.bad(); }
  for (i = 0; i < s.bins.length; i++) {
    h += '<tr><td>' + s.bins[i].lo + ' to ' + s.bins[i].hi + '</td><td>' + UB.fix(100 * s.bins[i].frac, 1) + '</td><td>' + UB.fix(100 * s.bins[i].cum, 1) + '</td><td>' + UB.fix(100 * s.bins[i].nf, 2) + '</td></tr>';
  }
  return '<table><tr><th>&micro;m</th><th>Volume %</th><th>Cumulative %</th><th>Number %</th></tr>' + h + '</table>' +
    UB.table(UB.row('Volume median d<sub>50</sub>', UB.fix(s.d50, 0) + ' &micro;m') + UB.row('Specific surface', UB.fix(s.ssa, 1) + ' m<sup>2</sup>/kg'));
};

UB.KINDS['weibull-fit'] = function (f) {
  var r = UB.txt(f, 'data'), pts = [], i, w;
  for (i = 0; i < r.length; i++) { if (r[i].v.length >= 2) { pts[pts.length] = r[i].v; } }
  w = UB.calc.weibullFit(pts, UB.n(f, 'total'));
  if (!w) { return UB.bad(); }
  return UB.table(UB.row('&beta;', UB.fix(w.beta, 3)) + UB.row('&tau;', UB.fix(w.tau, 1) + ' h (' + UB.fix(w.tau / 24, 1) + ' d)') + UB.row('R<sup>2</sup> of linearised fit', UB.fix(w.r2, 4)));
};

UB.KINDS.arrhenius = function (f) {
  var r = UB.calc.arrhenius(UB.n(f, 'k1'), UB.n(f, 't1'), UB.n(f, 'k2'), UB.n(f, 't2'), UB.n(f, 't3'));
  if (isNaN(r.ea)) { return UB.bad(); }
  return UB.table(UB.row('E<sub>a</sub>', UB.fix(r.ea, 1) + ' kJ/mol') + UB.row('k at T<sub>3</sub>', r.k3.toPrecision(3)) +
    UB.row('Half-life at T<sub>3</sub> if first order', UB.fix(Math.LN2 / r.k3, 1)));
};

UB.KINDS.moisture = function (f) {
  var r = UB.calc.moisture(UB.n(f, 'wb'), UB.n(f, 'mass'));
  if (isNaN(r.db)) { return UB.bad(); }
  return UB.table(UB.row('Dry basis', UB.fix(r.db, 2) + ' %') + UB.row('Water', UB.fix(r.water, 1) + ' g') + UB.row('Dry matter', UB.fix(r.dry, 1) + ' g'));
};

UB.KINDS['roast-sim'] = function (f) {
  var rows = UB.calc.roastSim(UB.n(f, 'mass'), UB.n(f, 'w0'), UB.n(f, 'tair'), UB.n(f, 'ha'), UB.n(f, 'cp'), UB.n(f, 'tend'), UB.n(f, 'het')), h = '', i, x, last;
  if (!rows.length || isNaN(rows[0].T)) { return UB.bad(); }
  for (i = 0; i < rows.length; i++) {
    x = rows[i];
    h += '<tr><td>' + UB.fix(x.t / 60, 1) + '</td><td>' + UB.fix(x.T, 0) + '</td><td>' + UB.fix(x.ror, 1) + '</td><td>' + UB.fix(x.w, 1) + '</td><td><div class="bar" style="width:' + Math.max(1, Math.round(x.T / 2)) + 'px">&nbsp;</div></td></tr>';
  }
  last = rows[rows.length - 1];
  return UB.table(UB.row('Time to drop', last.drop ? UB.fix(last.t / 60, 2) + ' min' : 'not reached in 25 min') + UB.row('Final moisture', UB.fix(last.w, 1) + ' %')) +
    '<table><tr><th>min</th><th>&deg;C</th><th>RoR</th><th>Water %</th><th></th></tr>' + h + '</table>';
};

UB.KINDS.ror = function (f) {
  var r = UB.txt(f, 'data'), pts = [], i, out, h = '';
  for (i = 0; i < r.length; i++) { if (r[i].v.length >= 2) { pts[pts.length] = r[i].v; } }
  out = UB.calc.ror(pts, Math.round(UB.n(f, 'win')));
  if (!out.length) { return UB.bad(); }
  for (i = 0; i < out.length; i++) { h += '<tr><td>' + out[i].t + '</td><td>' + out[i].T + '</td><td>' + UB.fix(out[i].ror, 1) + '</td></tr>'; }
  return '<table><tr><th>s</th><th>&deg;C</th><th>RoR (&deg;C/min)</th></tr>' + h + '</table>';
};

UB.KINDS.parallel = function (f) {
  var r = UB.calc.parallel(UB.n(f, 't0'), UB.n(f, 'rate'), UB.n(f, 'tend'), UB.n(f, 'ea1'), UB.n(f, 'ea2'), UB.n(f, 'ea3')), h = '', i, x;
  if (!r) { return UB.bad(); }
  for (i = 0; i < r.rows.length; i++) {
    x = r.rows[i];
    h += '<tr><td>' + UB.fix(x.t, 1) + '</td><td>' + UB.fix(x.T, 0) + '</td><td>' + UB.fix(x.a, 3) + '</td><td>' + UB.fix(x.l, 3) + '</td></tr>';
  }
  return UB.table(UB.row('Roast length', UB.fix(r.dur, 1) + ' min') + UB.row('Acids at drop', UB.fix(r.a, 3)) + UB.row('Bitter lactones at drop', UB.fix(r.l, 3)) + UB.row('Acid : bitter', UB.fix(r.ratio, 2))) +
    '<table><tr><th>min</th><th>&deg;C</th><th>Acids</th><th>Lactones</th></tr>' + h + '</table>';
};

/* ---------- quizzes ---------- */

UB.checkQuiz = function (f) {
  var key = UB.val(f, 'quiz-key'), n = UB.num(UB.val(f, 'quiz-n')), i, name, kind, ans, given,
      el, j, ok, fb, ex, score = 0;
  for (i = 0; i < n; i++) {
    name = key + '-' + i;
    kind = UB.val(f, name + '-kind');
    ans = UB.num(UB.val(f, name + '-ans'));
    given = NaN;
    el = f.elements[name];
    if (kind === 'choice') {
      if (el && el.length === undefined) { el = [el]; }
      for (j = 0; el && j < el.length; j++) {
        if (el[j].checked) { given = UB.num(el[j].value); }
      }
      ok = given === ans;
    } else {
      given = UB.num(el ? el.value : '');
      ok = !isNaN(given) && Math.abs(given - ans) <= UB.num(UB.val(f, name + '-tol')) + 1e-9;
    }
    fb = document.getElementById(f.id + '-' + i + '-fb');
    ex = document.getElementById(f.id + '-' + i + '-ex');
    if (!fb) { continue; }
    if (isNaN(given)) {
      fb.className = 'fb';
      fb.innerHTML = 'No answer given.';
    } else {
      if (ok) { score++; }
      fb.className = ok ? 'fb right' : 'fb wrong';
      fb.innerHTML = (ok ? 'Correct. ' : 'Not quite. ') + (ex ? ex.innerHTML : '');
    }
  }
  return score;
};

/* ---------- wiring ---------- */

UB.findButton = function (f, cls) {
  var ins = f.getElementsByTagName('input'), i;
  for (i = 0; i < ins.length; i++) {
    if (ins[i].type === 'button' && UB.hasClass(ins[i], cls)) { return ins[i]; }
  }
  return null;
};

UB.run = function (f, fn) {
  var out = document.getElementById(f.id + '-out');
  if (!out) { return; }
  try {
    out.innerHTML = fn(f);
  } catch (e) {
    out.innerHTML = '<p class="wrong">Could not calculate.</p>';
  }
};

UB.wireWidget = function (f) {
  var fn = UB.KINDS[UB.val(f, 'kind')], b = UB.findButton(f, 'calc');
  if (!fn || !b) { return; }
  b.onclick = function () { UB.run(f, fn); return false; };
  f.onsubmit = function () { UB.run(f, fn); return false; };
  UB.run(f, fn);
};

UB.wireQuiz = function (f) {
  var b = UB.findButton(f, 'check');
  if (!b) { return; }
  b.onclick = function () { UB.checkQuiz(f); return false; };
  f.onsubmit = function () { UB.checkQuiz(f); return false; };
};

UB.init = function () {
  var forms = document.forms, i;
  if (UB.started) { return; }
  UB.started = true;
  for (i = 0; i < forms.length; i++) {
    if (UB.hasClass(forms[i], 'w')) { UB.wireWidget(forms[i]); }
    else if (UB.hasClass(forms[i], 'quiz-form')) { UB.wireQuiz(forms[i]); }
  }
  UB.readyAt = new Date().getTime();
};

/* The script is loaded at the end of the body, so the forms already exist:
   start at once instead of waiting for every figure to download. */
if (typeof window !== 'undefined' && typeof document !== 'undefined') {
  if (document.body) { UB.init(); } else { window.onload = UB.init; }
}
if (typeof module !== 'undefined' && module.exports) { module.exports = UB; }
