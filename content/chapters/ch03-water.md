+++
number = 3
title = "Water"
question = "Which properties of brew water change the cup, and by what chemistry?"
objectives = [
  "Read a water report, convert its ions to common units and check its charge balance.",
  "Calculate the proportions of carbon dioxide, bicarbonate and carbonate in water at a given pH.",
  "Predict how much coffee acid a given alkalinity neutralises.",
  "Explain why current evidence ranks alkalinity above hardness for flavour, and how that evidence revised an earlier hypothesis.",
  "Design brew water from salts or by blending, and assess the risk of scale and corrosion.",
]
prerequisites = ["Chapter 0, Section 6 (titratable acidity)", "Appendix A, Sections A.2 to A.7 (units, equilibrium, acids and bases, solubility)"]
+++
A roastery changes the water in its cupping room. The old water was distilled water with a little magnesium salt added; the new water follows the Specialty Coffee Association standard. Within a week the cuppers complain that every coffee has lost its brightness. The coffees have not changed. What in the water could flatten their acidity, and why would one water do it and not the other?

A cup of filter coffee is more than 98% water, and that water arrives carrying dissolved minerals. This chapter identifies which of them matter, explains the chemistry by which they act, and shows how to design water for brewing and for equipment. Along the way it follows a scientific argument from a plausible hypothesis, through popular practice, to the experiments that tested it.

## 1. What is dissolved in water

Natural water dissolves minerals from the rocks it passes through. The minerals separate into {{ion|ions}}, atoms or groups of atoms that carry an electric charge. Positive ions are {{cation|cations}}; negative ions are {{anion|anions}}. A water report lists their concentrations in mg/L. The ions that matter for coffee are:

| Cations | Anions | Other |
|---|---|---|
| calcium, Ca<sup>2+</sup> | bicarbonate, HCO<sub>3</sub><sup>&minus;</sup> | pH |
| magnesium, Mg<sup>2+</sup> | chloride, Cl<sup>&minus;</sup> | chlorine or chloramine (disinfectant) |
| sodium, Na<sup>+</sup> | sulfate, SO<sub>4</sub><sup>2&minus;</sup> | |
| potassium, K<sup>+</sup> | nitrate, NO<sub>3</sub><sup>&minus;</sup> | |

Two summary quantities describe most of what matters for coffee. Both are reported in a unit that seems strange at first, mg/L "as CaCO<sub>3</sub>", explained in Appendix A.2. The unit converts ions with different masses and charges onto one scale by asking how much calcium carbonate would contain the same amount of charge.

{{Hardness}} is the concentration of the doubly charged cations, calcium and magnesium:

~~~eq hard
formula = "H = 100.09 &times; ([Ca<sup>2+</sup>] + [Mg<sup>2+</sup>])"
where = [["H", "total hardness", "mg/L as CaCO<sub>3</sub>"], ["[Ca<sup>2+</sup>], [Mg<sup>2+</sup>]", "concentrations of calcium and magnesium", "mmol/L"]]
~~~

{{Alkalinity}} is the water's capacity to neutralise acid. In drinking water it comes almost entirely from bicarbonate:

~~~eq alk
formula = "Alk = 50.04 &times; [HCO<sub>3</sub><sup>&minus;</sup>]"
where = [["Alk", "total alkalinity", "mg/L as CaCO<sub>3</sub>"], ["[HCO<sub>3</sub><sup>&minus;</sup>]", "concentration of bicarbonate", "mmol/L"]]
~~~

The factor is half that in {eq:hard} because one bicarbonate neutralises one hydrogen ion, while one carbonate ion (in CaCO<sub>3</sub>) neutralises two.

Hardness and alkalinity are often confused because the most common source of both is dissolved limestone, which supplies calcium and bicarbonate together. They are independent quantities. Magnesium sulfate adds hardness and no alkalinity; sodium bicarbonate adds alkalinity and no hardness.

### Checking a report: the charge balance

Water is electrically neutral, so the total positive charge of its cations must equal the total negative charge of its anions. Charge is counted in {{milliequivalent|milliequivalents}} (meq): one millimole of an ion with charge z carries z milliequivalents of charge. A report whose cations and anions do not balance is either wrong or incomplete.

~~~eq balance
formula = "e = (&Sigma; cations &minus; &Sigma; anions) / (&Sigma; cations + &Sigma; anions)"
where = [["e", "charge balance error", "fraction; within &plusmn;5% is acceptable"], ["&Sigma;", "sum of the ions' charges", "meq/L"]]
~~~

~~~example Checking a water report
Problem. A report lists Ca<sup>2+</sup> 32, Mg<sup>2+</sup> 8, Na<sup>+</sup> 12, K<sup>+</sup> 2, HCO<sub>3</sub><sup>&minus;</sup> 110, Cl<sup>&minus;</sup> 18, SO<sub>4</sub><sup>2&minus;</sup> 24 and NO<sub>3</sub><sup>&minus;</sup> 4 mg/L. Check the charge balance, and find the hardness and alkalinity.

Plan. Convert each ion to meq/L (mg/L &divide; molar mass &times; charge), sum, and apply {eq:balance}; then use {eq:hard} and {eq:alk}.

Solution. Cations: Ca 32/40.08 &times; 2 = 1.597, Mg 8/24.31 &times; 2 = 0.658, Na 12/22.99 = 0.522, K 2/39.10 = 0.051; total 2.828 meq/L. Anions: HCO<sub>3</sub> 110/61.02 = 1.803, Cl 18/35.45 = 0.508, SO<sub>4</sub> 24/96.06 &times; 2 = 0.500, NO<sub>3</sub> 4/62.00 = 0.065; total 2.875 meq/L. The error is (2.828 &minus; 2.875)/(2.828 + 2.875) = &minus;0.8%. Hardness = (0.798 + 0.329) &times; 100.09 = 113 mg/L as CaCO<sub>3</sub>. Alkalinity = 1.803 &times; 50.04 = 90 mg/L as CaCO<sub>3</sub>.

Check. The balance error is well within &plusmn;5%, so the report is internally consistent. The water is moderately hard and fairly alkaline.
~~~

~~~widget water-ions
~~~

## 2. The carbonate system

Alkalinity depends on bicarbonate, and bicarbonate belongs to a family of three related forms of dissolved carbon. Carbon dioxide dissolves in water and reacts with it as a weak acid in two steps (Appendix A.5):

CO<sub>2</sub>(aq) + H<sub>2</sub>O &harr; H<sup>+</sup> + HCO<sub>3</sub><sup>&minus;</sup>, with pK<sub>a1</sub> &asymp; 6.35;

HCO<sub>3</sub><sup>&minus;</sup> &harr; H<sup>+</sup> + CO<sub>3</sub><sup>2&minus;</sup>, with pK<sub>a2</sub> &asymp; 10.33 (both at 25 &deg;C).

The fraction of dissolved carbon in each form depends only on pH. Calculating these fractions is called {{speciation}}. Writing K<sub>1</sub> = 10<sup>&minus;6.35</sup> and K<sub>2</sub> = 10<sup>&minus;10.33</sup> for the two acid constants, the fraction present as bicarbonate is

~~~eq alpha
formula = "&alpha;<sub>1</sub> = [H<sup>+</sup>]K<sub>1</sub> / ([H<sup>+</sup>]<sup>2</sup> + [H<sup>+</sup>]K<sub>1</sub> + K<sub>1</sub>K<sub>2</sub>)"
where = [["&alpha;<sub>1</sub>", "fraction of dissolved inorganic carbon present as bicarbonate", "fraction"], ["[H<sup>+</sup>]", "hydrogen ion concentration, 10<sup>&minus;pH</sup>", "mol/L"]]
~~~

The fractions as CO<sub>2</sub> and as carbonate have the same denominator, with numerators [H<sup>+</sup>]<sup>2</sup> and K<sub>1</sub>K<sub>2</sub> respectively. {fig:speciation} plots all three.

~~~figure
id = "speciation"
file = "ch03-speciation.gif"
alt = "Speciation diagram of dissolved carbon dioxide, bicarbonate and carbonate fractions against pH from 3 to 14, with bands marking typical brew water pH and coffee pH."
caption = "Carbonate speciation at 25 &deg;C. Find the two shaded bands: in brew water (pH 6.5 to 7.5) dissolved carbon is mostly bicarbonate; at coffee pH near 5 it is almost all CO<sub>2</sub>."
~~~

~~~example Speciation of a brew water
Problem. A water at pH 7.5 has 40 mg/L alkalinity as CaCO<sub>3</sub>. How much of its dissolved carbon is bicarbonate, CO<sub>2</sub> and carbonate?

Plan. Compute the three fractions at pH 7.5, then use the alkalinity to find the total dissolved carbon.

Solution. At pH 7.5 the fractions are 0.066 (CO<sub>2</sub>), 0.933 (bicarbonate) and 0.0014 (carbonate). Alkalinity is 40/50.04 = 0.799 meq/L. Each bicarbonate contributes one equivalent and each carbonate two, so the total dissolved carbon is 0.799/(0.933 + 2 &times; 0.0014) = 0.855 mmol/L. That is 48.6 mg/L of bicarbonate, 2.5 mg/L of dissolved CO<sub>2</sub> and only 0.07 mg/L of carbonate.

Check. The three fractions sum to 1.000, and bicarbonate dominates as the figure predicts.
~~~

~~~widget speciation
~~~

The figure also shows what happens when brew water meets coffee. At the pH of a brew, near 5, almost all dissolved carbon is CO<sub>2</sub>. The bicarbonate in the water has taken up hydrogen ions from the coffee's acids, and the CO<sub>2</sub> formed escapes from the hot liquid.

## 3. Alkalinity neutralises coffee acids

That last observation is the key to the chapter. Each bicarbonate ion removes one acidic hydrogen from a coffee acid, written HA:

~~~eq neut
formula = "HA + HCO<sub>3</sub><sup>&minus;</sup> &rarr; A<sup>&minus;</sup> + CO<sub>2</sub> + H<sub>2</sub>O"
where = [["HA", "a coffee acid with its acidic hydrogen", ""], ["A<sup>&minus;</sup>", "the acid's conjugate base, which does not taste sour", ""]]
~~~

~~~example How much acid does the water remove?
Problem. A 300 mL cup is brewed with water of 40 mg/L alkalinity. The finished brew has a titratable acidity of 1.6 mmol/L (Chapter 0). How much acid did the water neutralise, compared with what remains?

Plan. Convert alkalinity to mmol/L of bicarbonate with {eq:alk}; one bicarbonate neutralises one acidic hydrogen ({eq:neut}).

Solution. Bicarbonate: 40/50.04 = 0.80 mmol/L, so 0.80 &times; 0.300 = 0.24 mmol is neutralised in the cup. The titratable acid remaining is 1.6 &times; 0.300 = 0.48 mmol. The water has removed a third of the acid that would otherwise have been there.

Check. At 120 mg/L alkalinity the water would neutralise 0.72 mmol, more than the cup retains. Alkalinity acts directly on titratable acidity, the quantity that best predicts sourness (Chapter 1).
~~~

Neutralisation also raises the brew's pH slightly, and by turning acids into their conjugate bases it changes the balance between the undissociated and dissociated forms, which may affect the character of the sourness as well as its intensity.

~~~widget water-recipe
epsom = 0.12
cacl = 0.0
soda = 0.068
~~~

~~~quiz
title = "Check your understanding: Sections 1 to 3"
[[q]]
type = "numeric"
prompt = "A water contains 20 mg/L Ca<sup>2+</sup> and no magnesium. What is its hardness in mg/L as CaCO<sub>3</sub>?"
answer = 49.9
tolerance = 0.2
unit = "mg/L"
explain = "20/40.08 = 0.499 mmol/L; &times; 100.09 = 49.9."

[[q]]
type = "numeric"
prompt = "How many millimoles of acidic hydrogen does 250 mL of water with 80 mg/L alkalinity neutralise?"
answer = 0.40
tolerance = 0.01
unit = "mmol"
explain = "80/50.04 = 1.60 mmol/L; &times; 0.250 L = 0.40 mmol."
~~~

## 4. Cations: a hypothesis from computation

Alkalinity acts through simple acid-base chemistry. A more elaborate idea about the cations dominated coffee practice for a decade, and its history shows how claims should be tested.

In 2014, Hendon and the Colonna-Dashwoods estimated how strongly sodium, magnesium and calcium ions bind to representative coffee compounds [1]. They used {{density functional theory}}, a method that computes how strongly molecules attract one another from the physics of their electrons. Binding was stronger for ions with higher charge and smaller size, so magnesium bound most strongly. They proposed that cations in brew water bind coffee compounds and help pull them out of the grounds, so that magnesium-rich water would extract more. They also pointed out that the conductivity meters often used to measure water do not show which ions are present. Magnesium-heavy water recipes became popular as a result.

The proposal rested on calculated binding strengths, not on brewing or tasting. It was a hypothesis, and it made predictions that experiments could test.

## 5. Experimental tests

Bratthäll, Figueira and Nording (2024) added magnesium chloride or calcium chloride either to the water before brewing or to the finished coffee, and measured citric, malic, lactic and quinic acids [2]. At drinking-water concentrations, the acid content changed only slightly; at ten times those concentrations changes became noticeable. In most cases, salt added before brewing and salt added afterwards gave similar results. If cations were pulling acids out of the grounds, salt present during brewing should have mattered more than salt added after extraction was over. The authors concluded that acid extraction proceeds largely independently of water composition, while interactions in the finished brew may still affect flavour.

Rune and colleagues (2026) brewed one Colombian coffee in ultrapure water with added calcium, magnesium and bicarbonate. Concentrations ranged from the low end of recommended values to extremes, with bicarbonate up to 300 mg/L. The cups were evaluated by trained panels using screening tests and descriptive analysis [3]. Hardness alone produced no marked change in flavour, even with calcium above typical drinking-water levels. Bicarbonate had a clear effect across its whole range: more alkalinity reduced acidity, fruitiness and complexity and increased roasted and bitter notes. Small interactions between calcium and bicarbonate were explained by calcium carbonate precipitating and lowering the effective alkalinity.

~~~misconception
"Magnesium in brew water extracts more flavour." The idea came from calculated binding energies, not from brewing. Experiments found that salts added before or after brewing changed acid content similarly, and that hardness alone had no marked effect on flavour, while bicarbonate had a large one. On current evidence, the water property that changes the cup most is alkalinity, through acid neutralisation.
~~~

~~~evidence
Both experimental studies are recent and each used a narrow range of coffees; the sensory study used one. The result concerns acids and overall flavour, and does not exclude effects of cations on other compounds or in espresso. But chemistry and sensory work point the same way, and the direction agrees with the stoichiometry of Section 3.
~~~

We can now answer the opening question. The old cupping water had no alkalinity; the SCA standard water has about 40 mg/L. The new water neutralises part of every coffee's acid before it is tasted.

## 6. Standards and recipes

The SCA water standard of 2009 gives these targets [4]:

| Property | Target | Acceptable range |
|---|---|---|
| Total dissolved solids | 150 mg/L | 75 to 250 |
| Calcium hardness | 68 mg/L as CaCO<sub>3</sub> | 17 to 85 |
| Total alkalinity | about 40 mg/L as CaCO<sub>3</sub> | near 40 |
| pH | 7.0 | 6.5 to 7.5 |
| Sodium | about 10 mg/L | |
| Chlorine | 0 | |

Published recipes differ mainly in alkalinity and in whether the hardness comes with sulfate or chloride. Given the evidence of Section 5, alkalinity is the first variable to test. Some commercial mineral products contain no alkalinity at all; they produce sharp, acidic cups and leave equipment without protection against corrosion (Section 8).

### Making water from salts

Distilled or reverse-osmosis water plus weighed salts gives complete control. Because the masses needed per litre are tenths of a gram, it is easier to make a {{concentrate}}: a known mass of one salt dissolved in a known mass of distilled water, dosed by weight into each batch. Keep calcium and bicarbonate in separate concentrates, because together they can precipitate calcium carbonate.

~~~example A two-concentrate recipe
Problem. Design concentrates that give water with 68 mg/L hardness from calcium and 40 mg/L alkalinity, using calcium chloride dihydrate (CaCl<sub>2</sub>&middot;2H<sub>2</sub>O, 147.01 g/mol) and sodium bicarbonate (84.01 g/mol).

Plan. Convert each target to mmol/L, then to mg/L of salt, then choose a concentrate strength and dose.

Solution. Calcium: 68/100.09 = 0.679 mmol/L, so 0.679 &times; 147.01 = 99.9 mg/L of the dihydrate. A concentrate of 10.0 g in 1,000 g of distilled water, dosed at 10.0 g per litre, delivers it. Alkalinity: 40/50.04 = 0.799 mmol/L of bicarbonate, so 0.799 &times; 84.01 = 67.1 mg/L of sodium bicarbonate, which also adds 18.4 mg/L of sodium. A concentrate of 6.71 g per 1,000 g, dosed at 10.0 g per litre, delivers it.

Check. Use the molar mass of the hydrated salt, including its water; using anhydrous CaCl<sub>2</sub> (110.98 g/mol) would supply a third too much calcium.
~~~

### Blending

If tap water is too alkaline, blending it with reverse-osmosis water lowers alkalinity and hardness together. Treating reverse-osmosis water as free of ions, the fraction of tap water needed is

~~~eq blend
formula = "f<sub>tap</sub> = Alk<sub>target</sub> / Alk<sub>tap</sub>"
where = [["f<sub>tap</sub>", "fraction of tap water in the blend", "fraction"]]
~~~

~~~widget blend
~~~

With tap water at 120 mg/L alkalinity and 180 mg/L hardness, a 40 mg/L target needs one part tap water to two parts reverse-osmosis water, and the blend has 60 mg/L hardness.

## 7. Treatment

Each treatment method removes something different.

- {{Activated carbon}} adsorbs chlorine and many organic off-flavours. Chloramine, a longer-lasting disinfectant, needs longer contact time or a catalytic carbon.
- {{Ion exchange}} resins swap ions in the water for ions held on the resin. A sodium-form softener replaces calcium and magnesium with sodium, lowering hardness and leaving alkalinity unchanged. It prevents scale without improving flavour.
- A hydrogen-form (weak-acid) resin replaces cations with hydrogen ions, which react with bicarbonate to form CO<sub>2</sub>. It removes alkalinity and is widely used in cafés for that reason.
- {{Reverse osmosis}} forces water through a membrane that holds back most ions. The product is then remineralised or blended [5].

~~~misconception
"Softened water is better coffee water." A sodium-exchange softener removes hardness but leaves alkalinity unchanged, so the water still neutralises coffee acid as before. It protects equipment from scale; it does not change the property that most affects flavour.
~~~

## 8. Scale and corrosion

Water that is good for flavour must also be safe for equipment. Calcium carbonate deposits as {{scale}} when the product of calcium and carbonate concentrations exceeds calcite's {{solubility product}}, K<sub>sp</sub> (Appendix A.7). Heating makes scaling more likely for two reasons: calcite becomes less soluble as temperature rises, and heat drives CO<sub>2</sub> out of the water, which shifts bicarbonate toward carbonate. Boilers therefore scale even when tap water does not.

The {{Langelier saturation index}} summarises the tendency. It compares the actual pH with pH<sub>s</sub>, the pH at which the water would be exactly saturated with calcite:

~~~eq lsi
formula = "LSI = pH &minus; pH<sub>s</sub>"
where = [["LSI", "Langelier saturation index; positive values predict scale, strongly negative values predict corrosion", ""], ["pH<sub>s</sub>", "saturation pH, from Equation 3.8", ""]]
~~~

A widely used empirical formula gives pH<sub>s</sub> from four measurements:

~~~eq phs
formula = "pH<sub>s</sub> = (9.3 + A + B) &minus; (C + D)"
where = [["A", "(log<sub>10</sub> TDS &minus; 1)/10, with TDS in mg/L", ""], ["B", "&minus;13.12 log<sub>10</sub>(T + 273) + 34.55, with T in &deg;C", ""], ["C", "log<sub>10</sub>(calcium hardness) &minus; 0.4, in mg/L as CaCO<sub>3</sub>", ""], ["D", "log<sub>10</sub>(alkalinity), in mg/L as CaCO<sub>3</sub>", ""]]
~~~

The terms follow from the chemistry. More calcium (C) or more alkalinity (D) lowers pH<sub>s</sub>, making saturation easier to reach. Higher temperature lowers B, which also lowers pH<sub>s</sub>, reflecting calcite's falling solubility. More total dissolved solids (A) raises pH<sub>s</sub> slightly, because other ions reduce the effective concentrations of calcium and carbonate.

~~~example Scale at boiler temperature
Problem. Compare SCA target water (TDS 150, calcium hardness 68, alkalinity 40, pH 7.0) with hard tap water (TDS 300, calcium hardness 150, alkalinity 120, pH 7.8), at 25 &deg;C and at 95 &deg;C.

Plan. Compute A, B, C and D with {eq:phs}, then the LSI with {eq:lsi}.

Solution. SCA water: pH<sub>s</sub> = 8.47 at 25 &deg;C and 7.27 at 95 &deg;C, so LSI = &minus;1.47 and &minus;0.27; undersaturated at both temperatures. Hard tap water: LSI = +0.12 at 25 &deg;C, near saturation, and +1.32 at 95 &deg;C, strongly scaling.

Check. The calculation holds pH fixed. In a real boiler, pH rises as CO<sub>2</sub> escapes, which makes the hard-water case worse.
~~~

Very soft water with almost no alkalinity cannot scale, but it is aggressive to copper and brass. Espresso machine water is therefore a compromise [6]: low enough alkalinity for good flavour, enough hardness and alkalinity to avoid corrosion, and not so much that the boiler scales.

## Looking ahead

Water and coffee meet at the surface of the ground particles, and the last three chapters have assumed those particles. Chapter 4 asks what a grinder actually produces, how to describe a grind with numbers, and which features of the grind control flow and extraction.

## Key equations

~~~equations
[[eq]]
name = "Hardness"
formula = "H = 100.09 &times; ([Ca<sup>2+</sup>] + [Mg<sup>2+</sup>])"
note = "Equation 3.1. mmol/L in, mg/L as CaCO<sub>3</sub> out."
[[eq]]
name = "Alkalinity"
formula = "Alk = 50.04 &times; [HCO<sub>3</sub><sup>&minus;</sup>]"
note = "Equation 3.2."
[[eq]]
name = "Bicarbonate fraction"
formula = "&alpha;<sub>1</sub> = [H<sup>+</sup>]K<sub>1</sub> / ([H<sup>+</sup>]<sup>2</sup> + [H<sup>+</sup>]K<sub>1</sub> + K<sub>1</sub>K<sub>2</sub>)"
note = "Equation 3.4."
[[eq]]
name = "Blending"
formula = "f<sub>tap</sub> = Alk<sub>target</sub> / Alk<sub>tap</sub>"
note = "Equation 3.6."
[[eq]]
name = "Langelier index"
formula = "LSI = pH &minus; pH<sub>s</sub>"
note = "Equations 3.7 and 3.8."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "choice"
prompt = "In the 2026 sensory study, which water variable had a clear effect on coffee flavour across its whole range?"
options = ["Calcium hardness", "Magnesium hardness", "Bicarbonate (alkalinity)"]
answer = 2
explain = "Hardness alone produced no marked change; bicarbonate reduced acidity and fruitiness and increased roasted and bitter notes."

[[q]]
type = "choice"
prompt = "A sodium-exchange water softener"
options = ["removes alkalinity", "removes hardness but leaves alkalinity unchanged", "removes both"]
answer = 1
explain = "It swaps Ca<sup>2+</sup> and Mg<sup>2+</sup> for Na<sup>+</sup>; bicarbonate is untouched."

[[q]]
type = "choice"
prompt = "Why does a boiler scale when the same water does not scale in the pipes?"
options = ["Heating lowers calcite solubility and drives off CO<sub>2</sub>, shifting bicarbonate to carbonate", "Heating adds calcium", "Boilers are made of reactive metals"]
answer = 0
explain = "Both effects raise the ion product relative to K<sub>sp</sub>."
~~~

## Labs

~~~lab
id = "L3.1"
title = "Alkalinity against hardness"
aim = "Separate the effects of hardness and alkalinity on the cup, following the design of Rune et al. at small scale."
safety = ["Label all concentrates and do not store them in drink containers."]
materials = ["Distilled or reverse-osmosis water", "Concentrates of Epsom salt, calcium chloride dihydrate and sodium bicarbonate", "pH meter and NaOH for titration", "Refractometer"]
method = [
  "Prepare five waters: (a) no hardness, no alkalinity; (b) 70 mg/L hardness from magnesium, no alkalinity; (c) 70 mg/L hardness from calcium, no alkalinity; (d) 70 hardness, 40 alkalinity; (e) 70 hardness, 120 alkalinity.",
  "Brew one coffee in each water at matched extraction yield (adjust the grind so that yields agree within &plusmn;0.5 points).",
  "Run duo-trio tests of (a) against (b), (b) against (c), and (d) against (e).",
  "Rate sourness, bitterness and fruitiness of all five blind; titrate 50 mL of each to pH 8.2.",
]
columns = ["Water", "Hardness", "Alkalinity", "EY (%)", "pH", "TA (mmol/L)", "Sour", "Bitter", "Fruity"]
rows = 5
analysis = [
  "Which duo-trio comparisons were significant?",
  "Plot titratable acidity and sourness against alkalinity, and against hardness at zero alkalinity.",
]
expected = "Small or no differences among (a), (b) and (c); clear differences between (d) and (e), with lower titratable acidity and sourness at higher alkalinity."
~~~

~~~lab
id = "L3.2"
title = "Alkalinity titration of tap water"
aim = "Measure your tap water's alkalinity by titration to pH 4.5 and compare it with the utility's report."
materials = ["pH meter", "0.0200 mol/L hydrochloric acid", "Burette or graduated syringe", "100 mL sample"]
method = [
  "Titrate 100.0 mL of tap water to pH 4.5 while stirring; record the volume of acid.",
  "Repeat twice.",
]
columns = ["Run", "Acid volume (mL)", "Alkalinity (mg/L as CaCO<sub>3</sub>)"]
rows = 3
analysis = [
  "Alkalinity = c<sub>acid</sub> &times; V<sub>acid</sub> &times; 50,040 / V<sub>sample</sub>, with c in mol/L and both volumes in the same unit.",
  "Compare with the utility report and with test-strip readings if you have them.",
]
~~~

~~~lab
id = "L3.3"
title = "Scale on boiling"
aim = "Observe calcium carbonate precipitating on heating and relate it to the Langelier index."
safety = ["Boiling water and hot glass."]
materials = ["Two clear heat-resistant beakers", "Hard tap water (or distilled water with calcium chloride and bicarbonate concentrates)", "Distilled water", "Hot plate"]
method = [
  "Boil 500 mL of each water down to 250 mL.",
  "Cool, then compare turbidity and deposits on the glass. Titrate the alkalinity of the cooled hard water.",
]
columns = ["Water", "Alkalinity before", "Alkalinity after", "Observation"]
rows = 2
analysis = ["Compute the LSI at 25 and 95 &deg;C for each water. Does the observation match?", "Explain why the alkalinity of the boiled hard water fell."]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "A water report gives Mg<sup>2+</sup> 12 mg/L. What hardness does it contribute, as CaCO<sub>3</sub>?"
solution = "12/24.31 = 0.494 mmol/L; &times; 100.09 = 49.4 mg/L."

[[p]]
tier = "A"
section = "1"
prompt = "Name a salt that adds hardness without alkalinity, and one that adds alkalinity without hardness."
solution = "Magnesium sulfate or calcium chloride adds hardness only; sodium or potassium bicarbonate adds alkalinity only."

[[p]]
tier = "A"
section = "2"
prompt = "Why is dissolved carbon in coffee at pH 5 almost entirely CO<sub>2</sub>?"
solution = "pH 5 is 1.35 units below pK<sub>a1</sub> (6.35), so [HCO<sub>3</sub><sup>&minus;</sup>]/[CO<sub>2</sub>] = 10<sup>&minus;1.35</sup> = 0.045; about 96% is CO<sub>2</sub>."

[[p]]
tier = "B"
section = "1"
prompt = "Check the charge balance of a water with Ca 40, Mg 10 and Na 20 mg/L, and HCO<sub>3</sub><sup>&minus;</sup> 150, Cl<sup>&minus;</sup> 30 and SO<sub>4</sub><sup>2&minus;</sup> 20 mg/L."
solution = "Cations: 1.996 + 0.823 + 0.870 = 3.689 meq/L. Anions: 2.458 + 0.846 + 0.416 = 3.721 meq/L. Error = &minus;0.43%: acceptable."

[[p]]
tier = "B"
section = "2"
prompt = "At pH 8.3, what fraction of dissolved carbon is bicarbonate, and what fraction is carbonate?"
solution = "&alpha;<sub>1</sub> = 0.980 and &alpha;<sub>2</sub> = 0.009; the remaining 0.011 is CO<sub>2</sub>. pH 8.3 is close to where bicarbonate peaks."

[[p]]
tier = "B"
section = "6"
prompt = "Tap water has 150 mg/L alkalinity and 220 mg/L hardness. What blend with reverse-osmosis water gives 50 mg/L alkalinity, and what is the blend's hardness?"
solution = "f = 50/150 = 1/3 tap water and 2/3 reverse-osmosis water. Hardness = 220/3 = 73 mg/L as CaCO<sub>3</sub>."

[[p]]
tier = "B"
section = "6"
prompt = "How many milligrams of potassium bicarbonate (100.12 g/mol) per litre give 30 mg/L alkalinity?"
solution = "30/50.04 = 0.600 mmol/L; &times; 100.12 = 60.0 mg/L."

[[p]]
tier = "B"
section = "1"
prompt = "A titration of 100.0 mL of tap water uses 3.60 mL of 0.0200 mol/L HCl to reach pH 4.5. Find the alkalinity."
solution = "Moles of H<sup>+</sup> = 3.60 &times; 10<sup>&minus;3</sup> &times; 0.0200 = 7.20 &times; 10<sup>&minus;5</sup> mol in 0.100 L, or 0.720 meq/L; &times; 50.04 = 36.0 mg/L as CaCO<sub>3</sub>."

[[p]]
tier = "B"
section = "3"
prompt = "A 12 L batch brew uses water at 100 mg/L alkalinity. How many millimoles of acid does the water neutralise, and how does that compare with a titratable acidity of 1.6 mmol/L in the finished brew?"
solution = "100/50.04 = 2.00 mmol/L &times; 12 L = 24.0 mmol. Per litre the water neutralises 2.0 mmol, more than the 1.6 mmol that remains: over half of the original titratable acid has been removed."

[[p]]
tier = "B"
section = "8"
prompt = "Compute the Langelier index at 95 &deg;C for water with TDS 200, calcium hardness 100, alkalinity 80 and pH 7.5."
solution = "A = (2.301 &minus; 1)/10 = 0.130. B = &minus;13.12 log<sub>10</sub>(368) + 34.55 = 0.887. C = 2.000 &minus; 0.4 = 1.600. D = 1.903. pH<sub>s</sub> = 9.3 + 0.130 + 0.887 &minus; 1.600 &minus; 1.903 = 6.81. LSI = 7.5 &minus; 6.81 = +0.69, so the water will scale."

[[p]]
tier = "C"
section = "5"
prompt = "Explain why salts added before or after brewing giving similar acid results argues against the cation-binding extraction hypothesis."
solution = "If cations increased extraction by binding acids in the grounds, salts present during brewing would raise acid content compared with salts added after extraction was over. Similar results in both cases mean the salts did not change what was extracted; any effect is an interaction in the finished brew."

[[p]]
tier = "C"
section = "5"
prompt = "Explain why calcium and bicarbonate together can have a smaller effect on flavour than bicarbonate alone, as Rune et al. observed."
solution = "With enough calcium, heating and loss of CO<sub>2</sub> can push the ion product above K<sub>sp</sub>, so CaCO<sub>3</sub> precipitates and removes carbonate, and therefore alkalinity, from solution. Less alkalinity reaches the coffee, so less acid is neutralised."

[[p]]
tier = "C"
section = "8"
prompt = "Design café water for an espresso machine with a copper boiler and a house espresso meant to keep its acidity. State targets and justify them with the Langelier index and the flavour evidence."
solution = "Aim for low alkalinity (about 30 to 40 mg/L) to keep acidity; enough calcium hardness (about 50 to 70 mg/L) to keep the water from being corrosive; and an LSI near zero or slightly negative at boiler temperature, checked with Equation 3.8 at about 120 &deg;C. Avoid zero alkalinity, which corrodes and gives sharp cups, and high alkalinity, which flattens acidity and scales."
~~~

## References

1. Hendon C.H., Colonna-Dashwood L., Colonna-Dashwood M. (2014), The role of dissolved cations in coffee extraction, *Journal of Agricultural and Food Chemistry* 62:4947.
2. Bratthäll T., Figueira J., Nording M.L. (2024), Influence of divalent cations on the extraction of organic acids in coffee determined by GC-MS and NMR, *Heliyon* 10:e26625.
3. Rune C.J.B., Steen I., Kjærgaard E.R., Maroun Z., Bredie W.L.P., Kjaergaard H.G. (2026), Impact of water composition on coffee sensory properties: a comprehensive analysis, *Journal of Food Science* 91(9):e71434.
4. SCAA Standards Committee, Water for Brewing Specialty Coffee (revised 2009).
5. Wellinger M., Smrke S., Yeretzian C. (2018), *The SCA Water Quality Handbook*.
6. Navarini L., Rivetti D. (2010), Water quality for espresso coffee, *Food Chemistry* 122:424.
