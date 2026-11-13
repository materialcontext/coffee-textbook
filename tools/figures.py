"""Generate the book's data figures as small palette GIFs.

Every figure here is computed from an exact formula, a published model, or
clearly labelled teaching data, so it can be regenerated and checked.
Run: python3 tools/figures.py content/figures
"""
import io
import math
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from PIL import Image  # noqa: E402
from scipy import stats  # noqa: E402

OUT = Path(sys.argv[1] if len(sys.argv) > 1 else "content/figures")
OUT.mkdir(parents=True, exist_ok=True)
BROWN, TAN, GREY = "#7a3b10", "#c8915a", "#555555"
plt.rcParams.update({"font.size": 9, "font.family": "DejaVu Sans", "axes.spines.top": False, "axes.spines.right": False})


def save(fig, name):
    buf = io.BytesIO()
    fig.savefig(buf, format="png", dpi=80, bbox_inches="tight", facecolor="white")
    plt.close(fig)
    img = Image.open(buf).convert("RGB").quantize(colors=16, method=Image.Quantize.MEDIANCUT)
    path = OUT / name
    img.save(path, format="GIF", optimize=True)
    print(f"{name}: {path.stat().st_size} bytes, {img.size[0]}x{img.size[1]}")


def fig(w=6.4, h=3.6):
    return plt.subplots(figsize=(w, h))


# Chapter 0: calibration line with residuals (teaching data).
x = [0.5, 1.0, 1.5, 2.0, 3.0]
y = [0.52, 1.01, 1.53, 2.02, 3.05]
n = len(x)
mx, my = sum(x) / n, sum(y) / n
b = sum((xi - mx) * (yi - my) for xi, yi in zip(x, y)) / sum((xi - mx) ** 2 for xi in x)
a = my - b * mx
f, (ax1, ax2) = plt.subplots(2, 1, figsize=(6.4, 4.4), sharex=True, gridspec_kw={"height_ratios": [3, 1]})
ax1.plot(x, y, "o", color=BROWN)
ax1.plot([0, 3.2], [a, a + b * 3.2], color=GREY)
ax1.set_ylabel("Refractometer reading (%)")
ax1.text(0.1, 2.6, f"reading = {a:.4f} + {b:.4f} x known", color=GREY)
ax2.axhline(0, color=GREY, lw=0.8)
ax2.plot(x, [yi - a - b * xi for xi, yi in zip(x, y)], "s", color=BROWN)
ax2.set_ylabel("Residual")
ax2.set_xlabel("Known TDS of standard, made by mass (%)")
save(f, "ch00-calibration.gif")

# Chapter 1: triangle test null distribution, n = 12.
f, ax = fig()
ks = list(range(13))
pm = [stats.binom.pmf(k, 12, 1 / 3) for k in ks]
crit = next(k for k in ks if stats.binom.sf(k - 1, 12, 1 / 3) <= 0.05)
ax.bar(ks, pm, color=[BROWN if k >= crit else TAN for k in ks])
ax.set_xlabel("Correct answers out of 12 when tasters are guessing")
ax.set_ylabel("Probability")
ax.text(crit + 0.2, max(pm) * 0.6, f"{crit} or more: p = {stats.binom.sf(crit - 1, 12, 1 / 3):.3f}", color=BROWN)
save(f, "ch01-triangle.gif")

# Chapter 2A: equilibrium model of Liang, Chan and Ristenpart (2021), K = 0.717, Emax = 0.3.
f, ax = fig()
ke = 0.717 * 0.3
rs = [r / 10 for r in range(40, 251)]
ax.plot(rs, [100 * ke / (r + ke) for r in rs], color=BROWN, label="TDS (%)")
ax2 = ax.twinx()
ax2.plot(rs, [100 * ke] * len(rs), color=GREY, ls="--", label="EY (%)")
ax2.set_ylim(0, 30)
ax2.set_ylabel("Extraction yield (%)", color=GREY)
ax.set_xlabel("Brew ratio (water : coffee)")
ax.set_ylabel("TDS (%)", color=BROWN)
ax.set_title("Immersion at equilibrium: ratio sets strength, not yield", fontsize=9)
save(f, "ch02a-equilibrium.gif")

# Chapter 2A: diffusion from spheres (exact series solution of Fick's second law).
def released(t, a, d, terms=60):
    s = sum(math.exp(-(k * k) * math.pi ** 2 * d * t / (a * a)) / (k * k) for k in range(1, terms + 1))
    return 1 - 6 / math.pi ** 2 * s


f, ax = fig()
D = 2e-10
ts = [t for t in range(0, 601, 5)]
for dum, col in [(100, TAN), (400, BROWN), (800, GREY)]:
    a_ = dum * 1e-6 / 2
    ax.plot([t / 60 for t in ts], [released(max(t, 0.01), a_, D) for t in ts], color=col, label=f"{dum} um")
ax.set_xlabel("Time (min)")
ax.set_ylabel("Fraction of solubles released")
ax.legend(frameon=False)
ax.set_title("Diffusion out of spheres, D = 2e-10 m2/s (illustrative)", fontsize=9)
save(f, "ch02a-diffusion.gif")

# Chapter 2A: brewing chart with percolation ratio lines (beverage : dose).
f, ax = fig()
eys = [e / 10 for e in range(160, 241)]
for r in [14, 15, 16, 17, 18]:
    ax.plot(eys, [e / r for e in eys], color=TAN)
    ax.text(24.1, 24 / r, f"{r}:1", fontsize=8, color=GREY, va="center")
ax.set_xlim(16, 25.5)
ax.set_ylim(0.9, 1.6)
ax.set_xlabel("Extraction yield (%)")
ax.set_ylabel("TDS (%)")
ax.set_title("Percolation: TDS = EY x dose / beverage", fontsize=9)
save(f, "ch02a-chart.gif")

# Chapter 2B: espresso flow against pressure, Darcy line and the universal poroelastic curve
# reported by Waszkiewicz et al. (2026 preprint), Q = P(4 - 6P + 4P^2 - P^3) in normalised units.
f, ax = fig()
ps = [p / 100 for p in range(0, 101)]
ax.plot(ps, [min(4 * p, 1.6) for p in ps], color=GREY, ls="--", label="Darcy, rigid puck (slope 4)")
ax.plot(ps, [p * (4 - 6 * p + 4 * p * p - p ** 3) for p in ps], color=BROWN, label="Compacting puck")
ax.set_xlabel("Normalised pressure P / P_c")
ax.set_ylabel("Normalised flow Q / Q_c")
ax.set_ylim(0, 1.2)
ax.legend(frameon=False, loc="lower right")
save(f, "ch02b-poroelastic.gif")

# Chapter 3: carbonate speciation.
f, ax = fig()
k1, k2 = 10 ** -6.35, 10 ** -10.33
phs = [p / 20 for p in range(60, 281)]
for idx, (lab, col) in enumerate([("CO2(aq)", GREY), ("HCO3-", BROWN), ("CO3 2-", TAN)]):
    vals = []
    for ph in phs:
        h = 10 ** -ph
        den = h * h + h * k1 + k1 * k2
        vals.append([h * h / den, h * k1 / den, k1 * k2 / den][idx])
    ax.plot(phs, vals, color=col, label=lab)
ax.axvspan(6.5, 7.5, color="#eeeeee")
ax.axvspan(4.8, 5.2, color="#f3e3cf")
ax.text(4.85, 0.5, "coffee", fontsize=8, rotation=90)
ax.text(6.6, 0.5, "brew water", fontsize=8, rotation=90)
ax.set_xlabel("pH")
ax.set_ylabel("Fraction of dissolved carbonate")
ax.legend(frameon=False, loc="center right")
save(f, "ch03-speciation.gif")

# Chapter 4: number against volume weighting for a bimodal grind (illustrative lognormal mixture).
f, (ax1, ax2) = plt.subplots(1, 2, figsize=(6.6, 3.0))
ds = [10 ** (e / 100) for e in range(50, 330)]


def lognorm(d, mu, s):
    return math.exp(-((math.log(d) - math.log(mu)) ** 2) / (2 * s * s))


vol = [0.12 * lognorm(d, 40, 0.5) + 1.0 * lognorm(d, 700, 0.35) for d in ds]
num = [v / d ** 3 for v, d in zip(vol, ds)]
mx_ = max(num)
ax1.plot(ds, vol, color=BROWN)
ax1.set_xscale("log")
ax1.set_title("By volume", fontsize=9)
ax1.set_xlabel("Particle diameter (um)")
ax2.plot(ds, [x / mx_ for x in num], color=GREY)
ax2.set_xscale("log")
ax2.set_title("By number", fontsize=9)
ax2.set_xlabel("Particle diameter (um)")
for a_ in (ax1, ax2):
    a_.set_yticks([])
save(f, "ch04-weighting.gif")

# Chapter 5: Weibull release curves.
f, ax = fig()
ts = list(range(0, 721, 6))
for beta, col in [(0.5, TAN), (0.7, BROWN), (1.0, GREY)]:
    ax.plot([t / 24 for t in ts], [1 - math.exp(-((t / 120) ** beta)) for t in ts], color=col, label=f"beta = {beta}")
ax.set_xlabel("Days after roasting")
ax.set_ylabel("Fraction of gas released")
ax.set_title("Weibull release, tau = 120 h", fontsize=9)
ax.legend(frameon=False)
save(f, "ch05-weibull.gif")


# Chapter 7: teaching roast model (same equations and constants as the roast-sim calculator).
def roast(evap=True, exo=True, tair=250, ha=0.006):
    t, T, dry, water, x = 0, 25.0, 0.89, 0.11, 0.0
    out = []
    while t <= 1500:
        out.append((t / 60, T))
        if T >= 215:
            break
        kd = 0.0008 * math.exp(0.035 * (T - 100)) if (evap and T > 60) else 0
        e = min(water, kd * water)
        kr = 0.00015 * math.exp(0.03 * (T - 150)) if (exo and T > 150) else 0
        r = min(1 - x, kr * (1 - x))
        T += (ha * (tair - T) - 2257 * e + 232 * dry * r) / (dry * 1.4 + water * 4.18)
        water -= e
        x += r
        t += 1
    return out


f, ax = fig()
for kw, lab, col, ls in [
    ({}, "full model", BROWN, "-"),
    ({"exo": False}, "no reaction heat", GREY, "--"),
    ({"evap": False}, "no evaporation", TAN, ":"),
]:
    pts = roast(**kw)
    ax.plot([p[0] for p in pts], [p[1] for p in pts], color=col, ls=ls, label=lab)
ax.set_xlabel("Time (min)")
ax.set_ylabel("Bean temperature (C)")
ax.set_title("Lumped teaching model of a 1 kg batch", fontsize=9)
ax.legend(frameon=False)
save(f, "ch07-roastmodel.gif")


# Chapter 8: competing pathways at two heating rates (same model as the parallel calculator).
def parallel(rate, t0=100, tend=215, ea=(90, 150, 130)):
    R, tref, steps = 8.314e-3, 463.15, 600
    dur = (tend - t0) / rate
    dt = dur / steps
    s, a, g, l = 1.0, 0.0, 1.0, 0.0
    out = []
    for i in range(steps + 1):
        out.append((t0 + rate * i * dt, a, l))
        T = t0 + rate * (i + 0.5) * dt + 273.15
        k1 = 0.3 * math.exp(-(ea[0] / R) * (1 / T - 1 / tref))
        k2 = 0.1 * math.exp(-(ea[1] / R) * (1 / T - 1 / tref))
        k3 = 0.05 * math.exp(-(ea[2] / R) * (1 / T - 1 / tref))
        e1, e2 = math.exp(-k1 * dt), math.exp(-k2 * dt)
        a = a * e2 + s * k1 / (k2 - k1) * (e1 - e2)
        s *= e1
        l += g * (1 - math.exp(-k3 * dt))
        g *= math.exp(-k3 * dt)
    return out


f, ax = fig()
for rate, ls in [(6, "--"), (15, "-")]:
    pts = parallel(rate)
    ax.plot([p[0] for p in pts], [p[1] for p in pts], color=BROWN, ls=ls, label=f"acids, {rate} C/min")
    ax.plot([p[0] for p in pts], [p[2] for p in pts], color=GREY, ls=ls, label=f"lactones, {rate} C/min")
ax.set_xlabel("Bean temperature (C)")
ax.set_ylabel("Relative amount")
ax.set_title("Same drop temperature, different paths (teaching model)", fontsize=9)
ax.legend(frameon=False, fontsize=8)
save(f, "ch08-pathways.gif")

# Chapter 8 and Appendix C: Arrhenius sensitivity.
f, ax = fig()
Ts = [t for t in range(150, 241)]
for ea, col in [(60, TAN), (100, BROWN), (150, GREY)]:
    ax.plot(Ts, [math.exp(-(ea * 1000 / 8.314) * (1 / (t + 273.15) - 1 / 463.15)) for t in Ts], color=col, label=f"Ea = {ea} kJ/mol")
ax.set_yscale("log")
ax.axvline(190, color="#cccccc", lw=0.8)
ax.set_xlabel("Temperature (C)")
ax.set_ylabel("k(T) / k(190 C)")
ax.legend(frameon=False)
save(f, "appc-arrhenius.gif")

# Chapter 6: green coffee composition ranges by species (dry basis), Caporaso et al. (2018) and the 2026 review.
f, ax = fig(6.4, 3.4)
rows = [("Polysaccharides", (34, 44), (48, 55)), ("Lipids", (15, 17), (7, 10)), ("Sucrose", (6.25, 8.45), (0.9, 4.85)),
        ("Caffeine", (0.9, 1.3), (1.51, 3.33)), ("Trigonelline", (0.6, 2.0), (0.6, 1.24))]
for i, (lab, ar, rb) in enumerate(rows):
    y = len(rows) - i
    ax.plot(ar, [y + 0.12] * 2, color=BROWN, lw=5, solid_capstyle="butt")
    ax.plot(rb, [y - 0.12] * 2, color=TAN, lw=5, solid_capstyle="butt")
ax.set_yticks([len(rows) - i for i in range(len(rows))])
ax.set_yticklabels([r[0] for r in rows])
ax.set_xscale("log")
ax.set_xticks([0.5, 1, 2, 5, 10, 20, 50])
ax.set_xticklabels(["0.5", "1", "2", "5", "10", "20", "50"])
ax.set_xlabel("Percent of dry matter (log scale)")
ax.plot([], [], color=BROWN, lw=5, label="Arabica")
ax.plot([], [], color=TAN, lw=5, label="Robusta")
ax.legend(frameon=False, loc="lower right")
save(f, "ch06-composition.gif")

# Chapter 6: heat budget to take 1 kg of green coffee (11 % moisture) from 20 to 210 C, cp values as in the roast simulator.
f, ax = fig(6.4, 2.6)
dry, water = 0.89, 0.11
parts = [("Dry matter, sensible", dry * 1.4 * 190), ("Water, sensible to 100 C", water * 4.18 * 80), ("Evaporation", water * 2257)]
left = 0
for i, ((lab, q), col) in enumerate(zip(parts, [BROWN, GREY, TAN])):
    ax.barh([0], [q], left=left, color=col, height=0.5)
    ax.text(left + q / 2, 0.4 if i != 1 else -0.62, f"{lab}\n{q:.0f} kJ", ha="center", fontsize=8)
    left += q
ax.set_xlim(0, left * 1.02)
ax.set_ylim(-0.8, 0.9)
ax.set_yticks([])
ax.set_xlabel("Heat required per kg of green coffee (kJ), excluding reaction heat and losses")
save(f, "ch06-heatbudget.gif")
