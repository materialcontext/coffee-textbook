+++
number = 1
title = "The cup: perception and composition"
question = "What is in the cup, how do we perceive it, and how reliably can we measure perception?"
objectives = [
  "Describe the taste, smell and touch pathways that combine into flavour.",
  "Measure a detection threshold and name the main biases in sensory judgement.",
  "Choose between triangle, duo-trio, two-alternative and tetrad tests, analyse the result, and plan the number of trials.",
  "Describe the SCA Coffee Value Assessment and explain why it separates description from preference.",
  "Relate sourness to titratable acidity and acid identity, and bitterness to specific compounds and receptors.",
  "State the current evidence on coffee sweetness and body.",
  "Interpret aroma research that uses dilution analysis, odour activity values and omission tests.",
]
prerequisites = ["Chapter 0", "Appendix A, Section A.5 (acids and pH)", "Appendix E, Sections E.2 to E.4 (binomial distribution, comparing means)"]
+++
A roaster gives you two cups of the same coffee, brewed with two different waters, and asks whether they taste different. You taste them and think one is brighter. When you taste again, you are less sure. Is there a difference? If there is, what is it, and what substance in the cup is responsible?

Answering requires three kinds of knowledge, and this chapter supplies each in turn. The first is how human perception works, since the taster is the instrument. The second is how to design a tasting so that its result means something, which is a problem in statistics. The third is what the cup contains, so that a perceived difference can be traced to chemistry. The later chapters of the book follow those substances back through brewing and roasting.

## 1. Three senses build flavour

What we call the flavour of coffee is assembled in the brain from three separate senses. Knowing which sense carries which part of the experience tells us where to look for its cause.

Taste. On the tongue and palate, {{taste receptor cell|taste receptor cells}} carry protein {{receptor|receptors}}, molecules that bind particular substances and send a signal when they do. Five taste qualities are well established:

| Quality | Receptor | Example in coffee |
|---|---|---|
| Sweet | a pair of proteins, T1R2 and T1R3 | sugars (present only in traces) |
| Umami | T1R1 with T1R3 | glutamate (minor) |
| Bitter | a family of about 25 proteins called TAS2Rs | caffeine, chlorogenic acid breakdown products |
| Sour | a channel, OTOP1, that lets hydrogen ions into the cell | organic acids |
| Salty | sodium channels and other routes | minerals (minor) |

Smell. Several hundred kinds of {{olfactory receptor|olfactory receptors}} in the upper nasal cavity respond to {{volatile|volatile compounds}}, substances that evaporate readily enough to travel through air. Volatiles reach the nose by two routes. Sniffing the cup carries them in through the nostrils ({{orthonasal}} smell). Drinking carries them from the back of the mouth up into the nasal cavity as you swallow and breathe out ({{retronasal}} smell). Almost all the descriptors that distinguish one coffee from another, from blueberry to toasted bread, are retronasal aroma. Pinch your nose while sipping and those notes disappear, while sourness and bitterness remain.

Touch. Nerves in the mouth detect temperature, thickness, particles and the drying, puckering sensation called {{astringency}}. These sensations make up {{mouthfeel}}, which coffee tasters call body.

The brain combines the three inputs and refers them all to the mouth. That is why retronasal aroma is usually reported as "taste", and why the senses interact: an aroma learned as sweet can raise rated sweetness, and bitterness can suppress perceived sourness.

## 2. The taster as an instrument

Chapter 0 described any instrument by its detection limit, resolution, drift and bias. A taster has all four, and each can be measured.

### Thresholds

The {{detection threshold}} is the lowest concentration at which a substance can be told apart from a blank. The {{recognition threshold}}, usually higher, is the concentration at which its quality can be named. A standard way to measure the detection threshold is the ascending forced-choice method (ASTM E679). At each concentration the taster receives three cups, one spiked and two blank, and must pick the spiked one. Concentrations rise in steps, usually doubling. The {{best-estimate threshold}} is the geometric mean of the last concentration missed and the first concentration from which every answer is correct:

~~~eq bet
formula = "BET = &radic;(c<sub>miss</sub> c<sub>hit</sub>)"
where = [["BET", "best-estimate threshold", "same unit as c"], ["c<sub>miss</sub>", "highest concentration answered wrongly", "e.g. g/L"], ["c<sub>hit</sub>", "lowest concentration answered correctly from then on", "e.g. g/L"]]
~~~

The geometric mean is used because concentrations rise by constant factors, so the midpoint on a logarithmic scale is the natural estimate.

~~~example A threshold estimate
Problem. A taster misses citric acid at 0.10 g/L and identifies it correctly at 0.20, 0.40 and 0.80 g/L. Estimate the detection threshold.

Plan. Apply {eq:bet} to the last miss and the first sustained hit.

Solution. BET = &radic;(0.10 &times; 0.20) = 0.14 g/L.

Check. The estimate lies between the two concentrations, closer to the lower in absolute terms but exactly halfway on a logarithmic scale.
~~~

Thresholds differ between people by factors of four or more and are spread roughly evenly on a logarithmic scale. A group threshold is therefore the geometric mean of individual thresholds.

### Scales and calibration

To describe how strong an attribute is, tasters rate {{intensity}} on a line or a scale numbered from 0 to 15. Untrained people use scales differently: one taster's 5 is another's 9. Trained panels therefore anchor the scale with {{reference standard|reference standards}}, physical samples assigned fixed values, for example a citric acid solution of stated strength as sourness 5. Even after training, tasters keep some personal offset, which is why experiments in which every taster scores every sample are more powerful (Appendix E.3).

### Bias

Five biases distort judgements. Expectation: knowing which cup is which changes the score. The halo effect: one strong attribute raises the scores of others. Contrast: a cup tasted after a very sour cup seems less sour. Position: tasters favour the first or middle sample. Adaptation: sensitivity falls with repeated exposure. The standard controls are to code cups with random three-digit numbers, to randomise and balance the serving order, to rinse and rest between samples, and to serve every sample at the same temperature.

~~~quiz
title = "Check your understanding: Sections 1 and 2"
[[q]]
type = "choice"
prompt = "A taster wearing a nose clip can still detect"
options = ["blueberry and chocolate notes", "sourness and bitterness", "neither"]
answer = 1
explain = "Taste qualities are sensed in the mouth. Descriptors such as blueberry are retronasal aroma and need airflow to the nose."

[[q]]
type = "numeric"
prompt = "A taster misses caffeine at 0.15 g/L and identifies it from 0.30 g/L upward. What is the best-estimate threshold in g/L?"
answer = 0.21
tolerance = 0.005
unit = "g/L"
explain = "&radic;(0.15 &times; 0.30) = 0.21 g/L."
~~~

## 3. Is there a difference? Discrimination tests

We return to the opening question: do the two cups differ? A {{discrimination test}} answers exactly that and nothing more.

| Test | Task | Probability of a correct guess |
|---|---|---|
| Triangle (ISO 4120) | Three cups, two alike: pick the odd one | 1/3 |
| Duo-trio | A reference cup and two test cups: pick the test cup that matches the reference | 1/2 |
| Two-alternative forced choice (2-AFC) | Two cups: pick the one with more of a named attribute | 1/2 |
| Tetrad | Four cups, two of each sample: sort them into two pairs | 1/3 |

The {{triangle test}} is the most widely used. Suppose twelve tasters each do one triangle test and eight pick the odd cup. Is that evidence of a difference?

The analysis starts from a {{null hypothesis}}, the assumption that there is no perceptible difference, so that every taster is guessing with a probability of 1/3 of being right. Under that assumption the number of correct answers follows the {{binomial distribution}} (Appendix E.2). The {{p-value}} is the probability, computed under the null hypothesis, of getting a result at least as extreme as the one observed. If it is small, conventionally below 0.05, we reject the null hypothesis and conclude that the samples differ. {fig:triangle} shows the distribution for twelve trials.

~~~figure
id = "triangle"
file = "ch01-triangle.gif"
alt = "Bar chart of the binomial distribution of correct answers out of 12 under guessing, with bars from 8 upwards highlighted as the significant region."
caption = "Number of correct answers out of 12 triangle tests when every taster guesses. The shaded bars, 8 or more, together have probability 0.019: rarely produced by guessing alone."
~~~

The probability of eight or more correct answers by guessing is 0.019, so eight of twelve is significant. Seven of twelve would give 0.066, not significant. Six of twelve feels convincing but happens by luck 18% of the time.

~~~widget triangle
n = 12
k = 8
~~~

A significant result says only that some tasters perceived a difference. How many? If a fraction p<sub>d</sub> of the population can genuinely tell the samples apart, those people always answer correctly and the rest guess. The {{proportion of discriminators}}, p<sub>d</sub>, is linked to the proportion of correct answers, p<sub>c</sub>, by

~~~eq pc
formula = "p<sub>c</sub> = 1/3 + (2/3) p<sub>d</sub>"
where = [["p<sub>c</sub>", "probability of a correct answer", "fraction"], ["p<sub>d</sub>", "proportion of the population who perceive the difference", "fraction"]]
~~~

Eight correct of twelve gives p<sub>c</sub> = 0.667 and, rearranging {eq:pc}, p<sub>d</sub> = (3p<sub>c</sub> &minus; 1)/2 = 0.50, with wide uncertainty at this sample size.

~~~quiz
title = "Check your understanding: Section 3"
[[q]]
type = "numeric"
prompt = "A triangle test gives 14 correct out of 30. Estimate the proportion of discriminators."
answer = 0.20
tolerance = 0.01
unit = ""
explain = "p<sub>c</sub> = 14/30 = 0.467; p<sub>d</sub> = (3 &times; 0.467 &minus; 1)/2 = 0.20."

[[q]]
type = "choice"
prompt = "In a duo-trio test the probability of a correct guess is"
options = ["1/3", "1/2", "1/4"]
answer = 1
explain = "The taster chooses between two test cups, one of which matches the reference."
~~~

## 4. Planning a test: how many trials?

A test that finds no difference is only informative if it could have found one. The {{statistical power}} of a test is the probability that it detects a difference of a stated size. With the triangle test, the size is naturally expressed as p<sub>d</sub>.

~~~example Planning a triangle test
Problem. How many triangle trials are needed to detect p<sub>d</sub> = 0.30 with 80% power, testing at the 0.05 level?

Plan. If p<sub>d</sub> = 0.30, {eq:pc} gives p<sub>c</sub> = 1/3 + (2/3)(0.30) = 0.533. For each candidate number of trials n, find the smallest number of correct answers that would be significant under guessing (p = 1/3), then compute the probability of reaching it when p = 0.533. Increase n until that probability reaches 0.80.

Solution. With 24 trials, 13 correct are needed and the probability of reaching 13 when p = 0.533 is only 0.55. The first n that reaches 0.80 is 40 trials, needing 19 correct.

Check. More trials are needed than most cafés would run. A café-scale test of 6 to 12 trials can detect only large differences.
~~~

~~~widget triangle-power
~~~

The tetrad and 2-AFC tests detect the same sensory difference with fewer trials than the triangle, because the taster's decision wastes less information. The 2-AFC requires naming the attribute in advance, so it suits a question such as "is cup A more sour?" but not "do they differ at all?". A method-independent measure of the size of a sensory difference, called d', is introduced in Appendix E.7.

~~~misconception
"We ran a triangle test with six staff and found no difference, so the two grinders are equivalent." With six trials, five correct are needed for significance, and if 30% of people could perceive the difference the chance of five or more correct is about 0.14. Such a test usually misses a real difference. Failure to find a difference is evidence of equivalence only when the test had high power for the smallest difference that matters.
~~~

## 5. Describing and judging: the Coffee Value Assessment

Discrimination tests ask whether samples differ. Two other kinds of test ask how. {{Descriptive analysis}} asks which attributes are present and how strong they are. {{Affective test|Affective tests}} ask how much a person likes the result. A trained panel can agree that a coffee is intensely acidic and still disagree on whether the acidity is good, so the two questions should be kept apart.

In 2024 the Specialty Coffee Association adopted a new cupping standard, the {{Coffee Value Assessment}} (CVA), in three documents [1]:

- SCA 102-2024 specifies sample preparation: roast, grind, ratio, water and timing.
- SCA 103-2024 specifies descriptive assessment: intensity ratings of fragrance, aroma, flavour, aftertaste, acidity, sweetness and mouthfeel, plus a checklist of descriptors.
- SCA 104-2024 specifies affective assessment: the cupper's judgement of quality, recorded separately.

The previous SCAA form (2004) gave each attribute one score that mixed intensity and quality, so a high acidity score could mean either "very acidic" or "pleasantly acidic". Separating the two follows standard practice in sensory science: trained cuppers should agree on description, while preferences are expected to differ. The SCA's Q Grader programme, which certifies cuppers, now also includes physical assessment of green coffee and extrinsic information such as origin and processing [2].

Descriptive profiling outside coffee cupping follows ISO 13299, and panellists are selected and trained under ISO 8586 [3]. Panel data are analysed by {{analysis of variance}} (ANOVA, Appendix E.4) with panellist and sample as factors. If panellists rank the samples differently from one another, the analysis shows it as a panellist-by-sample interaction, and the panel needs recalibration.

## 6. A shared vocabulary: lexicons and flavour wheels

Descriptive analysis needs words that mean the same thing to every taster. A {{sensory lexicon}} provides them: a list of attributes, each with a written definition, a physical reference sample and an intensity anchor. The World Coffee Research lexicon was developed by trained panels at Kansas State University and UC Davis.

The SCA and WCR Coffee Taster's Flavor Wheel (2016) was built from that lexicon by a statistical method [4]. Panellists sorted the lexicon's attributes into groups of similar items. Counting how often each pair of attributes was grouped together gave a measure of similarity, and {{hierarchical clustering}}, a method that repeatedly merges the most similar items into groups, produced a tree. The branches of that tree became the tiers of the wheel. An attribute's position therefore reflects measured similarity rather than an editor's judgement, and the wheel is only as good as the lexicon behind it.

## 7. What is in the cup

We now turn from the instrument to what it measures. A filter coffee is 98.5 to 98.8% water. The remaining 1.2 to 1.5% contains several hundred compounds, but a few groups account for most of what tasters notice. Each group below is defined here and treated chemically in Chapter 8.

| Group | What it is | Sensory role |
|---|---|---|
| Organic acids | small molecules with an acidic group: citric, malic, acetic, quinic, phosphoric | sourness |
| Chlorogenic acids | compounds made of caffeic acid joined to quinic acid; abundant in green coffee | bitterness and astringency after roasting |
| Lactones of chlorogenic acids | ring-shaped products formed from chlorogenic acids in roasting | bitterness |
| Alkaloids | nitrogen-containing compounds: caffeine, trigonelline | some bitterness |
| Carbohydrates | dissolved polysaccharides, traces of sugar | body |
| Melanoidins | brown polymers formed in roasting | colour, body, astringency |
| Lipids | oils, including cafestol and kahweol | body in unfiltered coffee |
| Volatiles | aroma compounds such as 2-furfurylthiol and guaiacol | aroma |

Green arabica contains roughly 4 to 12% {{chlorogenic acids}} and 0.6 to 2.0% trigonelline by dry mass [5]. Caffeine is about 1.2% in arabica and 2.2% in robusta. Roasting converts part of the chlorogenic acids to {{chlorogenic acid lactones|lactones}}, ring-shaped molecules formed when an acid group and an alcohol group in the same molecule join and release water (Appendix B.3). Roasting also destroys much of the trigonelline and builds {{melanoidins}} and volatiles. Chapter 8 follows each of these reactions.

## 8. Acids and sourness

Of all the attributes tasters notice, sourness has the clearest link to a measurable quantity. A meta-analysis (a study that combines the data of many studies) covered 129 publications and more than 8,000 data points on coffee acids [6]. Most of the data concerned chlorogenic acids; only 22% concerned other organic acids. Citric and malic acids dominate the organic acids of green coffee and are destroyed by roasting. Acetic acid increases during roasting, largely from the breakdown of sugar. In arabica, organic acids tended to be lowest at medium roast, but roast level did not predict individual acid concentrations in general. Few studies link individual acids to sensory scores.

Total acid gives the stronger result. In drip coffee, titratable acidity (Chapter 0, Section 6) predicted trained-panel sourness far better than pH did. Titratable acidity rose with TDS, and sourness was highest in strong brews with low extraction yield (Chapter 2A) [7].

Acid identity still matters, for three reasons. Acids differ in how much of each remains undissociated at brew pH near 5 (Appendix A.5). They differ in taste quality: citric and malic acids read as fruit, phosphoric as sharp, acetic as vinegar above low levels. And acetic acid is volatile, so it is smelled as well as tasted. Two brews with equal titratable acidity can therefore differ in the character of their sourness. The minerals in brew water also neutralise part of the acid before it is tasted (Chapter 3).

## 9. Bitterness

~~~misconception
"Coffee is bitter because of caffeine." Decaffeinated coffee is still bitter, and receptor studies show why. Coffee's bitter compounds act mainly through two bitter receptors, TAS2R43 and TAS2R46. In laboratory cells carrying TAS2R43, caffeine needed about 30 times the concentration of mozambioside, a bitter compound from arabica, to produce the same response. It needed about 300 times the concentration of bengalensol, which forms from mozambioside during roasting [8]. Caffeine contributes to bitterness, but it is one of the weakest bitter compounds in the cup.
~~~

The same study found that cafestol and kahweol, two oil-soluble compounds of the class called {{diterpene|diterpenes}}, also activated the receptors, and that kahweol partly blocked the response to mozambioside. Many people carry a version of the TAS2R43 gene that does not produce a working receptor, which is one source of genuine differences between tasters [8]. In 2026 a structure of TAS2R43 determined by cryo-electron microscopy, a method that images frozen proteins at near-atomic resolution, showed it binding caffeine, chlorogenic acid, catechol, cafestol and kahweol [9].

Most of coffee's bitterness is created by roasting. Lactones of chlorogenic acids dominate in light to medium roasts. In dark roasts they break down further into compounds called phenylindanes, whose bitterness is harsher and lingers longer (Chapter 8).

~~~evidence
Experiments with receptors in cultured cells show which molecules can trigger a receptor and at what concentration. They do not show how bitter a cup tastes, which depends on concentrations, interactions and the taster. Linking the two requires taste tests on coffee itself. The receptor work establishes that caffeine is a weak trigger and that several roast compounds are strong ones; it does not divide up the bitterness of a particular cup.
~~~

## 10. Sweetness

~~~misconception
"Coffee tastes sweet because of its sugar." Brewed coffee contains about 100 mg/L of sugars, about one twentieth of the sucrose detection threshold of roughly 2,000 mg/L. In a study with 125 trained tasters, coffees still differed in rated sweetness by up to 6 points on a 15-point scale, and the coffees with the most sugar were rated least sweet [10].
~~~

Wearing nose clips shrank the range of sweetness ratings from 6 to 3.6 points [10]. Aroma therefore contributes: caramel, vanilla and fruit volatiles are learned as sweet and raise rated sweetness. But substantial differences remained without aroma, so some non-volatile compounds that are not sugars must also contribute; they have not yet been identified. Low bitterness and moderate sourness may let sweetness be noticed more easily, but that remains a hypothesis.

~~~quiz
title = "Check your understanding: Sections 7 to 10"
[[q]]
type = "choice"
prompt = "Which measurement best predicts trained-panel sourness in drip coffee?"
options = ["Titratable acidity", "pH", "Caffeine concentration"]
answer = 0
explain = "Titratable acidity tracked sourness far better than pH in the UC Davis drip coffee work."

[[q]]
type = "choice"
prompt = "In receptor experiments, caffeine activated the bitter receptor TAS2R43"
options = ["more strongly than any roast compound", "only at much higher concentrations than mozambioside or bengalensol", "not at all"]
answer = 1
explain = "About 30 and 300 times higher concentrations were needed for the same response."
~~~

## 11. Body and mouthfeel

The usual account of body is physical: thickness comes from {{viscosity}} (resistance to flow) and from suspended material such as oil droplets, fine particles and dissolved polysaccharides. Paper filters remove most oil and fines, so a paper-filtered cup feels thinner than the same coffee from a French press.

That account leaves a puzzle: the viscosity of brewed coffee differs from that of water by only a few percent. Work at Ohio State University with panels of certified cuppers divided mouthfeel into chalkiness, mouth-coating, astringency and thickness, and linked these attributes to particular compounds [11]. Astringency was associated with fractions of the melanoidins, and other attributes with two chlorogenic acids, 3- and 4-caffeoylquinic acid, at concentrations where they could act on the mouth's sensory nerves rather than by thickening the liquid.

~~~evidence
The chemical account of mouthfeel comes mainly from one research group and was first reported at a scientific meeting. It is plausible and consistent with the small differences in viscosity, but it needs independent replication. Expect the balance between the physical and chemical accounts to be revised.
~~~

## 12. Aroma: finding the compounds that matter

More than 1,000 volatile compounds have been identified in roasted coffee, but only a few dozen shape its aroma. Finding them takes methods that combine chemical analysis with the human nose.

In {{gas chromatography}} (GC), a mixture of volatiles is carried by a gas through a long, thin column, and each compound leaves the column at its own characteristic time (Appendix D.6). In {{gas chromatography-olfactometry}} (GC-O), the gas leaving the column is split: part goes to a detector, and part to a sniffing port where a trained assessor records each odour and when it appears.

{{Aroma extract dilution analysis}} (AEDA) repeats GC-O on a series of dilutions of the aroma extract, usually halving the concentration each time. A compound's {{flavour dilution factor}} is the highest dilution at which it can still be smelled:

~~~eq fd
formula = "FD = 2<sup>n</sup>"
where = [["FD", "flavour dilution factor", "dimensionless"], ["n", "number of twofold dilutions after which the odour is still detected", ""]]
~~~

A second screening measure compares a compound's concentration with its {{odour threshold}}, the lowest concentration at which it can be smelled in a stated medium. The ratio is the {{odour activity value}}:

~~~eq oav
formula = "OAV = c / c<sub>t</sub>"
where = [["OAV", "odour activity value", "dimensionless"], ["c", "concentration of the compound in the coffee", "&micro;g/L"], ["c<sub>t</sub>", "odour threshold of the compound in the same medium", "&micro;g/L"]]
~~~

A compound with OAV above 1 is present above its threshold and may contribute to aroma.

~~~example Comparing two odorants
Problem. In a brew, 2-furfurylthiol is present at 20 &micro;g/L and its odour threshold in water is 0.01 &micro;g/L. Guaiacol is present at 3,000 &micro;g/L with a threshold of 2.5 &micro;g/L. Which is likely to matter more?

Plan. Compute each odour activity value with {eq:oav}.

Solution. 2-Furfurylthiol: 20/0.01 = 2,000. Guaiacol: 3,000/2.5 = 1,200. The thiol has the higher OAV despite being present at 150 times lower concentration.

Check. Both values are far above 1, so both are likely to matter. The values in this example are illustrative orders of magnitude, not one coffee's measurements.
~~~

~~~widget oav
~~~

Both screening methods have limits. They ignore how quickly perceived intensity rises above threshold, and they ignore interactions in which one odour masks or enhances another. The confirmation is a {{recombination test}}: a model mixture is made from pure compounds at the concentrations measured in the coffee and compared with the real coffee. In an {{omission test}}, one compound at a time is left out of the mixture and tasters perform triangle tests to see whether its absence is noticed.

In filtered coffee, AEDA found 40 odour-active compounds. Among the most potent were (E)-&beta;-damascenone, methional, 3-mercapto-3-methylbutyl formate, furaneol and several phenols. 2-Furfurylthiol, 2-methyl-3-furanthiol and several phenols were far more concentrated in filtered coffee than in instant coffee [12]. A 2025 study of commercial medium-roast arabica coffees found 25 compounds with OAV above 1, including furaneol, 4-vinylguaiacol, guaiacol, furfural, isovaleric acid and linalool, and confirmed guaiacol, furaneol and trimethylpyrazine by omission tests [13]. That study reported no thiols. Thiols are present in trace amounts and are easily lost or missed without a dedicated extraction method (Appendix D.7). The two lists disagree mainly because of method, so read the methods section of an odorant study before its results.

## Looking ahead

We now know what tasters can detect, how to test whether they detect it reliably, and which groups of compounds produce sourness, bitterness, aroma and body. All of those compounds reach the cup by dissolving out of ground coffee into water. Chapter 2A asks how they do so, how far the process can go, and how fast.

## Key equations

~~~equations
[[eq]]
name = "Best-estimate threshold"
formula = "BET = &radic;(c<sub>miss</sub> c<sub>hit</sub>)"
note = "Equation 1.1."
[[eq]]
name = "Triangle test, correct rate"
formula = "p<sub>c</sub> = 1/3 + (2/3)p<sub>d</sub>"
note = "Equation 1.2."
[[eq]]
name = "Flavour dilution factor"
formula = "FD = 2<sup>n</sup>"
note = "Equation 1.3."
[[eq]]
name = "Odour activity value"
formula = "OAV = c / c<sub>t</sub>"
note = "Equation 1.4. Same medium and units."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "numeric"
prompt = "A duo-trio test has a guessing probability of 1/2. With 20 trials, how many correct answers are needed for significance at 0.05 (one-sided)?"
answer = 15
tolerance = 0
unit = ""
explain = "P(X &ge; 15 | n = 20, p = 0.5) = 0.021; P(X &ge; 14) = 0.058."

[[q]]
type = "choice"
prompt = "With nose clips on, trained tasters' differences in coffee sweetness were"
options = ["eliminated", "smaller but still present", "larger than without clips"]
answer = 1
explain = "The range fell from 6 to 3.6 points on a 15-point scale, so aroma contributes but is not the whole story."

[[q]]
type = "choice"
prompt = "What does SCA 104-2024 cover?"
options = ["Sample preparation", "Descriptive assessment", "Affective assessment of quality"]
answer = 2
explain = "SCA 102 covers preparation, 103 description and 104 the cupper's judgement of quality."

[[q]]
type = "choice"
prompt = "Why can two odorant studies of similar coffees disagree about thiols?"
options = ["Thiols vary randomly between cups", "Thiols are at trace levels and need dedicated extraction and detection", "Thiols have no odour"]
answer = 1
explain = "Without a thiol-specific method, thiols are lost or fall below detection."
~~~

## Labs

~~~lab
id = "L1.1"
title = "Thresholds for four tastants"
aim = "Measure your detection thresholds for citric acid, malic acid, caffeine and sucrose by ascending forced choice."
safety = ["Use food-grade chemicals, weigh them in a clean area, and spit rather than swallow. Do not swallow caffeine solutions above 1 g/L in quantity."]
materials = ["Food-grade citric acid, malic acid, caffeine and sucrose", "Balance reading to 0.01 g", "Brew water (the same water for blanks and solutions)", "Small coded cups"]
method = [
  "Prepare a stock of each tastant and dilute it in factors of two to give six concentrations (for example citric acid 0.025 to 0.8 g/L; sucrose 0.5 to 16 g/L).",
  "At each step, a partner presents three coded cups, one spiked, in random positions. Identify the spiked cup.",
  "Work from the lowest to the highest concentration and record each answer as correct or incorrect.",
]
columns = ["Tastant", "Concentration (g/L)", "Correct?", "Notes"]
rows = 24
analysis = [
  "Compute your best-estimate threshold for each tastant.",
  "Pool the class results: compute the geometric mean for each tastant and the spread in factors of two.",
]
expected = "Individual thresholds spread over a factor of four or more; sucrose thresholds near or above 2 g/L."
~~~

~~~lab
id = "L1.2"
title = "A properly sized triangle test"
aim = "Plan a triangle test for a stated difference, run it, and interpret the p-value and the proportion of discriminators."
materials = ["One coffee brewed two ways (for example with two waters or at two grind settings)", "Coded cups", "A randomised order sheet"]
method = [
  "Decide the smallest proportion of discriminators worth detecting, and use the power calculator to choose the number of trials.",
  "Use each of the six serving orders (AAB, ABA, BAA, BBA, BAB, ABB) equally often.",
  "Serve at a fixed temperature and record each assessor's choice.",
]
columns = ["Trial", "Order", "Correct?"]
rows = 24
analysis = [
  "Compute the p-value and the proportion of discriminators.",
  "If the result is not significant, state the proportion of discriminators your test could have detected with 80% power.",
]
~~~

~~~lab
id = "L1.3"
title = "Nose-clip sweetness"
aim = "Repeat at small scale the finding that aroma contributes to coffee sweetness."
materials = ["Three coffees of different origin or roast", "Nose clips", "Sucrose references at 1, 2 and 4 g/L in water"]
method = [
  "Calibrate on the sucrose references, anchoring them at 2, 4 and 7 on a 0 to 15 scale.",
  "Rate the sweetness of each coffee, coded and in random order, first with nose clips and then without, in separate sessions.",
]
columns = ["Taster", "Coffee", "Sweetness, clip", "Sweetness, no clip"]
rows = 12
analysis = [
  "Compare the range of mean sweetness across coffees with and without clips.",
  "Use a paired t-test on each taster's clip-minus-no-clip difference (Appendix E.3).",
]
expected = "Ratings and their range fall with nose clips; some differences between coffees may remain."
~~~

~~~lab
id = "L1.4"
title = "A small descriptive profile"
aim = "Profile three coffees on anchored scales and test whether the panel repeats itself."
materials = ["Three coffees brewed to the same TDS and extraction yield", "Reference anchors: citric acid (sour), caffeine (bitter), sucrose (sweet)"]
method = [
  "Agree on five attributes and their reference anchors.",
  "Each panellist rates every coffee twice, in separate sessions, under random codes.",
]
columns = ["Panellist", "Session", "Coffee", "Sour", "Bitter", "Sweet", "Body", "Aftertaste"]
rows = 18
analysis = [
  "Run a one-way ANOVA for each attribute across the three coffees.",
  "For each panellist, compare the scores from sessions 1 and 2. Who is most repeatable?",
]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "Name the receptor or channel associated with each of sweet, umami, bitter and sour taste."
solution = "Sweet: T1R2 with T1R3. Umami: T1R1 with T1R3. Bitter: the TAS2R family of about 25 receptors. Sour: the hydrogen-ion channel OTOP1."

[[p]]
tier = "A"
section = "1"
prompt = "Explain why pinching your nose removes 'blueberry' from a coffee but not its sourness."
solution = "Blueberry is a retronasal aroma: blocking airflow through the nose stops the volatiles reaching the olfactory receptors. Sourness is a taste sensed in the mouth and is unaffected."

[[p]]
tier = "A"
section = "5"
prompt = "What is the difference between SCA 103 and SCA 104?"
solution = "SCA 103 covers descriptive assessment (which attributes are present and how intense they are); SCA 104 covers affective assessment (the cupper's judgement of quality). The CVA keeps them separate."

[[p]]
tier = "B"
section = "2"
prompt = "Five tasters have sucrose thresholds of 1.0, 2.0, 2.0, 4.0 and 8.0 g/L. Compute the group threshold as a geometric mean, and explain why the arithmetic mean is a poor summary."
solution = "The product is 128; its fifth root is 2.64 g/L. The arithmetic mean, 3.4 g/L, is pulled up by the single high value. Thresholds spread by factors rather than by fixed amounts, so the geometric mean is the appropriate average."

[[p]]
tier = "B"
section = "3"
prompt = "In a 2-AFC test, 15 of 20 assessors choose the more sour cup correctly. Compute the one-sided p-value."
solution = "Under guessing, p = 0.5. P(X &ge; 15 | n = 20) = 0.021, which is significant at 0.05."

[[p]]
tier = "B"
section = "3"
prompt = "Twelve tasters run a triangle test and seven are correct. Is the result significant at 0.05, and what is the estimated proportion of discriminators?"
solution = "P(X &ge; 7 | n = 12, p = 1/3) = 0.066, not significant. p<sub>c</sub> = 0.583, so p<sub>d</sub> = (3 &times; 0.583 &minus; 1)/2 = 0.375, but with twelve trials this estimate is very uncertain."

[[p]]
tier = "B"
section = "12"
prompt = "Compute odour activity values for compound X at 50 &micro;g/L with threshold 0.5 &micro;g/L and compound Y at 5,000 &micro;g/L with threshold 200 &micro;g/L. Rank them, and state one limitation of the ranking."
solution = "X: 100. Y: 25. X ranks higher. The OAV ignores how fast intensity rises above threshold and ignores masking and enhancement between odours, so a recombination or omission test is needed to confirm the ranking."

[[p]]
tier = "B"
section = "12"
prompt = "In AEDA, an odorant is still detected after 9 twofold dilutions but not after 10. What is its flavour dilution factor?"
solution = "2<sup>9</sup> = 512."

[[p]]
tier = "B"
section = "10"
prompt = "Brewed coffee contains about 100 mg/L of sugars. By what factor is this below a sucrose threshold of 2 g/L, and what does that imply?"
solution = "2,000/100 = 20. Sugars alone cannot explain rated sweetness; aroma and non-sugar compounds must contribute, as the nose-clip study found."

[[p]]
tier = "B"
section = "8"
prompt = "Using the pK<sub>a</sub> values in Appendix A.5, what fraction of acetic acid and of malic acid (first acidic group) is undissociated at pH 5.0? Which acid contributes more neutral molecules per mole?"
solution = "Acetic acid (pK<sub>a</sub> 4.76): 1/(1 + 10<sup>0.24</sup>) = 0.365. Malic acid (pK<sub>a</sub> 3.40): 1/(1 + 10<sup>1.6</sup>) = 0.025. Acetic acid contributes far more neutral molecules, which can evaporate and be smelled."

[[p]]
tier = "C"
section = "4"
prompt = "A café's triangle test with six staff finds no significant difference between two grinders. The manager concludes they are equivalent. Critique the conclusion and propose a better test."
solution = "Six trials need five correct for significance, and the probability of five or more correct is only about 0.14 if 30% of people perceive the difference. The test had little power, so it could not show equivalence. A better test uses about 40 trials (pooled across sessions) for p<sub>d</sub> = 0.3, or a similarity test designed under ISO 4120 with a stated maximum acceptable p<sub>d</sub>."

[[p]]
tier = "C"
section = "9"
prompt = "Design a study to test whether TAS2R43 genotype affects rated coffee bitterness. Include your reasoning about sample size."
solution = "Genotype participants for working and non-working TAS2R43 variants. Have each rate the bitterness of a light and a dark roast, and of caffeine and mozambioside solutions, blind and on anchored scales. Compare groups with a two-sample test, or a model with genotype and sample as factors. For a medium effect (a difference of half a standard deviation) about 63 participants per group are needed (Appendix E.6); recruit to balance the groups, since variant frequencies differ."

[[p]]
tier = "C"
section = "12"
prompt = "Two published key-odorant lists for medium-roast arabica disagree about thiols. Give three methodological reasons and propose how to resolve the disagreement."
solution = "Thiols are present at trace levels and need selective enrichment; they oxidise and bind to other compounds during sample preparation; and general GC detectors may lack the sensitivity without sulphur-selective detection. Resolve it by analysing one coffee with a thiol-specific extraction and stable isotope dilution analysis (Appendix D.7), followed by recombination and omission tests."

[[p]]
tier = "C"
section = "10"
prompt = "A café claims its new roast is sweeter. Design a test that separates aroma-driven sweetness from taste-driven sweetness."
solution = "Brew the old and new roasts to equal TDS and extraction yield. Have tasters rate sweetness against sucrose anchors, under random codes, with and without nose clips, in balanced sessions. Analyse with a two-way ANOVA (roast and clip). A roast effect present only without clips points to aroma; one that persists with clips points to non-volatile compounds."

[[p]]
tier = "C"
section = "5"
prompt = "Why does the 2004 SCAA form's combination of intensity and quality in one score make cuppers hard to compare?"
solution = "A score can be high because an attribute is intense or because it is liked. Cuppers who agree on intensity but differ in preference produce different numbers, and the numbers cannot reveal which kind of disagreement occurred. Separating description from preference lets agreement on description be measured and preferences be studied in their own right."
~~~

## References

1. SCA, The SCA officially adopts CVA descriptive and affective assessments as new cupping standards (SCA 102-2024, 103-2024, 104-2024).
2. SCA, Evolved Q Grader program, education.sca.coffee/q-grader.
3. ISO 4120:2021 (triangle test); ISO 13299 (sensory profiling); ISO 8586 (selection and training of assessors).
4. Spencer M. et al. (2016), Using single free sorting and multivariate exploratory methods to design a new coffee taster's flavor wheel, *Journal of Food Science*.
5. Review of chlorogenic acids and trigonelline in coffee, *Trends in Sciences* 23(9):13444 (2026).
6. Yeager S.E., Batali M.E., Guinard J.-X., Ristenpart W.D., Acids in coffee: a review of sensory measurements and meta-analysis of chemical composition, *Critical Reviews in Food Science and Nutrition*.
7. Batali M.E. et al., titratable acidity, pH and sourness in drip coffee; summarised in SCA *25*, issue 15.
8. Lang T., Behrens M., Somoza V. and colleagues (Leibniz-LSB at TUM, 2020), bitter receptors for coffee compounds.
9. Kim Y., Roth B.L. et al. (2026), structure of the bitter taste receptor TAS2R43, *Nature Structural and Molecular Biology*.
10. SCA *25*, issue 22, How sweet coffee tastes: towards an understanding of coffee sweetness.
11. Ohio State University mouthfeel research (2021), reported at the American Chemical Society fall meeting; summarised by Daily Coffee News.
12. Sanz C., Czerny M., Cid C., Schieberle P. (2002), Comparison of potent odorants in a filtered coffee brew and in an instant coffee beverage, *European Food Research and Technology* 214:299.
13. Chen X. et al. (2025), Identification of key aroma-active compounds in commercial coffee using GC-O/AEDA and OAV analysis, *Foods* 14:3192.
