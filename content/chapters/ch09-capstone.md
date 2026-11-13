+++
number = 9
title = "Capstone: experimental design and one variable followed to the cup"
question = "Can I change one thing in the roaster and show, with numbers and a blind panel, what it did to the cup?"
objectives = [
  "Turn a coffee question into a testable hypothesis with a stated effect size.",
  "Design a randomised, replicated and blocked experiment, including a factorial design that separates linked variables.",
  "Choose sample sizes by power analysis for chemical and sensory outcomes.",
  "Analyse results with t-tests, ANOVA and regression, and report effect sizes with confidence intervals.",
  "Combine sensory and chemical evidence in a mechanistic explanation.",
  "Write a report in the form of a short research paper.",
]
prerequisites = ["All previous chapters", "Appendix E (statistics)"]
+++
A roaster believes that a faster roast keeps more acidity. Three batches later, she has a notebook of temperatures and tasting notes that seem to agree. A colleague asks how she knows the difference was not the day, the batch of green coffee, or what she expected to taste. She cannot say.

Every chapter of this book has separated claims that are well supported from claims that rest on one study and practices that have never been tested. This final chapter gives you the tools to move a claim from the second or third group into the first. Sections 1 to 5 set out how to ask a testable question, design an experiment, size it, run the tasting and analyse the result. Section 6 works through three complete examples. Section 7 specifies the capstone project.

## 1. Questions worth asking

A good capstone question has three properties: it sits among the claims this book marks as uncertain or untested, it can be answered with equipment you have, and its answer would change what someone does. Examples:

- Does development time change sourness at fixed ground colour (Chapter 7)?
- Does a change in hardness alone make a detectable difference for your coffee (Chapter 3)?
- Does adding water before grinding reduce the variability of espresso extraction on your grinder (Chapter 4)?
- After how many days of rest does your espresso stop changing detectably (Chapter 5)?

Turn the question into a {{hypothesis}}, a precise prediction that data could contradict. Include the direction of the effect and the smallest {{effect size}} you would care about. For example: "At matched ground colour, increasing development time from 75 to 150 s lowers titratable acidity by at least 0.1 mmol/L, and at least 30% of tasters can detect the change in a triangle test."

Write the hypothesis down, with the date, before collecting data. This {{pre-registration}}, even if only in a notebook, protects you from quietly adjusting the question to fit whatever the data show.

## 2. Design principles

Five principles turn a comparison into an experiment.

{{Control}}. Hold every variable fixed except the one under study, and record what you cannot fix. Section 7 lists what to hold.

{{Replication}}. Repeat the treatment on independent {{experimental unit|experimental units}}, the things to which the treatment is applied. If the variable is a roast setting, the unit is a roast.

~~~misconception
"Three cups from one roast are three replicates." The roast setting was applied once, so three cups from one roast are three measurements of a single replicate. Treating them as independent replicates, called {{pseudo-replication}}, makes the standard error far too small and the result look far more certain than it is.
~~~

{{Randomisation}}. Decide the order of roasts, brews and servings by chance, so that slow drifts (a machine warming up, tasters tiring, room temperature changing) are not lined up with the treatment.

{{Blocking}}. When a nuisance factor cannot be held constant, arrange for each block of the experiment to contain every treatment. Roast one batch of each level on each day; serve every sample in each tasting session. The analysis then removes the variation between blocks.

Matching end points. Many roasting comparisons are {{confounding|confounded}}: two variables change together, so their effects cannot be separated. A slower roast to the same end temperature is also darker. Schenker avoided this by matching high- and low-temperature roasts on colour and mass loss before comparing them [1]. Matching on the outcome that would otherwise confound the comparison is the general method.

{{Factorial design}}. When two variables are naturally linked, cross them deliberately: test every combination of their levels. A 2 &times; 2 design of colour and development time (Chapter 7) estimates each effect at fixed levels of the other. It also estimates their {{interaction}}, which tells you whether the effect of one variable depends on the level of the other.

## 3. Power and sample size

Before running anything, estimate how many replicates you need. Statistical power, the probability that an experiment detects an effect of a stated size, was introduced in Chapter 1; Appendix E.6 gives the formulas. For comparing two means, the effect size is expressed as d, the difference divided by the standard deviation, and the number of replicates per group needed for 80% power at the 0.05 level is about 15.7/d<sup>2</sup>.

~~~widget power
~~~

~~~widget triangle-power
~~~

Take the standard deviation for a chemical outcome from your own repeat measurements (Chapter 0). For triangle tests, decide the proportion of discriminators worth detecting: 40 trials give 80% power for p<sub>d</sub> = 0.30, while 12 trials detect only large differences (Chapter 1). If the required size is out of reach, change the question (a bigger contrast, a paired design, a more sensitive outcome) rather than run an experiment that cannot answer it.

~~~misconception
"The test was not significant, so there is no effect." A non-significant result from a small experiment is weak evidence either way. Report the estimated effect with its confidence interval, and state the smallest effect the experiment could reliably have detected.
~~~

## 4. Running the tasting

Use discrimination tests to establish whether samples differ, then descriptive tests to say how, and keep preference separate from description, as the CVA does (Chapter 1).

Serve blind under random three-digit codes, in balanced orders (every sample in every position equally often), at a fixed temperature, with water and rest between samples. Use the same tasters for every treatment, so that each taster acts as a block. Anchor descriptive scales with reference samples, and include a repeated sample to check each taster's consistency.

Two published designs are good models. The UC Davis brewing studies crossed nine combinations of strength and extraction with roast level or temperature, 744 brews in total, assessed by a trained panel and a large consumer panel, with titratable acidity measured on every brew [2]. Aroma research supplies a model for showing cause: omission tests remove one compound at a time from a recombined mixture and ask whether tasters notice (Chapter 1).

## 5. Analysis

Report effect sizes with {{confidence interval|confidence intervals}} alongside p-values. A 95% confidence interval is a range constructed so that, over many repetitions of the experiment, 95% of such ranges would contain the true value. "Development time lowered titratable acidity by 0.21 mmol/L (95% confidence interval 0.08 to 0.33)" says what happened and how precisely it is known; "p = 0.01" alone says neither.

Choose the analysis from the design:

| Design | Analysis (Appendix E) |
|---|---|
| Two levels, independent replicates | two-sample t-test; difference with confidence interval |
| Two levels, the same tasters on both | paired t-test |
| Several levels | one-way ANOVA with a post-hoc comparison; with tasters as blocks, two-way ANOVA |
| Two crossed factors | two-way ANOVA with interaction |
| A continuous variable | regression, with a plot of the residuals |
| Many attributes | principal component analysis to describe structure, then tests on the attributes named in the hypothesis |

~~~widget anova
~~~

## 6. Three worked capstones

The data in these examples are illustrative, constructed to resemble the published designs they follow at classroom scale. They show the reasoning and the arithmetic; real data will be noisier.

~~~example Capstone 1: fast against slow at matched colour
Problem. At matched ground colour and mass loss, does a fast roast keep more titratable acidity than a slow one? (Design after Schenker [1].)

Plan. One green lot; three fast and three slow roasts in alternating order over two days; colour checked within &plusmn;1 unit; each roast brewed at an extraction yield of 20.0 &plusmn; 0.4%. Compare titratable acidity with a two-sample t-test, and run a triangle test between the two profiles.

Solution. Titratable acidity (mmol/L): fast 1.62, 1.70, 1.58 (mean 1.633, standard deviation 0.061); slow 1.41, 1.48, 1.39 (mean 1.427, standard deviation 0.047). The difference is 0.207 mmol/L with a standard error of 0.045, giving t = 4.63 with 4 degrees of freedom, p = 0.010, and a 95% confidence interval of 0.08 to 0.33 mmol/L. In a 24-trial triangle test between the profiles, 14 tasters were correct: p = 0.010, and p<sub>d</sub> &asymp; 0.38.

Check. The fast roast kept more acid, as the competing-pathway model of Chapter 8 predicts, and tasters could tell the difference. With only three roasts per group the interval is wide: the direction is clear, the size is not.
~~~

~~~example Capstone 2: titratable acidity across the brewing chart
Problem. In your own brewing, does titratable acidity follow strength rather than extraction yield? (Design after the UC Davis grid [2].)

Plan. One roast; a 3 &times; 3 grid of three ratios and three grind settings, each brewed twice in random order; strength, extraction yield and titratable acidity measured for each brew. Regress titratable acidity on strength and on extraction yield separately, then on both.

Solution. With data like those in Appendix E.5, the regression of titratable acidity on strength gives a slope of about 1.19 mmol/L per percent TDS, with R<sup>2</sup> = 0.99. Adding extraction yield to the model explains almost nothing more.

Check. Then regress the panel's sourness scores on titratable acidity and on extraction yield, to test whether extraction changes sourness beyond acid content, as the published data suggest.
~~~

~~~example Capstone 3: separating development time from colour
Problem. Does development time change sourness at fixed colour? (Design after Alstrup et al. [3].)

Plan. A 2 &times; 2 factorial design: colour (light, dark) crossed with development time (short, long). Three roasts per cell, one per cell on each of three days. A panel of eight tasters scores sourness from 0 to 15; each value below is the panel mean for one roast. Analyse with a two-way ANOVA.

Solution.

| Cell | Roast 1 | Roast 2 | Roast 3 | Mean |
|---|---|---|---|---|
| Light, short | 6.8 | 7.2 | 6.9 | 6.97 |
| Light, long | 6.1 | 5.8 | 6.3 | 6.07 |
| Dark, short | 4.9 | 5.3 | 5.0 | 5.07 |
| Dark, long | 4.6 | 4.4 | 4.9 | 4.63 |

The two-way ANOVA (error mean square 0.053 with 8 degrees of freedom) gives: colour, F = 156, p &lt; 0.001; development time, F = 25.0, p = 0.001; interaction, F = 3.06, p = 0.12.

Check. Both colour and development time lower sourness, and development time has an effect at fixed colour, which answers the question. The development-time effect is 0.90 points in light roasts and 0.43 in dark roasts, but the interaction is not significant with three roasts per cell, so the data cannot say whether the effect really shrinks in darker roasts. That is the next experiment, and it needs more replicates.
~~~

## 7. The project

Choose one variable. Test at least two levels (three if you want to see how the response changes with dose), with at least three independent replicates per level.

| Roast variable | Held constant | What it tests |
|---|---|---|
| Charge temperature | End colour; total time as far as possible | Early heating rate and the acid peak (Chapter 8) |
| End colour | Profile shape up to first crack | Roast degree: lactone breakdown, trigonelline loss |
| Development time | Ground colour (adjust earlier heat) | Development time beyond colour (Chapter 7) |
| Airflow | Burner setting; end colour | Convective against conductive heating |

Projects on water, grinding, resting or brewing follow the same structure.

### Protocol

1. Green coffee: one lot, mixed well, moisture measured (Chapter 6).
2. Roasting: log bean temperature every 1 to 10 seconds. Record charge mass, roasted mass, first crack, drop and ground colour. Randomise or block the roast order.
3. Resting: one rest time, supported by your Chapter 5 data, with all batches stored identically.
4. Water: one prepared water, specified by its ion composition (Chapter 3).
5. Grinding: one grinder, adjusted only enough to hold extraction yield within &plusmn;0.5 points of a target; record the setting.
6. Brewing: one method, ratio and temperature; strength and extraction yield for every brew, with uncertainties (Chapter 0).
7. Chemistry: titratable acidity to pH 8.2 for every brew; other measurements if available (colour, chlorogenic acids by UV-visible spectroscopy, degassing).
8. Sensory: triangle tests between levels, sized by power analysis; then descriptive scoring with anchored scales, blind and balanced; preference recorded separately.

### Report

Structure the report as a short research paper with these parts:

1. An abstract.
2. The question and hypothesis, as pre-registered.
3. Methods in enough detail for someone else to repeat them.
4. Results with effect sizes, confidence intervals and the tests used.
5. A discussion linking the results to mechanisms from the relevant chapters.
6. Limitations.
7. A closing paragraph naming the published result your experiment tests or extends, and how your conclusion could be wrong.

### Assessment

| Component | Weight |
|---|---|
| Design: hypothesis stated in advance; appropriate replication, randomisation and blocking; power considered | 25% |
| Control: confounding variables held fixed or measured | 20% |
| Measurement and analysis: uncertainties, correct tests, effect sizes with intervals | 20% |
| Sensory: blind, balanced, adequately sized and analysed | 15% |
| Explanation: specific chemistry and physics linked to your own data | 20% |

A null result from a well-controlled experiment with adequate power, honestly reported, earns full marks. A striking result from an uncontrolled experiment does not.

## Where to go next

This book has taken coffee from the cup back to the green bean. The appendices remain as reference for the chemistry, physics, analytical methods and statistics behind every chapter. The research cited in each chapter is the place to go deeper, and the evidence boxes mark where that research is thinnest and a well-designed student project could add something new.

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "choice"
prompt = "Three cups brewed from one roast are"
options = ["three replicates of the roast treatment", "three measurements of one replicate", "a block"]
answer = 1
explain = "The treatment was applied once, to one roast; the cups measure that single replicate."

[[q]]
type = "choice"
prompt = "Roasting one batch of each level on each of three days, and analysing day as a factor, is an example of"
options = ["blocking", "pseudo-replication", "pre-registration"]
answer = 0
explain = "Each day is a block containing every treatment; variation between days is removed in the analysis."

[[q]]
type = "choice"
prompt = "In a 2 &times; 2 design of colour and development time, a significant interaction would mean"
options = ["both factors have effects", "the effect of development time depends on colour", "the experiment was confounded"]
answer = 1
explain = "An interaction means that one factor's effect changes with the level of the other."

[[q]]
type = "numeric"
prompt = "About how many triangle trials are needed for 80% power to detect p<sub>d</sub> = 0.30 at the 0.05 level?"
answer = 40
tolerance = 2
unit = ""
explain = "From the binomial calculation in Chapter 1: 40 trials, with 19 correct needed."
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "Rewrite 'Does roast speed affect flavour?' as a testable hypothesis with a direction and a smallest effect of interest."
solution = "For example: 'At matched ground colour (&plusmn;1 unit) and mass loss (&plusmn;0.5 points), a roast reaching first crack in 7 minutes has titratable acidity at least 0.1 mmol/L higher than one reaching first crack in 10 minutes, and the difference is detectable by at least 30% of tasters in a triangle test.'"

[[p]]
tier = "A"
section = "4"
prompt = "Explain why randomising the serving order matters in a sensory test."
solution = "Tasters adapt, tire and compare each sample with the one before, and positions carry systematic biases. Randomised, balanced orders prevent these effects from lining up with a treatment."

[[p]]
tier = "B"
section = "2"
prompt = "A student roasts one batch at each of three charge temperatures and brews each batch five times. They report n = 15 and a significant ANOVA. Critique the design and propose a better one."
solution = "The experimental unit is the roast, so there is one replicate per level; the five brews are pseudo-replicates, and the ANOVA tests brew-to-brew noise rather than charge temperature. Better: at least three roasts per level, randomised or blocked by day, with brews averaged within each roast and the roast used as the unit of analysis."

[[p]]
tier = "B"
section = "6"
prompt = "In Capstone 1, compute a 95% confidence interval for the fast-minus-slow difference in titratable acidity if each group had six roasts with the same means and standard deviations."
solution = "Standard error = &radic;(0.061<sup>2</sup>/6 + 0.047<sup>2</sup>/6) = 0.0315. With about 10 degrees of freedom, t<sub>0.975</sub> = 2.23, so the interval is 0.207 &plusmn; 0.070, from 0.137 to 0.277 mmol/L."

[[p]]
tier = "B"
section = "6"
prompt = "In Capstone 3, compute the main effect of development time and the interaction contrast from the cell means."
solution = "Development-time main effect = mean(short) &minus; mean(long) = 6.02 &minus; 5.35 = 0.67. Interaction contrast = (6.97 &minus; 6.07) &minus; (5.07 &minus; 4.63) = 0.90 &minus; 0.43 = 0.47: the development-time effect is 0.47 points larger in light roasts."

[[p]]
tier = "B"
section = "3"
prompt = "A water experiment needs to detect a sourness difference of 0.6 points. The standard deviation between tasters is 1.5, and the standard deviation of within-taster differences is 0.8. How many tasters are needed for a paired design, and how many per group for an independent design?"
solution = "Paired: d = 0.6/0.8 = 0.75, so n &asymp; (2.80/0.75)<sup>2</sup> = 14 tasters. Independent: d = 0.6/1.5 = 0.4, so n &asymp; 2(2.80/0.4)<sup>2</sup> = 98 per group. The paired design is far more efficient."

[[p]]
tier = "B"
section = "2"
prompt = "Why is matching on ground colour necessary when comparing roast speeds, and what does it not control?"
solution = "Without matching, a slower roast to the same temperature is also darker, so the effects of speed and roast degree are confounded. Matching colour does not control the colour difference between surface and centre, mass loss, porosity, or the different chemical paths taken; those are what the comparison is then about."

[[p]]
tier = "C"
section = "7"
prompt = "Design a capstone to test whether a change in hardness alone is detectable in your café's filter coffee. Include the sample size, sensory method and analysis."
solution = "Prepare two waters with equal alkalinity (for example 40 mg/L) and hardness of 0 or 80 mg/L. Brew each at matched extraction yield in randomised, balanced sessions. Main outcome: a triangle test sized for p<sub>d</sub> = 0.3 (about 40 trials, pooled across sessions). Secondary outcomes: titratable acidity, and descriptive sourness and body with tasters as blocks (paired analysis). Report p<sub>d</sub> with a confidence interval; if the result is not significant, state the p<sub>d</sub> the test could rule out."

[[p]]
tier = "C"
section = "3"
prompt = "A capstone finds no significant difference between two development times (triangle test, 9 correct of 18, p = 0.11). Write the conclusion a careful scientist would write."
solution = "The triangle test did not detect a difference (9 of 18 correct, p = 0.11). The estimated proportion of discriminators was 0.25, with a confidence interval wide enough to include both zero and values above 0.5. The experiment had little power for differences below about p<sub>d</sub> = 0.5, so it neither demonstrates nor excludes a practically important effect. A test of about 40 trials, or a paired descriptive design, would be needed."

[[p]]
tier = "C"
section = "6"
prompt = "Explain how you would combine titratable acidity, colour, mass loss and sensory scores into a mechanistic account of a fast-against-slow roast result."
solution = "First confirm that the matched variables (colour, mass loss) really matched. Show that titratable acidity differs in the direction predicted by the formation-and-destruction kinetics of acids and the competing-pathway model (Chapter 8). Show that sensory sourness moves with titratable acidity, as Chapter 1 predicts. Discuss alternative explanations, such as porosity changing extraction (Chapter 7) or water alkalinity (Chapter 3), and how the data rule them out. State what remains unexplained."
~~~

## References

1. Schenker S. (2000), Investigations on the hot air roasting of coffee beans, Dissertation ETH Zurich No. 13620.
2. Batali M.E. et al. (2023), A new Coffee Brewing Control Chart relating sensory properties and consumer liking to brew strength, extraction yield, and brew ratio, *Journal of Food Science* 88:2168; titratable acidity and sourness study summarised in SCA *25*, issue 15.
3. Alstrup J., Petersen M.A., Larsen F.H., Münchow M. (2020), *Beverages* 6:70.
