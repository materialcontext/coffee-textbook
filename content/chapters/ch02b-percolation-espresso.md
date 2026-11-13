+++
number = 2
label = "2B"
title = "Extraction II: percolation and espresso"
question = "What happens when water is forced through a bed instead of sitting in it?"
objectives = [
  "Apply Darcy's law and the Kozeny-Carman relation to a coffee bed, and predict how grind, porosity, dose and pressure change the flow.",
  "Explain how pouring mixes a filter bed and why fine particles move during brewing.",
  "Calculate the strength and extraction yield of a batch brew with bypass.",
  "Explain why espresso yield peaks at an intermediate grind, and why flow and beverage mass dominate the outcome.",
  "Describe puck compaction and why flow levels off as pressure rises.",
  "Design an espresso recipe from these principles and test it.",
]
prerequisites = ["Chapter 2A", "Appendix C, Section C.5 (viscosity)"]
+++
An espresso machine pushes water through 18 g of coffee at nine times atmospheric pressure. A pour-over relies on a few centimetres of water standing above the grounds. The driving pressures differ by a factor of about a thousand, yet both brews flow at one or two millilitres per second. The two coffee beds must therefore differ enormously in how strongly they resist flow. Understanding that resistance explains why espresso is so sensitive to grind, why pressure is a weak control, and why some shots extract unevenly.

This chapter applies the extraction ideas of Chapter 2A to {{percolation}}, brewing in which water flows through a bed of grounds. Section 1 builds the physics of flow through a porous bed. Sections 2 and 3 apply it to filter brewing; Sections 4 to 6 apply it to espresso; Section 7 turns it into a method for designing recipes.

## 1. Flow through a porous bed

A coffee bed is a {{porous medium}}: a solid framework of particles with connected spaces between them through which liquid can flow. The {{porosity}} &epsilon; of a bed is the fraction of its volume that is open space. A loose bed of filter grounds has a porosity near 0.4; a tamped espresso puck, near 0.2.

### Darcy's law

In 1856 Henry Darcy measured water flowing through sand filters and found that the flow rate is proportional to the pressure difference across the bed. The result, {{Darcy's law}}, holds for slow flow through any porous medium:

~~~eq darcy
formula = "Q = &kappa; A &Delta;P / (&mu; L)"
where = [["Q", "volumetric flow rate", "m<sup>3</sup>/s"], ["&kappa;", "permeability of the bed", "m<sup>2</sup>"], ["A", "cross-sectional area of the bed", "m<sup>2</sup>"], ["&Delta;P", "pressure difference across the bed", "Pa"], ["&mu;", "dynamic viscosity of the liquid", "Pa s"], ["L", "thickness of the bed", "m"]]
~~~

Darcy's law has the same form as Ohm's law for an electrical resistor. Pressure difference plays the role of voltage, flow rate the role of current, and L/(&kappa;A) the role of resistance. A thicker bed resists more; a wider bed resists less. {{Viscosity}}, the liquid's internal resistance to flowing, appears because a thicker liquid moves more slowly under the same push.

The {{permeability}} &kappa; is a property of the bed alone: how easily it lets fluid through. It has units of area, because it is set by the size of the channels between particles.

### Estimating permeability: Kozeny-Carman

Permeability can be measured, but it is useful to estimate it from the particles. For a bed of spheres of diameter d, the {{Kozeny-Carman relation}} gives

~~~eq kc
formula = "&kappa; = d<sup>2</sup> &epsilon;<sup>3</sup> / [180 (1 &minus; &epsilon;)<sup>2</sup>]"
where = [["d", "particle diameter (for a real grind, an effective diameter)", "m"], ["&epsilon;", "porosity", "fraction"]]
~~~

The numerical factor 180 comes from fitting the relation to experiments on packed beds. Two features matter for coffee. Permeability rises with d<sup>2</sup>, so halving the particle size quarters the flow. Permeability is also sensitive to porosity, through the factor &epsilon;<sup>3</sup>: reducing &epsilon; from 0.20 to 0.17 cuts &kappa; by about 40%.

What is d for a grind that contains fines of a few micrometres and particles near a millimetre? Liquid flows through the gaps between particles, and the narrowest gaps are set by the smallest particles packed among the larger ones. The {{effective particle diameter}} in {eq:kc} is therefore much smaller than the median particle size. Chapter 4 shows that most particles in a typical grind, counted one by one, are fines.

~~~widget darcy
~~~

~~~example What an espresso puck's permeability must be
Problem. A shot runs at 2.2 mL/s through an 18 g puck in a 58 mm basket, 12 mm thick, at 9 bar (9 &times; 10<sup>5</sup> Pa) with water at 93 &deg;C (&mu; = 0.30 mPa s). Find the permeability, and the effective particle size it implies if &epsilon; = 0.20.

Plan. Rearrange {eq:darcy} for &kappa;, then solve {eq:kc} for d.

Solution. The area is A = &pi;(0.029 m)<sup>2</sup> = 2.64 &times; 10<sup>&minus;3</sup> m<sup>2</sup>, and Q = 2.2 &times; 10<sup>&minus;6</sup> m<sup>3</sup>/s. Then &kappa; = Q &mu; L/(A &Delta;P) = (2.2 &times; 10<sup>&minus;6</sup>)(3.0 &times; 10<sup>&minus;4</sup>)(0.012)/[(2.64 &times; 10<sup>&minus;3</sup>)(9 &times; 10<sup>5</sup>)] = 3.3 &times; 10<sup>&minus;15</sup> m<sup>2</sup>. With &epsilon; = 0.20, the factor &epsilon;<sup>3</sup>/[180(1 &minus; &epsilon;)<sup>2</sup>] = 0.008/115.2 = 6.9 &times; 10<sup>&minus;5</sup>, so d<sup>2</sup> = 3.3 &times; 10<sup>&minus;15</sup>/6.9 &times; 10<sup>&minus;5</sup> = 4.8 &times; 10<sup>&minus;11</sup> m<sup>2</sup> and d &asymp; 7 &micro;m.

Check. A typical espresso grind has a median near 300 &micro;m, forty times larger. The flow is controlled by the fines, as the argument above predicted.
~~~

~~~evidence
Mo and colleagues (2023) imaged espresso beds by X-ray micro-tomography (three-dimensional X-ray imaging) and simulated flow through the images [1]. They reported porosity near 0.17 and a permeability near 5.5 &times; 10<sup>&minus;13</sup> m<sup>2</sup> at low pressure, with corrections for inertia at espresso pressures and higher porosity near the outlet after extraction. That permeability is over a hundred times the value the worked example requires. Possible reasons include fines smaller than the scan could resolve, fines migrating to the outlet, compaction of the wet puck under load, and the resistance of the basket screen. The discrepancy is unresolved. Simple models like Kozeny-Carman are reliable for how flow scales and unreliable for absolute numbers.
~~~

### Is the flow slow enough for Darcy's law?

Darcy's law assumes that viscous forces dominate the flow and that inertia is negligible. The {{Reynolds number}} measures the balance between the two. For flow through a bed it is

~~~eq re
formula = "Re = &rho; v d / &mu;"
where = [["&rho;", "density of the liquid", "kg/m<sup>3</sup>"], ["v", "superficial velocity, Q/A", "m/s"]]
~~~

The {{superficial velocity}} is the flow rate divided by the whole cross-section of the bed, as if the particles were not there. Below Re of about 1, viscous forces dominate and the flow is smooth, or {{laminar}}. In the worked example v = Q/A = 0.85 mm/s, so with &rho; = 965 kg/m<sup>3</sup> the Reynolds number is 0.02 for d = 7 &micro;m and 0.8 for d = 300 &micro;m. Darcy's law is a reasonable approximation on average; inertia matters only locally, in wide pores and channels.

~~~quiz
title = "Check your understanding: Section 1"
[[q]]
type = "numeric"
prompt = "By what factor does Kozeny-Carman permeability change if the effective particle diameter halves at constant porosity?"
answer = 0.25
tolerance = 0.001
unit = ""
explain = "&kappa; scales with d<sup>2</sup>: (1/2)<sup>2</sup> = 0.25."

[[q]]
type = "choice"
prompt = "In Darcy's law, doubling the bed thickness at constant pressure will"
options = ["double the flow", "halve the flow", "leave the flow unchanged"]
answer = 1
explain = "Q is inversely proportional to L."
~~~

## 2. Pour-over: gravity, jets and moving fines

With the physics in hand we can answer the opening puzzle. In a pour-over the driving pressure is only the weight of liquid above the filter, the {{hydrostatic pressure}} &Delta;P = &rho;gh, where g = 9.81 m/s<sup>2</sup> and h is the height of liquid. With 8 cm of liquid and bed, &Delta;P = 965 &times; 9.81 &times; 0.08 &asymp; 760 Pa.

~~~example Why filter grind is coarser than espresso grind
Problem. A pour-over runs at about 1.5 mL/s under 0.8 kPa through a bed 30 mm thick. An espresso runs at 2 mL/s under 900 kPa through 12 mm. Taking similar bed areas, how much more permeable must the filter bed be, and what effective particle size does that imply?

Plan. From {eq:darcy}, &kappa; is proportional to QL/&Delta;P at fixed area and viscosity. Then separate the contributions of porosity and particle size using {eq:kc}.

Solution. The permeability ratio is &kappa;<sub>filter</sub>/&kappa;<sub>espresso</sub> = (1.5/2)(900/0.8)(30/12) &asymp; 2,100. Porosity accounts for part of this: &epsilon;<sup>3</sup>/(1 &minus; &epsilon;)<sup>2</sup> is 0.178 for a loose bed at 0.40 and 0.0125 for a puck at 0.20, a ratio of 14. The rest must come from particle size: d<sub>filter</sub>/d<sub>espresso</sub> = &radic;(2,100/14) &asymp; 12. If the espresso's effective size is 7 &micro;m, the filter bed's is about 85 &micro;m.

Check. The argument rests on scaling rather than measurement, but it explains why the controlling size in a filter grind is roughly ten times larger, and why a small change in fines content changes drawdown time markedly.
~~~

Pouring also mixes the bed. Park, Young and Mathijssen (2025) filmed jets of water striking a transparent model of a coffee bed [2]. A smooth, unbroken jet triggered avalanches of particles that stirred grounds and liquid together; a jet that broke into droplets mixed poorly. Pouring from as high as the jet stays unbroken, with a thick enough stream, raised extraction from the same dose. A gooseneck kettle's job is to deliver a controlled, unbroken jet.

Fine particles move during the brew. Liquid flowing through the bed carries the smallest particles downward, and they can collect at the filter, lowering permeability as the brew goes on. Stirring or swirling late in a brew lifts fines, which then settle on top of the bed or at the filter depending on timing. Because the effect on drawdown depends on where they end up, it is best determined by experiment (Lab L2.3).

### Filters

Paper filters hold back fines, insoluble oils and most of the cafestol and kahweol, two oil-soluble compounds that raise blood cholesterol in people who drink large amounts of unfiltered coffee [3]. Metal filters and French presses let more of these through, giving a heavier, cloudier cup. Papers also differ in their own resistance to flow, and a dense paper can be the main resistance in a fast, coarse brew.

## 3. Batch brewers and bypass

Large batch brewers face a scaling problem: a big bed needs either a very coarse grind or a very long contact time to avoid over-strong coffee. Many solve it with {{bypass}}, sending part of the water around the bed directly into the carafe. The bed then runs at higher strength, and the bypass water dilutes it to drinking strength.

~~~example A two-litre batch with 20% bypass
Problem. A brewer uses 2,000 g of water, 110 g of coffee and 20% bypass. The grounds retain 2.0 g of liquid per gram, and the extract leaving the bed reads 1.90% TDS. Find the extraction yield and the final strength.

Plan. Follow the water: through the bed, held in the bed, out of the bed, then diluted. Use the percolation mass balance, Equation 2A.2, for the bed.

Solution. Water through the bed: 0.80 &times; 2,000 = 1,600 g. Held in the grounds: 2.0 &times; 110 = 220 g. Extract leaving the bed: 1,380 g. Solids extracted: 0.0190 &times; 1,380 = 26.2 g, so E = 26.2/110 = 23.8%. Final beverage: 1,380 + 400 = 1,780 g, with S = 26.2/1,780 = 1.47%.

Check. The final strength is lower than the bed's strength by the dilution factor 1,380/1,780 = 0.78, as expected. Bypass lets strength and extraction be set separately.
~~~

## 4. Espresso as pressure-driven percolation

{{Espresso}} pushes water at about 9 bar through a compacted bed of fine grounds called the {{puck}}. A typical recipe takes 18 g of coffee to 36 g of beverage in 25 to 35 s, with strength around 8 to 12%. A 36 g shot at 9.5% TDS from 18 g therefore has an extraction yield of 0.095 &times; 36/18 = 19.0%.

### The grind paradox

Finer grinding shortens diffusion times (Chapter 2A), so a model with uniform flow predicts that yield should keep rising as the grind gets finer. Cameron and colleagues (2020) built such a model and compared it with careful experiments [4]. Measured yield rose with finer grinding to a maximum and then fell. Their explanation was that at fine settings the flow through the puck becomes uneven: water finds faster paths and bypasses regions that remain under-extracted (Section 6). Grinding coarser and reducing the dose, while keeping beverage mass and yield the same, gave more reproducible shots and used less coffee.

### Fines

A 2024 study varied the fraction of fines in espresso beds [5]. Adding fines lengthened shot time substantially by lowering permeability but barely changed extraction yield, and the added fines did not lower sensory scores. Fines act mainly on flow, as the Darcy analysis predicts.

### What controls what reaches the cup

Schmieder and colleagues (2023) built a model espresso system in which they could set flow rate, particle size and temperature independently [6]. They measured trigonelline, caffeine, 5-caffeoylquinic acid and total dissolved solids in successive fractions of the beverage. Flow rate had by far the strongest effect on how much of each reached the cup; grind size and temperature had minor effects by comparison.

~~~misconception
"A good espresso takes 25 to 30 seconds." Shot time is a symptom of flow resistance, not a cause of extraction. What fixes how much is extracted is how much water passed through, which is measured by beverage mass. Weigh the beverage and stop at a target mass; use time only to diagnose grind and puck preparation.
~~~

## 5. Puck compaction and the pressure plateau

If a puck were rigid, {eq:darcy} would make flow proportional to pressure. Measurements show something else: above a few bar, raising the pressure barely raises the flow.

Waszkiewicz and colleagues (2026) measured flow against pressure for many grinds and doses [7]. Flow rose in proportion to pressure below about 5 bar and then levelled off. Their explanation is {{poroelastic compaction}}: the pressure gradient pushes particles together, porosity falls, and permeability falls with &epsilon;<sup>3</sup>. The reduced permeability offsets the increased push. When flow and pressure were scaled by characteristic values for each puck, all the measurements fell on one curve, shown in {fig:poro}.

~~~figure
id = "poro"
file = "ch02b-poroelastic.gif"
alt = "Graph of normalised flow against normalised pressure; a dashed straight line for a rigid puck rises steeply, while the curve for a compacting puck bends over and flattens at the critical pressure."
caption = "Flow against pressure for a rigid bed (Darcy's law, dashed) and for a compacting bed, in units scaled by the critical pressure and flow (after Waszkiewicz et al., 2026, preprint). Notice how far the two curves have separated by the critical pressure."
~~~

~~~evidence
The poroelastic result is a {{preprint}}, a paper posted publicly before peer review, from one apparatus. It agrees with long-standing barista experience that raising pressure above about 6 bar changes flow less than changing the grind does, and with the porosity gradient found by micro-tomography [1]. Treat the plateau as well supported in its general shape and the exact curve as provisional.
~~~

~~~misconception
"Higher pump pressure makes espresso flow faster and extract more." For a compacting puck, pressures above the critical value compress the bed and lower its permeability, so flow barely rises. Compaction is one reason shots at about 6 bar can work as well as shots at 9 bar. Grind, which acts on permeability through d<sup>2</sup>, and dose, which sets the bed thickness, are the strong controls.
~~~

## 6. Channelling

A {{channel}} is a path of locally high permeability through which a disproportionate share of the water flows. Water in a channel has short contact with little coffee and leaves dilute. The coffee next to the channel is over-extracted, and the coffee away from it is under-extracted. The result is a lower yield than uniform flow would give at the same grind, and a cup that can taste bitter and sour at once.

A two-path model estimates the penalty. Suppose a fraction f of the water passes through channels and extracts the coffee around them to a low yield E<sub>c</sub>, while the rest extracts the bulk to E<sub>b</sub>. If each gram of water carries a similar share of solids, the measured yield is approximately the flow-weighted mean:

~~~eq channel
formula = "E &asymp; (1 &minus; f) E<sub>b</sub> + f E<sub>c</sub>"
where = [["f", "fraction of the water flowing through channels", "fraction"], ["E<sub>b</sub>, E<sub>c</sub>", "extraction yields of the bulk and of the channel regions", "fraction"]]
~~~

With f = 0.3, E<sub>c</sub> = 12% and E<sub>b</sub> = 22%, the shot measures about 19%, three points below uniform flow.

Practices that reduce channelling all make the puck uniform before water arrives. Grounds are distributed evenly, for example by stirring them with fine needles, then tamped level with no gap at the basket wall. {{Pre-infusion}} wets the puck at low pressure first, so that it swells and settles before full pressure is applied.

~~~quiz
title = "Check your understanding: Sections 4 to 6"
[[q]]
type = "numeric"
prompt = "A 40 g shot at 9.0% TDS comes from 18 g of coffee. What is the extraction yield in percent?"
answer = 20
tolerance = 0.05
unit = "%"
explain = "E = S m<sub>b</sub>/m<sub>c</sub> = 0.090 &times; 40/18 = 20.0%."

[[q]]
type = "choice"
prompt = "According to Schmieder et al., which variable most directly controls how much of each soluble reaches the espresso cup?"
options = ["Flow rate and beverage mass", "Brew temperature", "Tamping pressure"]
answer = 0
explain = "Flow rate dominated; grind size and temperature had minor effects by comparison."

[[q]]
type = "numeric"
prompt = "In the two-path model, what channel fraction f gives a measured yield of 18% when E<sub>b</sub> = 22% and E<sub>c</sub> = 12%?"
answer = 0.4
tolerance = 0.005
unit = ""
explain = "18 = 22 &minus; 10f, so f = 0.4."
~~~

## 7. Designing a recipe

The models of this chapter suggest an order of adjustment:

1. Fix the ratio by beverage mass (for example 1:2) and fix the dose to suit the basket.
2. Adjust the grind until the cup tastes balanced; measure TDS and calculate the extraction yield.
3. If yield falls as you grind finer, you are past the peak: grind coarser, consider a smaller dose, and improve distribution.
4. Use pressure and temperature only for fine adjustments, because they are weaker controls than grind and beverage mass.

Record dose, beverage mass, time, TDS and a tasting note for every shot. The uncertainty methods of Chapter 0 apply: with typical refractometer and scale precision, two shots within about 0.5 percentage points of extraction yield cannot be told apart.

## Looking ahead

Chapters 2A and 2B treated water as a neutral carrier that dissolves coffee and flows through it. It is not neutral. The minerals dissolved in brew water react with coffee acids before the cup reaches the taster, and they decide whether a machine scales or corrodes. Chapter 3 asks which properties of water matter for the cup, and by what chemistry.

## Key equations

~~~equations
[[eq]]
name = "Darcy's law"
formula = "Q = &kappa; A &Delta;P / (&mu; L)"
note = "Equation 2B.1."
[[eq]]
name = "Kozeny-Carman"
formula = "&kappa; = d<sup>2</sup>&epsilon;<sup>3</sup> / [180(1 &minus; &epsilon;)<sup>2</sup>]"
note = "Equation 2B.2."
[[eq]]
name = "Reynolds number"
formula = "Re = &rho; v d / &mu;"
note = "Equation 2B.3."
[[eq]]
name = "Hydrostatic pressure"
formula = "&Delta;P = &rho; g h"
note = "Pour-over driving pressure."
[[eq]]
name = "Two-path channelling"
formula = "E &asymp; (1 &minus; f) E<sub>b</sub> + f E<sub>c</sub>"
note = "Equation 2B.4."
~~~

~~~keyterms
~~~

## Labs

~~~lab
id = "L2.3"
title = "Pour height and drawdown"
aim = "Test whether the height and smoothness of the pouring jet change strength and drawdown at fixed dose, grind and water."
safety = ["Pouring from height splashes hot water: use a deep server, keep hands clear, and wear an apron."]
materials = ["Cone dripper and filters", "Gooseneck kettle", "Ruler or marked stand", "Scale with timer", "Refractometer"]
method = [
  "Fix the dose (15 g), water (250 g), grind and temperature.",
  "Brew three cups with the spout 3 cm above the bed and three with it at the highest point at which the jet stays unbroken (often 10 to 15 cm), in random order.",
  "Record total brew time, drawdown time (end of pour to end of dripping), beverage mass and TDS.",
]
columns = ["Run", "Height (cm)", "Brew time (s)", "Drawdown (s)", "Beverage (g)", "TDS (%)", "EY (%)"]
rows = 6
analysis = [
  "Compare the mean extraction yield at each height and compute the standard error of each mean.",
  "Is the difference larger than the measurement uncertainty from Chapter 0?",
]
expected = "Higher unbroken pours usually give slightly higher extraction; drawdown may lengthen if fines are carried to the filter."
~~~

~~~lab
id = "L2.4"
title = "Espresso: yield against grind"
aim = "Find the grind setting at which espresso yield peaks, and compare shot-to-shot variation on either side of it."
safety = ["Portafilters and group heads are hot; steam wands can scald."]
materials = ["Espresso machine", "Grinder with repeatable settings", "Scale under the cup", "Refractometer with syringe filters"]
method = [
  "Fix the dose and target beverage mass (for example 18 g to 36 g).",
  "Pull three shots at each of five grind settings from coarse to fine, in random order.",
  "Stop each shot at the target mass; record time and TDS.",
]
columns = ["Setting", "Shot", "Time (s)", "Beverage (g)", "TDS (%)", "EY (%)"]
rows = 15
analysis = [
  "Plot extraction yield against grind setting with error bars. Where is the peak?",
  "Compare the standard deviation of yield at the settings either side of the peak.",
  "Repeat the best coarse setting with a dose 2 g smaller. Does yield hold?",
]
expected = "Yield rises and then falls as the grind gets finer; variation is often larger at fine settings."
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "State Darcy's law and give each quantity with its SI unit."
solution = "Q = &kappa; A &Delta;P/(&mu; L): Q flow rate (m<sup>3</sup>/s), &kappa; permeability (m<sup>2</sup>), A cross-section (m<sup>2</sup>), &Delta;P pressure difference (Pa), &mu; dynamic viscosity (Pa s), L bed thickness (m)."

[[p]]
tier = "A"
section = "4"
prompt = "Why should espresso be dosed by beverage mass rather than by time?"
solution = "Beverage mass fixes how much water passed through and therefore, for a given strength, how much was extracted. Time depends on flow resistance, which varies with grind, fines and puck preparation; it diagnoses the flow but does not measure extraction."

[[p]]
tier = "A"
section = "2"
prompt = "What do paper filters remove that metal filters let through, and why might it matter to a heavy coffee drinker?"
solution = "Fines, insoluble oils and most of the cafestol and kahweol. These compounds raise blood cholesterol at high intakes of unfiltered coffee."

[[p]]
tier = "B"
section = "1"
prompt = "Calculate the Kozeny-Carman permeability for d = 7 &micro;m at porosities of 0.20 and 0.17. By what factor does it fall?"
solution = "&epsilon; = 0.20: &kappa; = (7 &times; 10<sup>&minus;6</sup>)<sup>2</sup> &times; 0.008/(180 &times; 0.64) = 3.40 &times; 10<sup>&minus;15</sup> m<sup>2</sup>. &epsilon; = 0.17: &epsilon;<sup>3</sup> = 0.004913 and (1 &minus; &epsilon;)<sup>2</sup> = 0.6889, so &kappa; = 49 &times; 10<sup>&minus;12</sup> &times; 0.004913/(180 &times; 0.6889) = 1.94 &times; 10<sup>&minus;15</sup> m<sup>2</sup>. It falls to 0.57 of its value."

[[p]]
tier = "B"
section = "5"
prompt = "Using the Darcy calculator's default values, predict the flow at 6 bar for a rigid puck. Would a real, compacting puck run faster or slower than this prediction, relative to its flow at 9 bar?"
solution = "For a rigid puck Q scales with &Delta;P: 2.25 &times; 6/9 = 1.50 mL/s. A compacting puck is less compressed at 6 bar, so its permeability is higher than at 9 bar; its real flow at 6 bar will be closer to its 9 bar flow than the rigid prediction suggests."

[[p]]
tier = "B"
section = "1"
prompt = "Increasing the dose from 18 to 20 g raises the puck thickness from 12.0 to 13.3 mm. At fixed grind and pressure, what does Darcy's law predict for the flow if it was 2.25 mL/s?"
solution = "Q scales as 1/L: 2.25 &times; 12.0/13.3 = 2.03 mL/s. Real pucks also compact differently, so this is a first estimate."

[[p]]
tier = "B"
section = "1"
prompt = "Compute the Reynolds number for v = 0.85 mm/s, d = 300 &micro;m, &rho; = 965 kg/m<sup>3</sup> and &mu; = 0.30 mPa s. Is Darcy's law plausible?"
solution = "Re = 965 &times; 8.5 &times; 10<sup>&minus;4</sup> &times; 3 &times; 10<sup>&minus;4</sup>/(3 &times; 10<sup>&minus;4</sup>) = 0.82. Below about 1 viscous forces dominate, so Darcy's law is a reasonable approximation on average."

[[p]]
tier = "B"
section = "3"
prompt = "A batch brewer uses 1,500 g of water, 85 g of coffee and 15% bypass. The grounds retain 2.0 g/g and the bed extract reads 1.80%. Find the extraction yield and final strength."
solution = "Through the bed: 1,275 g; retained: 170 g; extract: 1,105 g. Solids: 0.0180 &times; 1,105 = 19.9 g, so E = 19.9/85 = 23.4%. Final mass: 1,105 + 225 = 1,330 g, so S = 19.9/1,330 = 1.50%."

[[p]]
tier = "B"
section = "2"
prompt = "What is the hydrostatic pressure at the bottom of a 5 cm liquid column above a 3 cm wet bed, taking &rho; = 970 kg/m<sup>3</sup>? Express it as a fraction of 9 bar."
solution = "&Delta;P = 970 &times; 9.81 &times; 0.08 = 761 Pa. 9 bar = 9 &times; 10<sup>5</sup> Pa, so the ratio is 8.5 &times; 10<sup>&minus;4</sup>."

[[p]]
tier = "B"
section = "1"
prompt = "In the espresso example the permeability was 3.3 &times; 10<sup>&minus;15</sup> m<sup>2</sup>. What flow would Darcy's law predict at 9 bar with the micro-tomography permeability of 5.5 &times; 10<sup>&minus;13</sup> m<sup>2</sup>, other values unchanged?"
solution = "Flow scales with &kappa;: 2.2 &times; (5.5 &times; 10<sup>&minus;13</sup>/3.3 &times; 10<sup>&minus;15</sup>) &asymp; 370 mL/s, impossible for a real shot. The low-pressure imaging value cannot describe the loaded, fines-clogged puck."

[[p]]
tier = "B"
section = "1"
prompt = "Water's viscosity is about 0.89 mPa s at 25 &deg;C and 0.30 mPa s at 93 &deg;C. At fixed permeability, how much faster does the hot water flow?"
solution = "Q scales as 1/&mu;: 0.89/0.30 = 3.0 times faster."

[[p]]
tier = "C"
section = "4"
prompt = "Explain the espresso grind paradox using both the diffusion time scale of Chapter 2A and the permeability scaling of this chapter."
solution = "Finer grinding shortens diffusion times (as d<sup>2</sup>), which should raise yield. It also lowers permeability (as d<sup>2</sup>), so small non-uniformities in a low-permeability bed grow into channels. Beyond some fineness, the yield lost to channelled flow outweighs the yield gained from faster diffusion, which produces a peak."

[[p]]
tier = "C"
section = "4"
prompt = "Suggest why fines added to an espresso bed lengthen shot time but barely change yield."
solution = "Fines dominate the effective diameter in Kozeny-Carman, so adding them lowers permeability and slows the flow. They hold a small share of the dose and empty almost instantly, adding little yield; the larger particles gain only slightly from the longer contact at fixed beverage mass."

[[p]]
tier = "C"
section = "6"
prompt = "Design an experiment to test whether pre-infusion reduces channelling. State what you would measure and how many shots you would need."
solution = "Alternate shots with and without pre-infusion at fixed dose, grind and beverage mass, in random order. Use the standard deviation of extraction yield as the main outcome (less channelling should give higher, less variable yields), with mean yield as a secondary outcome. Detecting a 0.5 point change in mean yield with a standard deviation of 0.6 at 80% power needs about 23 shots per condition (Appendix E.6); comparing variances needs more."

[[p]]
tier = "C"
section = "5"
prompt = "A colleague proposes raising pump pressure to 12 bar to speed up a slow shot. Using the poroelastic picture, argue for a better adjustment."
solution = "Above the critical pressure flow levels off, so 12 bar will barely increase it and may compact the puck further. A coarser grind raises permeability with the square of the effective size, and a smaller dose reduces the bed thickness; both act directly on flow. Better distribution can also remove local resistances."
~~~

## References

1. Mo C. et al. (2023), Exploring the link between coffee matrix microstructure and flow properties using combined X-ray microtomography and smoothed particle hydrodynamics simulations, *Scientific Reports* 13:16374.
2. Park E., Young M., Mathijssen A.J.T.M. (2025), Pour-over coffee: mixing by a water jet impinging on a granular bed with avalanche dynamics, *Physics of Fluids* 37(4).
3. Urgert R., Katan M.B. (1997), The cholesterol-raising factor from coffee beans, *Annual Review of Nutrition* 17:305.
4. Cameron M.I. et al. (2020), Systematically improving espresso: insights from mathematical modeling and experiment, *Matter* 2:631.
5. The role of fines in espresso extraction dynamics (2024), *Scientific Reports*.
6. Schmieder B.K.L., Pannusch V.B., Vannieuwenhuyse L., Briesen H., Minceva M. (2023), Influence of flow rate, particle size, and temperature on espresso extraction kinetics, *Foods* 12:2871.
7. Waszkiewicz R. et al. (2026), Under pressure: poroelastic regulation of flow in espresso brewing, arXiv:2512.21528 (preprint).
