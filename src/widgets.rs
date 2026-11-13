//! Interactive calculators.
//!
//! Each widget renders a plain HTML form plus the formula it computes, so a
//! reader without script can still work the numbers by hand. The script in
//! `assets/book.js` finds forms with class `w`, reads the hidden `kind`
//! field, and dispatches to the matching function there.
//!
//! Adding a widget: implement `Widget`, add it to `REGISTRY`, and add a
//! function with the same kind name to the `KINDS` table in `book.js`.

use crate::domain::WidgetSpec;
use crate::render::esc;

pub type Params = toml::Table;

pub trait Widget: Sync {
    fn kind(&self) -> &'static str;
    fn title(&self, p: &Params) -> String;
    /// Form fields, static explanation and output placeholders.
    fn body(&self, id: &str, p: &Params) -> Result<String, String>;
}

static REGISTRY: &[&dyn Widget] = &[
    &Extraction,
    &Recipe,
    &WaterIons,
    &WaterRecipe,
    &Triangle,
    &Degassing,
    &RoastLog,
    &Kinetics,
];

pub fn kinds() -> Vec<&'static str> {
    REGISTRY.iter().map(|w| w.kind()).chain(GENERIC.iter().map(|g| g.kind)).collect()
}

fn lookup(kind: &str) -> Option<&'static dyn Widget> {
    REGISTRY
        .iter()
        .copied()
        .find(|w| w.kind() == kind)
        .or_else(|| GENERIC.iter().find(|g| g.kind == kind).map(|g| g as &dyn Widget))
}

pub fn render(id: &str, spec: &WidgetSpec) -> Result<String, String> {
    let w = lookup(&spec.kind).ok_or_else(|| format!("unknown widget kind `{}`", spec.kind))?;
    Ok(format!(
        "<div class=\"widget\">\n<h3>{}</h3>\n<form class=\"w\" id=\"{id}\" action=\"#\">\n<div><input type=\"hidden\" name=\"kind\" value=\"{}\"></div>\n{}<p><input type=\"button\" class=\"calc\" value=\"Calculate\"></p>\n<div class=\"out\" id=\"{id}-out\"></div>\n</form>\n</div>\n",
        esc(&w.title(&spec.params)),
        w.kind(),
        w.body(id, &spec.params)?
    ))
}

// ---- small pure helpers ---------------------------------------------------

fn num(p: &Params, key: &str, default: f64) -> Result<f64, String> {
    match p.get(key) {
        None => Ok(default),
        Some(toml::Value::Float(f)) => Ok(*f),
        Some(toml::Value::Integer(i)) => Ok(*i as f64),
        Some(v) => Err(format!("parameter `{key}` must be a number, got {v}")),
    }
}

fn text<'a>(p: &'a Params, key: &str, default: &'a str) -> &'a str {
    p.get(key).and_then(|v| v.as_str()).unwrap_or(default)
}

fn field(id: &str, name: &str, label: &str, value: f64, unit: &str) -> String {
    format!(
        "<div class=\"f\"><label for=\"{id}-{name}\">{label}</label> <input type=\"text\" size=\"7\" name=\"{name}\" id=\"{id}-{name}\" value=\"{value}\"> {unit}</div>\n"
    )
}

fn select(id: &str, name: &str, label: &str, opts: &[(&str, &str)], chosen: &str) -> String {
    let mut h = format!("<div class=\"f\"><label for=\"{id}-{name}\">{label}</label> <select name=\"{name}\" id=\"{id}-{name}\">");
    for (v, l) in opts {
        let sel = if *v == chosen { " selected" } else { "" };
        h.push_str(&format!("<option value=\"{v}\"{sel}>{l}</option>"));
    }
    h.push_str("</select></div>\n");
    h
}

fn formula(s: &str) -> String {
    format!("<p class=\"formula\">{s}</p>\n")
}

/// Grid axis values from `lo` to `hi` inclusive in steps of `step`.
pub fn axis(lo: f64, hi: f64, step: f64) -> Vec<f64> {
    let n = ((hi - lo) / step).round() as usize;
    (0..=n).map(|i| lo + step * i as f64).collect()
}

// ---- widgets ---------------------------------------------------------------

pub struct Extraction;

impl Widget for Extraction {
    fn kind(&self) -> &'static str {
        "extraction"
    }
    fn title(&self, p: &Params) -> String {
        match text(p, "mode", "filter") {
            "espresso" => "Espresso extraction calculator".into(),
            _ => "Extraction calculator and brewing chart".into(),
        }
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let espresso = text(p, "mode", "filter") == "espresso";
        let (dose, liquid, tds) = if espresso { (18.0, 40.0, 9.0) } else { (15.0, 250.0, 1.35) };
        let method = if espresso { "perc" } else { text(p, "method", "perc") };
        let (tlo, thi, tstep) = if espresso { (6.0, 12.0, 0.5) } else { (1.0, 1.6, 0.05) };
        let (elo, ehi, estep) = (16.0, 24.0, if espresso { 1.0 } else { 0.5 });

        let mut h = String::new();
        h.push_str(&select(
            id,
            "method",
            "Method",
            &[("perc", "Percolation (pour-over, espresso)"), ("imm", "Full immersion")],
            method,
        ));
        h.push_str(&field(id, "dose", "Dry coffee dose", num(p, "dose", dose)?, "g"));
        h.push_str(&field(
            id,
            "liquid",
            "Beverage mass (percolation) or water mass (immersion)",
            num(p, "liquid", liquid)?,
            "g",
        ));
        h.push_str(&field(id, "tds", "Measured TDS", num(p, "tds", tds)?, "%"));
        h.push_str(&formula(
            "Percolation: EY = TDS &times; beverage mass / dose. Immersion: EY = TDS &times; water / ((1 &minus; TDS) &times; dose).",
        ));
        h.push_str(&format!(
            "<div><input type=\"hidden\" name=\"grid\" value=\"{tlo},{thi},{tstep},{elo},{ehi},{estep}\"></div>\n"
        ));
        // The chart: rows are TDS (high at top), columns are EY.
        let tds_axis: Vec<f64> = axis(tlo, thi, tstep).into_iter().rev().collect();
        let ey_axis = axis(elo, ehi, estep);
        h.push_str("<table class=\"bcc\" summary=\"Brewing chart: TDS against extraction yield\">\n<tr><th>TDS %</th>");
        for (c, ey) in ey_axis.iter().enumerate() {
            let label = if c % 2 == 0 { format!("{ey}") } else { String::new() };
            h.push_str(&format!("<th>{label}</th>"));
        }
        h.push_str("</tr>\n");
        for (r, t) in tds_axis.iter().enumerate() {
            h.push_str(&format!("<tr><th>{:.2}</th>", t));
            for c in 0..ey_axis.len() {
                h.push_str(&format!("<td id=\"{id}-c-{r}-{c}\">&nbsp;</td>"));
            }
            h.push_str("</tr>\n");
        }
        h.push_str(&format!(
            "<tr><th></th><th colspan=\"{}\">Extraction yield %</th></tr>\n</table>\n",
            ey_axis.len()
        ));
        h.push_str("<p class=\"small\">The marked cell is your brew. Shaded cells lie on your brew ratio: changing grind or time moves a brew along this line, changing the ratio moves the line. Zones are deliberately not drawn; see the chapter text.</p>\n");
        Ok(h)
    }
}

pub struct Recipe;

impl Widget for Recipe {
    fn kind(&self) -> &'static str {
        "recipe"
    }
    fn title(&self, _: &Params) -> String {
        "Recipe from a brew ratio".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "ratio", "Brew ratio (water : coffee)", num(p, "ratio", 16.0)?, ": 1"));
        h.push_str(&field(id, "water", "Brew water", num(p, "water", 300.0)?, "g"));
        h.push_str(&field(id, "lrr", "Liquid retained per gram of grounds", num(p, "lrr", 2.0)?, "g/g"));
        h.push_str(&formula("Dose = water / ratio. Beverage &asymp; water &minus; retained liquid &times; dose."));
        Ok(h)
    }
}

pub struct WaterIons;

impl Widget for WaterIons {
    fn kind(&self) -> &'static str {
        "water-ions"
    }
    fn title(&self, _: &Params) -> String {
        "Hardness and alkalinity from an ion analysis".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "ca", "Calcium, Ca<sup>2+</sup>", num(p, "ca", 20.0)?, "mg/L"));
        h.push_str(&field(id, "mg", "Magnesium, Mg<sup>2+</sup>", num(p, "mg", 8.0)?, "mg/L"));
        h.push_str(&field(id, "na", "Sodium, Na<sup>+</sup>", num(p, "na", 10.0)?, "mg/L"));
        h.push_str(&field(id, "hco3", "Bicarbonate, HCO<sub>3</sub><sup>&minus;</sup>", num(p, "hco3", 50.0)?, "mg/L"));
        h.push_str(&formula(
            "Hardness (as CaCO<sub>3</sub>) = 2.497 &times; Ca + 4.118 &times; Mg. Alkalinity (as CaCO<sub>3</sub>) = 0.820 &times; HCO<sub>3</sub>.",
        ));
        Ok(h)
    }
}

pub struct WaterRecipe;

impl Widget for WaterRecipe {
    fn kind(&self) -> &'static str {
        "water-recipe"
    }
    fn title(&self, _: &Params) -> String {
        "Brew water from distilled water and salts".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "epsom", "Epsom salt, MgSO<sub>4</sub>&middot;7H<sub>2</sub>O", num(p, "epsom", 0.12)?, "g per litre"));
        h.push_str(&field(id, "cacl", "Calcium chloride dihydrate, CaCl<sub>2</sub>&middot;2H<sub>2</sub>O", num(p, "cacl", 0.0)?, "g per litre"));
        h.push_str(&field(id, "soda", "Baking soda, NaHCO<sub>3</sub>", num(p, "soda", 0.068)?, "g per litre"));
        h.push_str(&formula(
            "Ion mg/L = salt g/L &times; 1000 &times; (ion molar mass / salt molar mass). Molar masses: MgSO<sub>4</sub>&middot;7H<sub>2</sub>O 246.47, CaCl<sub>2</sub>&middot;2H<sub>2</sub>O 147.01, NaHCO<sub>3</sub> 84.01, Mg 24.305, Ca 40.078, Na 22.990, HCO<sub>3</sub> 61.017 g/mol.",
        ));
        Ok(h)
    }
}

pub struct Triangle;

impl Widget for Triangle {
    fn kind(&self) -> &'static str {
        "triangle"
    }
    fn title(&self, _: &Params) -> String {
        "Triangle test significance".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "n", "Number of trials", num(p, "n", 12.0)?, ""));
        h.push_str(&field(id, "k", "Correct identifications", num(p, "k", 8.0)?, ""));
        h.push_str(&formula(
            "p = P(X &ge; k) for X ~ Binomial(n, 1/3) = sum over j from k to n of C(n, j) (1/3)<sup>j</sup> (2/3)<sup>n&minus;j</sup>.",
        ));
        Ok(h)
    }
}

pub struct Degassing;

impl Widget for Degassing {
    fn kind(&self) -> &'static str {
        "degassing"
    }
    fn title(&self, _: &Params) -> String {
        "CO2 release after roasting (Weibull model)".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "tau", "Scale time &tau;", num(p, "tau", 120.0)?, "h"));
        h.push_str(&field(id, "beta", "Shape &beta;", num(p, "beta", 0.7)?, ""));
        h.push_str(&field(id, "minf", "Total releasable gas", num(p, "minf", 0.6)?, "% of bean mass"));
        h.push_str(&formula(
            "Released fraction F(t) = 1 &minus; exp(&minus;(t/&tau;)<sup>&beta;</sup>). Time to reach fraction F: t = &tau; (&minus;ln(1 &minus; F))<sup>1/&beta;</sup>.",
        ));
        h.push_str("<p class=\"small\">Default parameters are illustrative, not measured values; fit your own from the jar experiment in the chapter.</p>\n");
        Ok(h)
    }
}

pub struct RoastLog;

impl Widget for RoastLog {
    fn kind(&self) -> &'static str {
        "roastlog"
    }
    fn title(&self, _: &Params) -> String {
        "Roast log arithmetic".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "green", "Green mass", num(p, "green", 250.0)?, "g"));
        h.push_str(&field(id, "roasted", "Roasted mass", num(p, "roasted", 215.0)?, "g"));
        h.push_str(&field(id, "fc", "First crack at", num(p, "fc", 8.5)?, "min"));
        h.push_str(&field(id, "drop", "Drop (end of roast) at", num(p, "drop", 10.5)?, "min"));
        h.push_str(&field(id, "t1", "Bean probe reading", num(p, "t1", 180.0)?, "&deg;C"));
        h.push_str(&field(id, "t2", "Reading one interval later", num(p, "t2", 189.0)?, "&deg;C"));
        h.push_str(&field(id, "dt", "Interval", num(p, "dt", 1.0)?, "min"));
        h.push_str(&formula(
            "Mass loss = 1 &minus; roasted/green. Development time = drop &minus; first crack; ratio = development / drop. Rate of rise = (T<sub>2</sub> &minus; T<sub>1</sub>) / interval.",
        ));
        Ok(h)
    }
}

pub struct Kinetics;

impl Widget for Kinetics {
    fn kind(&self) -> &'static str {
        "kinetics"
    }
    fn title(&self, _: &Params) -> String {
        "Formation and destruction during a roast: A to B to C".into()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        h.push_str(&field(id, "t0", "Bean temperature at start", num(p, "t0", 100.0)?, "&deg;C"));
        h.push_str(&field(id, "rate", "Heating rate", num(p, "rate", 10.0)?, "&deg;C/min"));
        h.push_str(&field(id, "tend", "Final temperature", num(p, "tend", 225.0)?, "&deg;C"));
        h.push_str(&field(id, "ea1", "E<sub>a</sub>, formation of B", num(p, "ea1", 100.0)?, "kJ/mol"));
        h.push_str(&field(id, "ea2", "E<sub>a</sub>, destruction of B", num(p, "ea2", 140.0)?, "kJ/mol"));
        h.push_str(&field(id, "k1ref", "Formation rate constant at 190 &deg;C", num(p, "k1ref", 0.3)?, "per min"));
        h.push_str(&field(id, "k2ref", "Destruction rate constant at 190 &deg;C", num(p, "k2ref", 0.1)?, "per min"));
        h.push_str(&formula(
            "dA/dt = &minus;k<sub>1</sub>A, dB/dt = k<sub>1</sub>A &minus; k<sub>2</sub>B, with k<sub>i</sub>(T) = k<sub>i,ref</sub> exp[&minus;(E<sub>a,i</sub>/R)(1/T &minus; 1/T<sub>ref</sub>)], T in kelvin, T<sub>ref</sub> = 463.15 K. Temperature rises linearly until the final temperature, then the roast ends.",
        ));
        Ok(h)
    }
}

// ---- data-driven widgets ---------------------------------------------------
//
// Most calculators are a list of inputs, a formula line and a script function.
// They are declared here as data; parameters in the chapter override defaults.

pub enum Field {
    /// name, label, default, unit
    Num(&'static str, &'static str, f64, &'static str),
    /// name, label, default text, rows. Data are typed one record per line.
    Text(&'static str, &'static str, &'static str, usize),
}

pub struct Generic {
    pub kind: &'static str,
    pub title: &'static str,
    pub fields: &'static [Field],
    pub formula: &'static str,
    pub note: &'static str,
}

impl Widget for Generic {
    fn kind(&self) -> &'static str {
        self.kind
    }
    fn title(&self, p: &Params) -> String {
        text(p, "title", self.title).to_string()
    }
    fn body(&self, id: &str, p: &Params) -> Result<String, String> {
        let mut h = String::new();
        for f in self.fields {
            match f {
                Field::Num(name, label, default, unit) => h.push_str(&field(id, name, label, num(p, name, *default)?, unit)),
                Field::Text(name, label, default, rows) => {
                    let v = text(p, name, default);
                    h.push_str(&format!(
                        "<div class=\"f\"><label for=\"{id}-{name}\">{label}</label><br><textarea name=\"{name}\" id=\"{id}-{name}\" rows=\"{rows}\" cols=\"36\">{}</textarea></div>\n",
                        esc(v)
                    ));
                }
            }
        }
        h.push_str(&formula(self.formula));
        if !self.note.is_empty() {
            h.push_str(&format!("<p class=\"small\">{}</p>\n", self.note));
        }
        Ok(h)
    }
}

use Field::{Num, Text};

pub static GENERIC: &[Generic] = &[
    Generic {
        kind: "calibration",
        title: "Calibration line by least squares",
        fields: &[Text("data", "Pairs, one per line: known value, instrument reading", "0.50 0.52\n1.00 1.01\n1.50 1.53\n2.00 2.02\n3.00 3.05", 6)],
        formula: "Reading = a + b &times; known. b = S<sub>xy</sub>/S<sub>xx</sub>, a = mean(y) &minus; b mean(x); residual SD s = &radic;(&Sigma;e<sup>2</sup>/(n &minus; 2)).",
        note: "",
    },
    Generic {
        kind: "ey-uncertainty",
        title: "Uncertainty of an extraction yield",
        fields: &[
            Num("dose", "Dose", 15.0, "g"), Num("ddose", "&plusmn;", 0.1, "g"),
            Num("bev", "Beverage mass", 250.0, "g"), Num("dbev", "&plusmn;", 1.0, "g"),
            Num("tds", "TDS", 1.35, "%"), Num("dtds", "&plusmn;", 0.02, "%"),
        ],
        formula: "(&delta;EY/EY)<sup>2</sup> = (&delta;TDS/TDS)<sup>2</sup> + (&delta;B/B)<sup>2</sup> + (&delta;D/D)<sup>2</sup> for EY = TDS &times; B / D.",
        note: "",
    },
    Generic {
        kind: "triangle-power",
        title: "Triangle test: power and proportion of discriminators",
        fields: &[Num("n", "Trials", 24.0, ""), Num("pd", "True proportion of discriminators", 0.3, ""), Num("alpha", "Significance level", 0.05, "")],
        formula: "Probability correct = 1/3 + (2/3) p<sub>d</sub>. Power = P(X &ge; k<sub>crit</sub>) under that probability, where k<sub>crit</sub> is the smallest k with P(X &ge; k | 1/3) &le; &alpha;. From data, p<sub>d</sub> = (3 p<sub>c</sub> &minus; 1)/2.",
        note: "",
    },
    Generic {
        kind: "oav",
        title: "Odour activity values",
        fields: &[Text("data", "One compound per line: name, concentration, odour threshold (same units)", "2-furfurylthiol 20 0.01\nguaiacol 3000 2.5\n4-vinylguaiacol 40000 20\nmethional 250 0.2\nfuraneol 100000 25", 7)],
        formula: "OAV = concentration / odour threshold. Compounds with OAV &gt; 1 are candidates for contributing to aroma; recombination and omission tests confirm it.",
        note: "Example values are illustrative orders of magnitude for teaching, not measurements of one coffee.",
    },
    Generic {
        kind: "anova",
        title: "One-way ANOVA for panel scores",
        fields: &[Text("data", "One sample per line, scores separated by spaces", "6 7 6 8 7 6\n5 5 6 4 5 6\n7 8 8 7 9 8", 6)],
        formula: "F = [SS<sub>between</sub>/(k &minus; 1)] / [SS<sub>within</sub>/(N &minus; k)]; p from the F distribution with (k &minus; 1, N &minus; k) degrees of freedom.",
        note: "",
    },
    Generic {
        kind: "power",
        title: "Sample size for comparing two means",
        fields: &[Num("d", "Effect size (difference / SD)", 0.8, ""), Num("alpha", "Significance level (two-sided)", 0.05, ""), Num("power", "Desired power", 0.8, "")],
        formula: "n per group &asymp; 2 [(z<sub>1&minus;&alpha;/2</sub> + z<sub>power</sub>) / d]<sup>2</sup> (normal approximation; add one or two for small samples).",
        note: "",
    },
    Generic {
        kind: "equilibrium",
        title: "Immersion at equilibrium (desorption model)",
        fields: &[Num("k", "Equilibrium constant K", 0.717, ""), Num("emax", "Maximum extractable fraction E<sub>max</sub>", 0.30, ""), Num("rlo", "Brew ratio from", 8.0, ": 1"), Num("rhi", "to", 20.0, ": 1")],
        formula: "EY = K E<sub>max</sub> (independent of ratio). TDS = K E<sub>max</sub> / (R + K E<sub>max</sub>), R = water/coffee.",
        note: "Default K is the value Liang, Chan and Ristenpart (2021) fitted for 1 L brews.",
    },
    Generic {
        kind: "diffusion",
        title: "Diffusion time for a spherical particle",
        fields: &[Num("d", "Particle diameter", 600.0, "&micro;m"), Num("diff", "Effective diffusivity D", 2.0e-10, "m<sup>2</sup>/s")],
        formula: "For a sphere of radius a, the slowest-decaying term of the series solution decays with time constant &tau; = a<sup>2</sup>/(&pi;<sup>2</sup>D).",
        note: "Effective diffusivities inside coffee particles are uncertain; vary D to see the scaling with d<sup>2</sup>.",
    },
    Generic {
        kind: "darcy",
        title: "Flow through an espresso puck (Darcy and Kozeny-Carman)",
        fields: &[
            Num("dp", "Effective particle diameter", 7.0, "&micro;m"), Num("eps", "Porosity", 0.20, ""),
            Num("dia", "Basket diameter", 58.0, "mm"), Num("len", "Puck thickness", 12.0, "mm"),
            Num("mu", "Water viscosity at 93 &deg;C", 0.30, "mPa s"), Num("pbar", "Pressure drop", 9.0, "bar"),
        ],
        formula: "k = d<sup>2</sup>&epsilon;<sup>3</sup>/[180(1 &minus; &epsilon;)<sup>2</sup>]; Q = k A &Delta;P / (&mu; L).",
        note: "Permeability is set by the fine particles, so the effective diameter is far below the median grind size; about 7 &micro;m reproduces a typical 2 mL/s shot. Real pucks also compact and channel (Chapter 2B).",
    },
    Generic {
        kind: "speciation",
        title: "Carbonate speciation",
        fields: &[Num("ph", "pH", 7.5, ""), Num("alk", "Alkalinity", 40.0, "mg/L as CaCO<sub>3</sub>"), Num("pka1", "pK<sub>a1</sub>", 6.35, ""), Num("pka2", "pK<sub>a2</sub>", 10.33, "")],
        formula: "&alpha;<sub>0</sub> = [H<sup>+</sup>]<sup>2</sup>/D, &alpha;<sub>1</sub> = [H<sup>+</sup>]K<sub>1</sub>/D, &alpha;<sub>2</sub> = K<sub>1</sub>K<sub>2</sub>/D, D = [H<sup>+</sup>]<sup>2</sup> + [H<sup>+</sup>]K<sub>1</sub> + K<sub>1</sub>K<sub>2</sub>.",
        note: "",
    },
    Generic {
        kind: "blend",
        title: "Blending tap water with reverse-osmosis water",
        fields: &[Num("tapalk", "Tap alkalinity", 120.0, "mg/L CaCO<sub>3</sub>"), Num("taphard", "Tap hardness", 180.0, "mg/L CaCO<sub>3</sub>"), Num("target", "Target alkalinity", 40.0, "mg/L CaCO<sub>3</sub>"), Num("vol", "Volume to make", 10.0, "L")],
        formula: "Fraction of tap water f = target / tap alkalinity (RO water assumed ion-free). Hardness of the blend = f &times; tap hardness.",
        note: "",
    },
    Generic {
        kind: "sieve",
        title: "Sieve analysis to a size distribution",
        fields: &[Text("data", "One sieve fraction per line: lower aperture, upper aperture (&micro;m), mass retained (g)", "0 100 0.9\n100 200 1.1\n200 400 2.6\n400 600 6.8\n600 800 5.4\n800 1200 3.2", 7)],
        formula: "Volume fraction = mass fraction (uniform density). Midpoint d<sub>i</sub> = (lower + upper)/2. Specific surface &asymp; &Sigma; w<sub>i</sub> &times; 6/(&rho; d<sub>i</sub>). Number fraction &prop; w<sub>i</sub>/d<sub>i</sub><sup>3</sup>.",
        note: "Assumes spheres and particle density 1.2 g/cm<sup>3</sup>; the fines bin dominates surface area and number.",
    },
    Generic {
        kind: "weibull-fit",
        title: "Fit a Weibull degassing curve to your jar data",
        fields: &[Text("data", "One reading per line: hours since roast, cumulative mass lost (g)", "6 0.05\n24 0.13\n48 0.20\n96 0.28\n168 0.35\n336 0.43", 7), Num("total", "Estimated total releasable gas", 0.55, "g")],
        formula: "ln[&minus;ln(1 &minus; F)] = &beta; ln t &minus; &beta; ln &tau;, with F = loss/total. Slope gives &beta;; intercept gives &tau;.",
        note: "",
    },
    Generic {
        kind: "arrhenius",
        title: "Arrhenius: activation energy and extrapolation",
        fields: &[Num("k1", "Rate constant at T<sub>1</sub>", 0.437, "per day"), Num("t1", "T<sub>1</sub>", 40.0, "&deg;C"), Num("k2", "Rate constant at T<sub>2</sub>", 2.10, "per day"), Num("t2", "T<sub>2</sub>", 60.0, "&deg;C"), Num("t3", "Extrapolate to T<sub>3</sub>", 25.0, "&deg;C")],
        formula: "E<sub>a</sub> = R ln(k<sub>2</sub>/k<sub>1</sub>) / (1/T<sub>1</sub> &minus; 1/T<sub>2</sub>); k(T<sub>3</sub>) = k<sub>1</sub> exp[&minus;(E<sub>a</sub>/R)(1/T<sub>3</sub> &minus; 1/T<sub>1</sub>)].",
        note: "",
    },
    Generic {
        kind: "moisture",
        title: "Moisture: wet and dry basis",
        fields: &[Num("wb", "Moisture, wet basis", 11.0, "%"), Num("mass", "Green mass", 1000.0, "g")],
        formula: "Dry basis X = w/(1 &minus; w); water mass = w &times; mass; dry matter = (1 &minus; w) &times; mass.",
        note: "",
    },
    Generic {
        kind: "roast-sim",
        title: "Lumped energy balance for a roasting batch",
        fields: &[
            Num("mass", "Green batch mass", 1.0, "kg"), Num("w0", "Initial moisture (wet basis)", 11.0, "%"),
            Num("tair", "Effective air/drum temperature", 250.0, "&deg;C"), Num("ha", "Heat transfer h A per kg of green coffee", 0.006, "kW/(K kg)"),
            Num("cp", "Specific heat of dry bean", 1.4, "kJ/(kg K)"), Num("tend", "Drop temperature", 215.0, "&deg;C"),
            Num("het", "Reaction heat released", 232.0, "kJ per kg dry matter"),
        ],
        formula: "m c<sub>p</sub> dT/dt = hA (T<sub>air</sub> &minus; T) &minus; L<sub>v</sub> dm<sub>w</sub>/dt + Q<sub>rxn</sub>. Drying above 100 &deg;C at a rate proportional to remaining water; reaction heat released by a first-order process above 150 &deg;C.",
        note: "A teaching model with illustrative rate constants. It reproduces the shape of a roast curve (fast rise, drying plateau, exothermic acceleration), not a particular roaster.",
    },
    Generic {
        kind: "ror",
        title: "Rate of rise from a roast log",
        fields: &[Text("data", "One reading per line: seconds, bean probe &deg;C", "0 200\n60 110\n120 118\n180 132\n240 146\n300 158\n360 169\n420 178\n480 186\n540 193\n600 199\n660 204", 8), Num("win", "Window (readings)", 3.0, "")],
        formula: "RoR at each point = slope of a least-squares line through the surrounding window of readings, in &deg;C per minute.",
        note: "Wider windows smooth noise but delay and flatten real changes.",
    },
    Generic {
        kind: "parallel",
        title: "Competing pathways: acids against bitter compounds",
        fields: &[
            Num("t0", "Start", 100.0, "&deg;C"), Num("rate", "Heating rate", 10.0, "&deg;C/min"), Num("tend", "Drop temperature", 215.0, "&deg;C"),
            Num("ea1", "E<sub>a</sub> acid formation", 90.0, "kJ/mol"), Num("ea2", "E<sub>a</sub> acid destruction", 150.0, "kJ/mol"), Num("ea3", "E<sub>a</sub> bitter-compound formation", 130.0, "kJ/mol"),
        ],
        formula: "Precursor S &rarr; acids A (k<sub>1</sub>), A &rarr; loss (k<sub>2</sub>), chlorogenic acid G &rarr; bitter lactones L (k<sub>3</sub>); k<sub>i</sub> referenced to 0.3, 0.1 and 0.05 per min at 190 &deg;C.",
        note: "Compare two heating rates at the same drop temperature and note the acid-to-bitter ratio.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_is_inclusive() {
        let a = axis(1.0, 1.6, 0.05);
        assert_eq!(a.len(), 13);
        assert!((a[12] - 1.6).abs() < 1e-9);
    }

    #[test]
    fn every_widget_renders_with_defaults() {
        for k in kinds() {
            let spec = WidgetSpec { kind: k.to_string(), params: toml::Table::new() };
            let h = render("t", &spec).unwrap();
            assert!(h.contains(&format!("value=\"{k}\"")));
            assert!(h.contains("class=\"formula\""), "{k} lacks a formula");
        }
    }

    #[test]
    fn unknown_kind_is_error() {
        let spec = WidgetSpec { kind: "nope".into(), params: toml::Table::new() };
        assert!(render("t", &spec).is_err());
    }

    #[test]
    fn bad_param_type_is_error() {
        let mut p = toml::Table::new();
        p.insert("dose".into(), toml::Value::String("x".into()));
        let spec = WidgetSpec { kind: "extraction".into(), params: p };
        assert!(render("t", &spec).is_err());
    }
}
