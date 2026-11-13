+++
number = 0
title = "Instruments and measurement"
question = "How do I know a number is good enough to act on?"
objectives = [
  "Report any measurement as a value with an uncertainty, and classify the uncertainty as random or systematic.",
  "Calibrate a scale, a thermometer and a refractometer, and fit a calibration line by least squares.",
  "Propagate uncertainty through sums, products and quotients, including extraction yield.",
  "Measure extraction yield by oven drying and correct it for liquid retained in the grounds.",
  "Titrate a brew to pH 8.2, report titratable acidity, and explain why it differs from pH.",
  "Explain why roast colour and particle size readings belong to one instrument and one method.",
]
prerequisites = ["Appendix A, Sections A.1 and A.2 (amount of substance, concentration)", "Appendix E, Section E.1 (mean and standard deviation)"]
+++
Two baristas compare grinders. With the old grinder their pour-over measured an extraction of 20.3%; with the new one, 20.7%. One says the new grinder extracts more. The other says the difference is noise. Neither can settle the argument, because neither knows how far a single measurement can be trusted.

Every later chapter of this book asks a question of the same kind: did a change in water, grind, rest or roast change the cup? This chapter builds the tools to answer it. We start with what a measurement is, then work through the instruments a coffee laboratory uses (scale, thermometer, refractometer, pH meter) and finish with the human taster, the least precise instrument of all.

## 1. A measurement has two parts

A measured value is incomplete without a statement of how far it might be from the true value. That statement is the {{measurement uncertainty|uncertainty}}, written after the value with a plus-or-minus sign. A dose weighed as 15.0 g on a scale that is reliable to 0.1 g is reported as 15.0 &plusmn; 0.1 g. Omitting the uncertainty does not make a number more exact; it only hides whether two results differ.

### Random and systematic error

Errors come in two kinds, and they need different remedies.

{{Random error}} scatters repeated readings around their average. The last digit of a refractometer flickers; one pour is a gram heavier than the next. Repeating a measurement and averaging reduces random error.

{{Systematic error}} shifts every reading in the same direction by the same amount. A thermometer that reads 2 &deg;C low is wrong by 2 &deg;C every time, so averaging a hundred readings leaves the 2 &deg;C error untouched. Only {{calibration}}, comparing the instrument against something whose value is known, removes systematic error.

Two related words are often confused. {{Precision}} describes how closely repeated readings agree with each other. {{Accuracy}} describes how close their average is to the true value. A refractometer can be precise and inaccurate at once: it gives the same reading every time, and that reading is wrong.

### Quantifying random error

The scatter of n repeated readings x<sub>1</sub>, x<sub>2</sub>, &hellip;, x<sub>n</sub> is measured by the {{sample standard deviation|standard deviation}}, s, defined in Appendix E.1. Roughly two thirds of individual readings fall within one standard deviation of the mean.

The average of n readings is more reliable than any single reading. Its uncertainty is the {{standard error of the mean|standard error}}:

~~~eq se
formula = "SE = s / &radic;n"
where = [["s", "standard deviation of the individual readings", "same unit as the readings"], ["n", "number of independent readings", ""]]
~~~

Because of the square root, halving the standard error requires four times as many readings. {eq:se} applies only to random error; systematic error does not shrink with n.

The uncertainty also fixes how many digits are worth reporting. If a calculation gives an extraction yield of 22.48 &plusmn; 0.38%, the second decimal place carries no information, and the result is written 22.5 &plusmn; 0.4%. Digits that the uncertainty does not support are not {{significant figures}}.

### Combining uncertainties

Most results are calculated from several measurements, each with its own uncertainty. If the errors are independent of one another, they partly cancel, and the combined uncertainty is smaller than the simple sum. The rule for combining them is called addition {{in quadrature}}: square the individual uncertainties, add the squares, and take the square root.

For a sum or difference such as q = x + y, the absolute uncertainties combine:

~~~eq sum
formula = "&delta;q = &radic;[(&delta;x)<sup>2</sup> + (&delta;y)<sup>2</sup>]"
where = [["&delta;x, &delta;y", "uncertainties of the measured quantities x and y", "same unit as x and y"], ["&delta;q", "uncertainty of the result q", ""]]
~~~

For a product or quotient such as q = xy/z, the relative uncertainties (each uncertainty divided by its value) combine:

~~~eq product
formula = "&delta;q/q = &radic;[(&delta;x/x)<sup>2</sup> + (&delta;y/y)<sup>2</sup> + (&delta;z/z)<sup>2</sup>]"
where = [["&delta;q/q", "relative uncertainty of the result", "dimensionless"]]
~~~

~~~deeper Where the quadrature rule comes from
For a result q(x, y) that depends on two independent measurements, a small error &delta;x changes q by about (&part;q/&part;x)&delta;x, and similarly for y. The variance of a sum of independent random quantities is the sum of their variances, so (&delta;q)<sup>2</sup> = (&part;q/&part;x)<sup>2</sup>(&delta;x)<sup>2</sup> + (&part;q/&part;y)<sup>2</sup>(&delta;y)<sup>2</sup>. For q = x + y both derivatives equal 1, which gives {eq:sum}. For q = xy, &part;q/&part;x = y; dividing through by q<sup>2</sup> gives {eq:product}. Simple addition of the uncertainties is the worst case, in which every error happens to push the same way.
~~~

To apply {eq:product} to coffee we need the two quantities that the rest of the book is built on. The {{total dissolved solids}} (TDS) of a brew is the mass of dissolved coffee material per mass of brew; we write it S, for strength, and quote it in percent. The {{extraction yield}} (EY), written E, is the fraction of the dry coffee that ended up dissolved. For a pour-over, where all the dissolved material we count is in the cup,

~~~eq ey
formula = "E = S m<sub>b</sub> / m<sub>c</sub>"
where = [["E", "extraction yield", "fraction; quoted in %"], ["S", "total dissolved solids, as a mass fraction", "fraction; quoted in %"], ["m<sub>b</sub>", "mass of beverage", "g"], ["m<sub>c</sub>", "mass of dry coffee (the dose)", "g"]]
~~~

Chapter 2A derives {eq:ey} and its counterpart for immersion brewing.

~~~example Uncertainty of an extraction yield
Problem. A pour-over uses a dose of 15.0 &plusmn; 0.1 g, gives 250 &plusmn; 1 g of beverage, and reads 1.35 &plusmn; 0.02% TDS. Find the extraction yield and its uncertainty.

Plan. Calculate E from {eq:ey}. It is a product and quotient, so combine relative uncertainties with {eq:product}.

Solution. From {eq:ey}, E = 0.0135 &times; 250 / 15.0 = 0.225, or 22.5%. The relative uncertainties are 0.02/1.35 = 1.48% for TDS, 1/250 = 0.40% for the beverage and 0.1/15.0 = 0.67% for the dose. Combined: &radic;(1.48<sup>2</sup> + 0.40<sup>2</sup> + 0.67<sup>2</sup>) = 1.68%. The absolute uncertainty is 0.0168 &times; 22.5 = 0.38 percentage points, so E = 22.5 &plusmn; 0.4%.

Check. The TDS term dominates, so the combined relative uncertainty should be a little above 1.48%; it is. A better scale would barely help; a better TDS measurement would.
~~~

The answer resolves the baristas' argument: with these instruments, single brews at 20.3% and 20.7% cannot be told apart.

~~~widget ey-uncertainty
~~~

~~~quiz
title = "Check your understanding: Section 1"
[[q]]
type = "choice"
prompt = "Averaging twenty refractometer readings of one sample reduces"
options = ["random error only", "systematic error only", "both random and systematic error"]
answer = 0
explain = "Averaging reduces scatter. A calibration offset is identical in every reading and survives averaging."

[[q]]
type = "numeric"
prompt = "Ten readings have a standard deviation of 0.030%. What is the standard error of their mean, in %?"
answer = 0.0095
tolerance = 0.0005
unit = "%"
explain = "SE = 0.030/&radic;10 = 0.0095%."
~~~

## 2. Mass

Having seen that uncertainty decides what a number can tell us, we turn to the instruments, starting with the one used most often. Coffee and kitchen scales weigh with a {{load cell}}, a metal element whose electrical resistance changes slightly as it bends under load. Four properties decide whether a scale can be trusted.

{{Resolution}} is the smallest step the display shows. It is not the uncertainty: a scale that displays 0.1 g may repeat only to 0.2 g.

{{Linearity}} is whether the reading is proportional to the true mass across the range you use. To check it, weigh objects of known mass at several points across the range. Plotting reading against known mass should give a straight line through the origin with slope 1.

{{Repeatability}} is the standard deviation of repeated weighings of one object, removing and replacing it each time.

{{Drift}} is a slow change of reading with time. Load cells drift as they warm, and a cup of hot coffee loses a few tenths of a gram per minute by evaporation. Weigh beverage promptly, or cover the cup.

## 3. Temperature

A thermometer is calibrated at two fixed points. Crushed ice stirred in water is at 0.0 &deg;C. Water boils at 100 &deg;C only at standard sea-level pressure; the boiling point falls by about 0.3 &deg;C for every 100 m of altitude and shifts by a few tenths of a degree with the weather.

A probe also has a {{response time}}: a thick probe can take thirty seconds to settle. That lag is small in a kettle and large in a roaster, where bean temperature changes by several degrees per minute (Chapter 7).

Location matters as much as calibration. The water in the kettle, the slurry in a dripper and the bed beneath it can differ by several degrees. Always record where a temperature was measured.

## 4. Refractometry

Weighing and temperature are familiar. The measurement of dissolved solids is not, and most of the quantitative work in Chapters 1 to 5 depends on it.

When light passes from one transparent material into another, it changes direction. The amount of bending depends on the {{refractive index}} of each material, a number that rises as more material is dissolved in a liquid. A {{refractometer}} measures the refractive index of a drop of liquid on a glass prism. At the low concentrations of brewed coffee, refractive index rises almost in proportion to dissolved solids, and the instrument converts the reading to TDS with a calibration curve stored by the manufacturer.

Three practical consequences follow.

1. Temperature. Refractive index falls as temperature rises. Instruments compensate, but a hot sample keeps changing as it cools on the prism. Cool samples to room temperature in a sealed cup first, so that no water evaporates.
2. Particles. Suspended particles scatter light and bias the reading. Unfiltered samples (espresso, French press) must be settled or filtered. If you filter, filter every sample the same way and say so, since filtering can also remove some dissolved material.
3. Calibration basis. The conversion is specific to coffee solids. A sugar refractometer reads in degrees Brix (grams of sucrose per 100 g of solution) and needs a separate conversion factor.

~~~misconception
A refractometer that displays 1.35% TDS has measured TDS to &plusmn;0.01%. The display resolution is 0.01%, but the reading also carries the instrument's calibration error, the effect of sample temperature and particles, and the variation between samples drawn from the same brew. Lab L0.3 measures that sampling variation directly; it is usually two or three times the display resolution.
~~~

### Calibrating against standards made by mass

The most reliable way to calibrate a refractometer is to make solutions whose TDS is known because you weighed every component. Liang, Chan and Ristenpart (2021) calibrated their instrument this way, using solutions of instant coffee between 0.5 and 4 g per 100 g of water [1].

The procedure has four steps. First, dry a sample of instant coffee in a low oven to find its moisture content. Second, dissolve weighed amounts of instant coffee in weighed water. Third, compute each standard's true TDS as mass of dry solids divided by total mass. Fourth, read each standard several times.

The readings are then compared with the true values by fitting a straight line. The method of {{least squares}} chooses the slope and intercept that make the sum of squared vertical distances from the points to the line as small as possible (Appendix E.5). The slope shows whether the instrument scales correctly, the intercept shows whether it is correctly zeroed, and the scatter of the points about the line estimates the precision of a single reading. {fig:calibration} shows such a fit.

~~~figure
id = "calibration"
file = "ch00-calibration.gif"
alt = "Calibration line: refractometer reading against known TDS for five standards, with small residuals below."
caption = "Calibration of a refractometer against five standards made by mass (teaching data). The lower panel shows the residuals, the differences between each reading and the fitted line; a good calibration leaves residuals that scatter without pattern."
~~~

~~~widget calibration
~~~

~~~example Correcting a reading with a calibration line
Problem. A calibration gives reading = 0.0057 + 1.0127 &times; true, both in % TDS. A brew reads 1.373%. What is its true TDS, and how large is the error if the calibration is ignored?

Plan. Invert the calibration line to express the true value in terms of the reading.

Solution. true = (reading &minus; 0.0057)/1.0127 = (1.373 &minus; 0.0057)/1.0127 = 1.350%. Using the raw reading would overstate TDS by 0.023 percentage points, a relative error of 1.7%.

Check. A slope 1.3% above 1 and a small positive intercept both push readings up, so the corrected value must be below the reading. It is. The 1.7% systematic error is as large as the whole random uncertainty found in Section 1.
~~~

### Zeroing

The zero point depends on the water used to set it. Zero with your brew water to measure only the solids that came from the coffee. Zero with distilled water to measure everything dissolved, including the minerals already in the water. With 150 mg/L of minerals in the brew water the difference is 0.015% TDS, which shifts extraction yield by about one percentage point at filter strength. Record which you did.

### How well do instruments agree?

Commercial coffee refractometers agree closely with one another but are rarely compared with an independent standard. In one informal comparison of three instruments, two current models differed by at most 0.007% TDS across three brews, while an older model differed from both and scattered more; the comparison had no reference standard [2]. Calibrate your own instrument, keep the method fixed, and report the method with every result.

~~~quiz
title = "Check your understanding: Section 4"
[[q]]
type = "choice"
prompt = "A refractometer is zeroed with distilled water and then reads a brew made with mineral-rich water. The TDS of the coffee alone is"
options = ["overstated by the minerals' contribution", "understated by the minerals' contribution", "unaffected"]
answer = 0
explain = "Zeroing with distilled water counts the dissolved minerals of the brew water as if they came from the coffee."

[[q]]
type = "numeric"
prompt = "A calibration gives reading = 0.010 + 0.985 &times; true. A brew reads 1.380%. What is its true TDS, in %?"
answer = 1.391
tolerance = 0.002
unit = "%"
explain = "true = (1.380 &minus; 0.010)/0.985 = 1.391%."
~~~

## 5. Extraction yield by oven drying

A refractometer infers dissolved solids indirectly. Oven drying measures extraction directly: weigh the dry dose, brew, dry the spent grounds in a low oven (about 100 &deg;C) until their mass stops changing, and weigh them again. The mass lost by the grounds is the mass extracted.

The method contains a hidden bias. Spent grounds hold liquid, and in an immersion brew that liquid has the same TDS as the cup. When the grounds are dried, the solids dissolved in that retained liquid stay behind on the grounds and are counted as unextracted. Let r be the {{brew ratio}}, the mass of brew water per gram of dry coffee, and r<sub>ret</sub> the mass of liquid retained per gram of grounds. Liang and colleagues showed that for immersion brews [1]

~~~eq oven
formula = "E<sub>oven</sub> &asymp; E (1 &minus; r<sub>ret</sub> / r)"
where = [["E<sub>oven</sub>", "extraction yield measured by drying the grounds", "fraction"], ["E", "true extraction yield", "fraction"], ["r<sub>ret</sub>", "liquid retained per gram of grounds", "g/g"], ["r", "brew ratio, water per gram of dry coffee", "g/g"]]
~~~

They also estimated that about 2.3% of the dissolved solids are lost by evaporation during drying [1].

~~~example Correcting an oven-dried yield
Problem. An immersion brew uses 20.0 g of coffee and 320 g of water. After decanting, 280 g of liquid is recovered. Drying the grounds shows a mass loss of 3.70 g. Find the true extraction yield.

Plan. Compute the apparent yield, estimate r<sub>ret</sub> from the missing liquid, then correct with {eq:oven}.

Solution. Apparent yield: 3.70/20.0 = 18.5%. The brew ratio is r = 320/20.0 = 16. About 320 &minus; 280 = 40 g of liquid stayed in the grounds, so r<sub>ret</sub> &asymp; 40/20.0 = 2.0 g/g (neglecting the small mass of dissolved solids). Correction factor: 1 &minus; 2.0/16 = 0.875. True yield: 18.5/0.875 = 21.1%.

Check. The correction raises the yield, as it must, since the bias counts extracted solids as unextracted. Its size, 2.6 percentage points, is larger than every random error in Section 1.
~~~

A second gravimetric method dries a weighed portion of the beverage itself and weighs the residue. It gives TDS directly and is the reference against which refractometers are ultimately judged. It needs an analytical balance reading to 0.001 g, because 20 g of brew at 1.3% leaves only 0.26 g of residue.

## 6. pH and titratable acidity

Sourness is one of the attributes tasters notice first, and two different measurements are used to describe the acid behind it.

{{pH}} measures the hydrogen ions that are free in solution at the moment of measurement: pH = &minus;log<sub>10</sub>[H<sup>+</sup>], with [H<sup>+</sup>] in mol/L (Appendix A.5). Most acids in coffee are {{weak acid|weak acids}}: at the pH of a brew, most of their molecules keep their hydrogen ions rather than releasing them. Those undissociated molecules hardly affect pH.

{{Titratable acidity}} counts them too. In a {{titration}}, a base of known concentration (here sodium hydroxide, NaOH) is added to a measured volume of brew until the pH reaches a chosen {{endpoint}}. Each hydroxide ion removes one hydrogen ion, so the amount of base added equals the number of acidic hydrogens that the acids could give up between the starting pH and the endpoint.

~~~eq ta
formula = "TA = c<sub>NaOH</sub> V<sub>NaOH</sub> / V<sub>sample</sub>"
where = [["TA", "titratable acidity", "mol/L (quoted in mmol/L)"], ["c<sub>NaOH</sub>", "concentration of the sodium hydroxide", "mol/L"], ["V<sub>NaOH</sub>", "volume of sodium hydroxide added to reach the endpoint", "L"], ["V<sub>sample</sub>", "volume of brew titrated", "L"]]
~~~

The UC Davis coffee group titrates 50 mL of brew to pH 8.2 [3]. Across 744 drip brews, titratable acidity rose in proportion to TDS and tracked trained tasters' ratings of sourness closely. pH hardly changed with brewing strength, tracked sourness weakly or not at all, and followed roast level instead [3]. In a separate study, hot and cold brew had almost the same pH (4.85 to 5.13) while hot brew had more titratable acid [4].

~~~misconception
A lower pH means a more sour coffee. Brewed coffee is a {{buffer}}, a solution that resists changes in pH because it contains weak acids together with their salts (Appendix A.6). Making a brew stronger adds acid and salt in roughly fixed proportion, so pH barely moves while the total acid rises. Tasters respond to the total. Across brews of one coffee, titratable acidity predicts sourness; pH does not.
~~~

~~~example From a titration to titratable acidity
Problem. 50.0 mL of brew needs 4.10 mL of 0.0200 mol/L NaOH to reach pH 8.2. Find the titratable acidity.

Plan. Convert the volume of base to moles, then divide by the sample volume ({eq:ta}).

Solution. Moles of NaOH = 4.10 &times; 10<sup>&minus;3</sup> L &times; 0.0200 mol/L = 8.20 &times; 10<sup>&minus;5</sup> mol. TA = 8.20 &times; 10<sup>&minus;5</sup> mol / 0.0500 L = 1.64 &times; 10<sup>&minus;3</sup> mol/L = 1.64 mmol/L.

Check. Units: mol divided by L gives mol/L. Typical drip coffee lies between about 1 and 2.5 mmol/L by this method, so the value is plausible.

Your turn. Some laboratories report TA as milligrams of citric acid per litre. Citric acid (192.1 g/mol) gives up three hydrogen ions. Show that 1.64 mmol/L corresponds to 105 mg/L as citric acid. The convention allows comparison; it does not mean the acid is citric.
~~~

A pH meter is calibrated with two {{buffer solution|buffer solutions}} of known pH that bracket the samples (pH 4 and 7 for coffee) and rinsed between samples. Its glass electrode responds slowly in dilute solutions and its reading shifts with temperature, so measure all samples at one temperature. Appendix D.2 explains how the electrode works.

~~~quiz
title = "Check your understanding: Section 6"
[[q]]
type = "choice"
prompt = "Two coffees have the same pH, but one tastes much more sour. Which measurement is most likely to differ?"
options = ["Titratable acidity", "Refractive index of the brew water", "Roast colour"]
answer = 0
explain = "Titratable acidity counts all acidic hydrogens up to the endpoint and tracks sourness better than pH."

[[q]]
type = "numeric"
prompt = "50.0 mL of cold brew needs 2.95 mL of 0.0200 mol/L NaOH to reach pH 8.2. What is its titratable acidity in mmol/L?"
answer = 1.18
tolerance = 0.01
unit = "mmol/L"
explain = "2.95 &times; 10<sup>&minus;3</sup> &times; 0.0200 = 5.90 &times; 10<sup>&minus;5</sup> mol in 0.0500 L gives 1.18 mmol/L."
~~~

## 7. Roast colour

Roast degree is usually reported as a colour reading. A {{reflectance}} instrument shines light on a sample, usually ground coffee pressed flat, and measures the fraction reflected. Coffee instruments report on their own scales (Agtron-type numbers or proprietary units). General-purpose colorimeters report CIE L\*a\*b\* coordinates, in which L\* is lightness from 0 (black) to 100 (white) (Appendix D.10). Readings from different instruments are not interchangeable, and grind size and how firmly the sample is pressed both change the number [5].

A colour reading is one number for a bean whose surface and centre can differ (Chapter 7). When researchers want two roasts at the same degree, they therefore match them on two independent measures. In an ETH Zurich study, hot-air roasts made at high and low temperature were matched on L\* (24.06 against 24.02) before any other property was compared. They were also matched on roast loss, the fraction of mass lost in roasting (15.33% against 15.81%) [6]. Record ground colour and mass loss for every roast you make.

## 8. Particle size

A grind is described by its {{particle size distribution}}, the proportion of material in each range of particle sizes. It is measured by sieving, by laser diffraction or by image analysis. Each method counts particles differently, so their results cannot be compared directly, and grinder speed and feed rate also change the result [7]. Low-cost image analysis from a phone camera resolves small particles better than sieves [8]. Chapter 4 develops the statistics. Until then, treat "grind size" without a named method as an incomplete measurement.

## 9. The taster as an instrument

A human taster has the same properties as any instrument. A {{threshold}} is the smallest concentration of a substance that the taster can detect. Resolution is the smallest difference the taster can detect between two samples. Drift comes from fatigue and adaptation, and bias from expectation, serving order and even the colour of the cup. International standards specify how to select and train assessors and how to run tasting tests [9]. Chapter 1 shows how to measure thresholds and how to design tests whose results survive statistical analysis. Until then, record the conditions of every tasting: time of day, what you ate beforehand, the serving order and how many samples came before each one.

## Looking ahead

We can now attach an honest uncertainty to a mass, a temperature, a TDS reading and a titration. The one instrument we have not yet calibrated is the person drinking the coffee. Chapter 1 asks what that instrument detects, how its readings can be made reliable, and which substances in the cup it is responding to.

## Key equations

~~~equations
[[eq]]
name = "Standard error of a mean"
formula = "SE = s / &radic;n"
note = "Equation 0.1. Reduces random error only."
[[eq]]
name = "Product or quotient"
formula = "&delta;q/q = &radic;[&Sigma;(&delta;x<sub>i</sub>/x<sub>i</sub>)<sup>2</sup>]"
note = "Equation 0.3. Independent errors."
[[eq]]
name = "Extraction yield, percolation"
formula = "E = S m<sub>b</sub> / m<sub>c</sub>"
note = "Equation 0.4."
[[eq]]
name = "Oven-yield bias, immersion"
formula = "E<sub>oven</sub> &asymp; E (1 &minus; r<sub>ret</sub>/r)"
note = "Equation 0.5. Liang et al. 2021."
[[eq]]
name = "Titratable acidity"
formula = "TA = c<sub>NaOH</sub> V<sub>NaOH</sub> / V<sub>sample</sub>"
note = "Equation 0.6. Endpoint pH 8.2."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "numeric"
prompt = "A brew has dose 20.0 &plusmn; 0.1 g, beverage 320 &plusmn; 2 g and TDS 1.30 &plusmn; 0.03%. What is the relative uncertainty of its extraction yield, in percent?"
answer = 2.4
tolerance = 0.15
unit = "%"
explain = "Relative errors 0.50%, 0.63% and 2.31% combine in quadrature to about 2.4%."

[[q]]
type = "choice"
prompt = "An immersion brew is dried to measure extraction. Ignoring the liquid held in the grounds makes the measured yield"
options = ["too high", "too low", "unaffected"]
answer = 1
explain = "Solids dissolved in the retained liquid stay on the grounds and are counted as unextracted."

[[q]]
type = "choice"
prompt = "Which property of a scale is tested by weighing one object ten times?"
options = ["Linearity", "Repeatability", "Resolution"]
answer = 1
explain = "Repeatability is the spread of repeated weighings of one object."

[[q]]
type = "choice"
prompt = "Why do researchers match roasts on both colour and mass loss?"
options = ["Colour readings are too imprecise to use", "Colour describes the ground surface, while mass loss reflects the whole bean, and the two can disagree", "Mass loss is required by the SCA standard"]
answer = 1
explain = "Two roasts can match on one measure and differ on the other, especially when heating rates differ."
~~~

## Labs

~~~lab
id = "L0.1"
title = "Calibrating the instruments"
aim = "Establish the linearity, repeatability and calibration of your scale, thermometer and refractometer."
safety = ["Boiling water scalds: use a stable kettle and a heat-resistant container.", "Wipe up spills near electrical scales."]
materials = ["0.1 g scale and, if available, an analytical balance (0.001 g)", "Objects of known mass or a calibrated flask", "Thermometer, crushed ice, kettle", "Refractometer, instant coffee, distilled water, small sealed cups", "Low oven for the moisture content of the instant coffee"]
method = [
  "Dry 5 g of instant coffee at 100 &deg;C to constant mass to find its moisture content.",
  "Prepare five standards by mass between 0.5 and 3.0% true TDS, correcting for the moisture you measured.",
  "Read each standard three times at room temperature, wiping the prism between readings.",
  "Weigh one object ten times, removing and replacing it each time.",
  "Check the thermometer in stirred ice water and in boiling water; look up the boiling point for your altitude.",
]
columns = ["Standard", "Solids (g)", "Water (g)", "True TDS (%)", "Reading 1", "Reading 2", "Reading 3"]
rows = 6
analysis = [
  "Fit reading against true TDS by least squares with the calibration calculator. Report slope, intercept and residual standard deviation.",
  "Compute the standard deviation of the ten weighings.",
  "Write down the corrections you will apply to future refractometer and thermometer readings.",
]
expected = "Slopes within a few percent of 1 and residual standard deviations of 0.01 to 0.02% TDS are typical of current coffee refractometers."
~~~

~~~lab
id = "L0.2"
title = "Titratable acidity and pH of three brews"
aim = "Measure pH and titratable acidity of brews that differ in strength, and compare them."
safety = ["Sodium hydroxide solution damages eyes: wear eye protection and gloves.", "Label all solutions."]
materials = ["pH meter with pH 4 and 7 buffers", "Burette or 10 mL graduated syringe", "0.0200 mol/L NaOH (standardised; see Appendix D.2)", "50 mL volumetric pipette or graduated cylinder", "Three brews of one coffee at about 1.0, 1.3 and 1.6% TDS"]
method = [
  "Calibrate the pH meter with both buffers.",
  "Cool each brew to room temperature in a covered cup and measure its pH.",
  "Titrate 50.0 mL of each brew with NaOH, stirring, to a stable pH of 8.2; record the volume.",
  "Repeat each titration once.",
]
columns = ["Brew", "TDS (%)", "pH", "NaOH (mL) run 1", "NaOH (mL) run 2", "TA (mmol/L)"]
rows = 4
analysis = [
  "Plot titratable acidity against TDS and against pH.",
  "Which tracks strength more closely? Compare your answer with the UC Davis results in Section 6.",
]
expected = "Titratable acidity should rise roughly in proportion to TDS while pH changes by only a few hundredths."
~~~

~~~lab
id = "L0.3"
title = "Repeatability of one TDS measurement"
aim = "Find how much of the spread in your extraction yields comes from measurement rather than from brewing."
materials = ["One brew", "Refractometer", "A second operator if possible"]
method = [
  "Take ten samples from one well-stirred brew and read each once.",
  "If possible, have a second person repeat the ten readings using their own sampling technique.",
]
columns = ["Sample", "Operator 1 reading", "Operator 2 reading"]
rows = 10
analysis = [
  "Compute the mean and standard deviation for each operator.",
  "Use the standard deviation as the TDS uncertainty in the uncertainty calculator. What is the resulting uncertainty in extraction yield?",
]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "Define precision and accuracy, and give a coffee example of an instrument that is precise but inaccurate."
solution = "Precision is how closely repeated readings agree; accuracy is how close their mean is to the true value. A refractometer zeroed with mineral-rich tap water gives tightly grouped readings that are all about 0.015% TDS too high: precise but inaccurate."

[[p]]
tier = "A"
section = "1"
prompt = "Why does averaging not remove the error of a thermometer that reads 2 &deg;C low?"
solution = "The error is systematic: every reading carries the same offset, so the mean carries it too. Only calibration against a known temperature removes it."

[[p]]
tier = "A"
section = "4"
prompt = "List three reasons a refractometer reading can differ from the true TDS of a brew."
solution = "Sample temperature different from the instrument's reference; suspended particles scattering light; calibration error (slope or zero), including zeroing with water that contains minerals. Sampling variation within the brew adds a fourth."

[[p]]
tier = "B"
section = "1"
prompt = "An espresso uses 18.0 &plusmn; 0.1 g of coffee, yields 40.0 &plusmn; 0.5 g, and reads 9.50 &plusmn; 0.10% TDS. Compute the extraction yield and its uncertainty."
solution = "E = 0.0950 &times; 40.0 / 18.0 = 21.1%. Relative errors: 0.10/9.50 = 1.05%, 0.5/40.0 = 1.25%, 0.1/18.0 = 0.56%. Combined: &radic;(1.05<sup>2</sup> + 1.25<sup>2</sup> + 0.56<sup>2</sup>) = 1.73%, so the uncertainty is 0.0173 &times; 21.1 = 0.37 points. E = 21.1 &plusmn; 0.4%. Here beverage mass matters as much as TDS, because 0.5 g is large relative to 40 g."

[[p]]
tier = "B"
section = "1"
prompt = "You weigh 100 g of water ten times and find a standard deviation of 0.08 g. What is the standard error of the mean, and what does it describe?"
solution = "SE = 0.08/&radic;10 = 0.025 g. It describes the uncertainty of the average of the ten readings. A single weighing still has an uncertainty of about 0.08 g."

[[p]]
tier = "B"
section = "3"
prompt = "At 1,200 m altitude, what boiling-point reading would you expect from a correct thermometer, using 0.3 &deg;C per 100 m?"
solution = "100 &minus; 0.3 &times; 12 = 96.4 &deg;C, to within the weather-dependent variation of about &plusmn;0.5 &deg;C."

[[p]]
tier = "B"
section = "4"
prompt = "You make a standard from 2.000 g of instant coffee containing 3.5% moisture dissolved in 98.000 g of water. What is its true TDS?"
solution = "Dry solids = 2.000 &times; 0.965 = 1.930 g. Total mass = 100.000 g. TDS = 1.930%. The 0.070 g of water in the instant coffee is already part of the total mass."

[[p]]
tier = "B"
section = "4"
prompt = "Your brew water contains 120 mg/L of dissolved solids, and you zero the refractometer with distilled water. By how much does the reading overstate the coffee's own TDS, and what does that do to the extraction yield of a brew reading 1.30%?"
solution = "120 mg/L is about 0.012% by mass. The coffee's own TDS is 1.288%, so extraction yield is overstated by 0.012/1.300 = 0.9% in relative terms, about 0.2 percentage points at 20% EY."

[[p]]
tier = "B"
section = "5"
prompt = "An immersion brew of 15 g in 240 g of water retains 2.2 g of liquid per gram of grounds. By what factor does the oven method underestimate the yield?"
solution = "r = 16 and r<sub>ret</sub> = 2.2. By {eq:oven} the factor is 1 &minus; 2.2/16 = 0.8625. The oven yield is 13.75% too low in relative terms: for example 18.9% instead of 21.9%."

[[p]]
tier = "B"
section = "6"
prompt = "Express a titratable acidity of 1.18 mmol/L as mg/L of citric acid equivalents."
solution = "1.18/3 = 0.393 mmol/L of citric acid; &times; 192.1 g/mol = 75.6 mg/L."

[[p]]
tier = "C"
section = "4"
prompt = "Design a procedure to decide whether two refractometers give different readings for filter coffee. Include sampling, replication and the analysis."
solution = "Prepare several brews spanning the working range. From each, take one well-mixed sample, cool it to room temperature, and read it alternately on both instruments several times, randomising which goes first. For each brew compute the mean difference between instruments and its confidence interval (a paired comparison, Appendix E.3), and plot the difference against TDS to see whether it changes across the range. Include standards made by mass so that at least one instrument can be compared with a known value."

[[p]]
tier = "C"
section = "1"
prompt = "A colleague reports that a new grinder raised their extraction yield from 20.3% to 20.7%. List what you would need to know before believing it."
solution = "The uncertainty of each yield (typically &plusmn;0.3 to 0.4 points per brew), the number of replicate brews and their spread, whether the refractometer and its zeroing were the same, whether beverage mass was weighed promptly, whether the same coffee at the same age was used, and whether the brews with each grinder were interleaved in time. A 0.4 point difference between single brews is within measurement noise."

[[p]]
tier = "C"
section = "7"
prompt = "Why should a roasting experiment match samples on both roast colour and mass loss rather than colour alone?"
solution = "Colour is measured on ground material and depends on the instrument and on sample preparation; mass loss reflects water loss and gas production in the whole bean. Two roasts can match on one and differ on the other, especially when heating rates differ. Matching on both narrows the remaining differences to the variable under study."

[[p]]
tier = "C"
section = "6"
prompt = "Explain why a pH meter can show almost no change across brews of different strength while titratable acidity changes a lot."
solution = "Coffee acids are weak and the brew is buffered by their salts. A stronger brew contains more acid and more salt in roughly constant proportion; the ratio sets pH (Appendix A.6), so pH changes little, while the total acid that can be titrated grows with concentration."

[[p]]
tier = "C"
section = "7"
prompt = "A home roaster reports roast degree only as 'Agtron 65'. What further information would you need to compare that roast with yours?"
solution = "The instrument model and scale, whether the sample was whole bean or ground, how the sample was prepared (grind, pressing, levelling), which calibration tiles were used, and ideally the mass loss. Without these a number from one reflectance instrument cannot be placed on another's scale."
~~~

## References

1. Liang J., Chan K.C., Ristenpart W.D. (2021), An equilibrium desorption model for the strength and extraction yield of full immersion brewed coffee, *Scientific Reports* 11:6904.
2. Socratic Coffee (2015), Measuring total dissolved solids: a refractometer comparison, part II, socraticcoffee.com.
3. Batali M.E., Cotter A.R., Frost S.C., Ristenpart W.D., Guinard J.-X., titratable acidity, pH and sourness in drip brew coffee, *ACS Food Science and Technology*; summarised in SCA *25* magazine, issue 15.
4. Rao N.Z., Fuller M. (2018), Acidity and antioxidant activity of cold brew coffee, *Scientific Reports* 8.
5. Roasted coffee beans characterization through optoelectronic color sensing, Brazilian coffee research repository (sbicafe.ufv.br).
6. Schenker S. (2000), *Investigations on the Hot Air Roasting of Coffee Beans*, Dissertation ETH Zurich No. 13620.
7. Gagn&eacute; J. (2023), What I learned from analyzing 300 particle size distributions for 24 espresso grinders, *Coffee ad Astra*.
8. Home-Barista forum, software for analysing grinder particle size distribution with a phone camera.
9. ISO 4120:2021 Sensory analysis, triangle test; ISO 13299:2016 sensory profile; ISO 8586 selection and training of assessors.
