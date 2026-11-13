// Checks for assets/book.js: ES3 syntax, banned APIs, and calculation results.
// Run: node calc.test.js
var fs = require('fs');
var path = require('path');
var assert = require('assert');
var acorn = require('acorn');

var src = fs.readFileSync(path.join(__dirname, '..', '..', 'assets', 'book.js'), 'utf8');

// 1. Parses as ECMAScript 3.
acorn.parse(src, { ecmaVersion: 3 });

// 2. No APIs missing from IE6 / Firefox 1.5 era engines.
var banned = [
  /\.forEach\(/, /\.map\(/, /\.filter\(/, /\.reduce\(/, /\.trim\(/, /\bJSON\b/, /addEventListener/,
  /querySelector/, /getElementsByClassName/, /\.textContent/, /classList/, /Object\.keys/,
  /\[\s*\w+\s*\]\.indexOf|Array\.prototype\.indexOf/, /\blet\s/, /\bconst\s/, /=>/, /dataset/
];
var code = src.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^\s*\/\/.*$/mg, '');
banned.forEach(function (re) {
  assert(!re.test(code), 'banned API in book.js: ' + re);
});

var UB = require('../../assets/book.js');
var C = UB.calc;
function near(a, b, tol, msg) { assert(Math.abs(a - b) <= tol, msg + ': ' + a + ' vs ' + b); }

// Extraction: worked example from Chapter 2 (immersion, 15 g, 250 g water, 1.35 %).
near(C.extraction('imm', 15, 250, 1.35).ey, 22.81, 0.01, 'immersion EY');
near(C.extraction('perc', 20, 320, 1.30).ey, 20.8, 1e-9, 'percolation EY');
near(C.extraction('perc', 18, 40, 9).ey, 20.0, 1e-9, 'espresso EY');
assert.strictEqual(C.extraction('perc', 0, 100, 1), null, 'zero dose rejected');
// Brew-ratio line is the inverse of the EY formula.
near(C.tdsOnLine('imm', 15, 250, C.extraction('imm', 15, 250, 1.35).ey), 1.35, 1e-9, 'imm line');
near(C.tdsOnLine('perc', 20, 320, 20.8), 1.30, 1e-9, 'perc line');

// Recipe.
near(C.recipe(16, 320, 2).dose, 20, 1e-9, 'dose');
near(C.recipe(16, 320, 2).beverage, 280, 1e-9, 'beverage');

// Water.
var w = C.waterRecipe(0.12, 0, 0.068);
near(w.mg, 11.83, 0.01, 'Mg from Epsom');
near(w.hardness, 48.7, 0.1, 'hardness');
near(w.alkalinity, 40.5, 0.1, 'alkalinity');
near(C.waterIons(40.078, 0, 0, 0).hardness, 100.07, 0.05, '1 mmol Ca = 100 mg/L CaCO3');

// Triangle test: n = 12, k = 8 -> p about 0.019; 8 needed for significance.
var tri = C.triangle(12, 8);
near(tri.p, 0.0188, 0.0005, 'triangle p');
assert.strictEqual(tri.needed, 8, 'triangle threshold');
near(C.binomTail(10, 0, 1 / 3), 1, 1e-12, 'tail at 0');
// The one-pass critical value agrees with the direct search for many n.
for (var nn = 1; nn <= 120; nn++) {
  var direct = nn + 1;
  for (var kk = 0; kk <= nn; kk++) { if (C.binomTail(nn, kk, 1 / 3) <= 0.05) { direct = kk; break; } }
  assert.strictEqual(C.critical(nn, 1 / 3, 0.05), direct, 'critical value n=' + nn);
}

// Weibull inverse.
near(C.weibull(C.weibullTime(0.5, 120, 0.7), 120, 0.7), 0.5, 1e-12, 'weibull inverse');

// Roast log.
var rl = C.roastLog(250, 215, 8.5, 10.5, 180, 189, 1);
near(rl.loss, 14, 1e-9, 'mass loss');
near(rl.dtr, 19.05, 0.01, 'DTR');

// Kinetics: mass balance stays at 1, and a slower ramp peaks at a lower temperature.
var fast = C.kinetics(100, 15, 225, 100, 140, 0.3, 0.1, 800);
var slow = C.kinetics(100, 6, 225, 100, 140, 0.3, 0.1, 800);
assert(fast.peak.b > 0 && slow.peak.b > 0, 'B forms');
assert(slow.peak.T < fast.peak.T, 'slow ramp peaks cooler: ' + slow.peak.T + ' vs ' + fast.peak.T);
var coarse = C.kinetics(100, 10, 225, 100, 140, 0.3, 0.1, 50);
var fine = C.kinetics(100, 10, 225, 100, 140, 0.3, 0.1, 4000);
near(coarse.peak.b, fine.peak.b, 0.02, 'step-size insensitive');
console.log('fast peak', fast.peak, 'slow peak', slow.peak);

// Number helpers.
assert.strictEqual(UB.fix(1.005, 2), '1.01');
assert.strictEqual(UB.fix(-0.0001, 2), '0.00');
assert.strictEqual(UB.num(' 1,5 '), 1.5);
assert(isNaN(UB.num('abc')));

// Statistics (reference values from SciPy).
near(C.ncdf(1.96), 0.97500, 1e-5, 'normal cdf');
near(C.nquant(0.975), 1.959964, 1e-5, 'normal quantile');
near(C.fp(4, 2, 12), 0.046656, 1e-6, 'F p-value');
near(C.fp(3, 3, 20), 0.0548586, 1e-6, 'F p-value 2');
near(C.sampleSize(0.8, 0.05, 0.8), 24.53, 0.02, 'sample size');
var tp = C.triPower(24, 0.3, 0.05);
assert.strictEqual(tp.crit, 13, 'triangle critical value n=24');
near(tp.power, 0.55048, 1e-4, 'triangle power');
var av = C.anova([[6,7,6,8,7,6],[5,5,6,4,5,6],[7,8,8,7,9,8]]);
assert(av.p < 0.001 && av.df1 === 2 && av.df2 === 15, 'anova');
var lf = C.linfit([0.5,1,1.5,2,3],[0.52,1.01,1.53,2.02,3.05]);
near(lf.b, 1.01270, 1e-4, 'calibration slope (numpy polyfit)');
// Uncertainty: Chapter 0 worked example and self-check.
near(C.eyUnc(15, 0.1, 250, 1, 1.35, 0.02).rel, 0.0168, 2e-4, 'EY relative uncertainty');
near(C.eyUnc(20, 0.1, 320, 2, 1.30, 0.03).rel, 0.0244, 2e-4, 'EY uncertainty self-check');
// Equilibrium model (Liang et al. 2021, K = 0.717, Emax = 0.3).
near(C.equilibrium(0.717, 0.3, 16).ey, 21.51, 0.01, 'equilibrium EY');
near(C.equilibrium(0.717, 0.3, 16).tds, 1.3264, 1e-3, 'equilibrium TDS');
// Diffusion: 600 um, D = 2e-10 -> tau = (3e-4)^2/(pi^2 2e-10) = 45.6 s.
near(C.diffTau(600, 2e-10), 45.59, 0.05, 'diffusion tau');
near(C.diffTau(100, 2e-10), 1.267, 0.01, 'diffusion tau fines');
// Darcy default: about 2.2 mL/s.
near(C.darcy(7, 0.2, 58, 12, 0.3, 9).q, 2.248, 0.01, 'darcy flow');
// Carbonate speciation at pH 7.5, 40 mg/L alkalinity.
near(C.speciation(7.5, 40, 6.35, 10.33).hco3, 48.63, 0.05, 'bicarbonate');
near(C.blend(120, 180, 40, 10).tap, 3.333, 1e-3, 'blend');
var sv = C.sieve([[0,100,0.9],[100,200,1.1],[200,400,2.6],[400,600,6.8],[600,800,5.4],[800,1200,3.2]]);
near(sv.d50, 558.82, 0.01, 'sieve d50');
assert(sv.bins[0].nf > 0.9, 'fines dominate number');
near(sv.ssa, 14.629, 0.001, 'specific surface m2/kg (rho 1200 kg/m3)');
var wf = C.weibullFit([[6,0.05],[24,0.13],[48,0.20],[96,0.28],[168,0.35],[336,0.43]], 0.55);
assert(wf.beta > 0.4 && wf.beta < 1 && wf.r2 > 0.98, 'weibull fit ' + wf.beta + ' ' + wf.r2);
var ar = C.arrhenius(0.437, 40, 2.10, 60, 25);
near(ar.ea, 68.08, 0.01, 'arrhenius Ea');
near(C.moisture(11, 1000).db, 12.36, 0.01, 'dry basis');
var rs = C.roastSim(1, 11, 250, 0.006, 1.4, 215, 232);
assert(rs[rs.length - 1].drop && rs[rs.length - 1].t > 600 && rs[rs.length - 1].t < 960, 'roast sim drops in 10-16 min');
var pr6 = C.parallel(100, 6, 215, 90, 150, 130).ratio, pr15 = C.parallel(100, 15, 215, 90, 150, 130).ratio;
assert(pr15 > pr6, 'faster roast keeps more acid relative to bitter');
var ro = C.ror([[0,100],[60,110],[120,120],[180,130]], 3);
near(ro[1].ror, 10, 1e-9, 'ror');

console.log('book.js: all checks passed');
