+++
number = 5
title = "Resting, staling and storage"
question = "What changes in roasted coffee after it leaves the roaster, how fast, and what slows it?"
objectives = [
  "Explain where the carbon dioxide in roasted coffee comes from and estimate its volume.",
  "Fit a Weibull model to degassing data and judge how sensitive the fit is to the assumed total.",
  "Explain how escaping gas affects wetting, bloom and crema.",
  "Describe how aroma is lost from beans and from brewed coffee, particularly 2-furfurylthiol.",
  "Use published freshness indices to design a storage study with replicates.",
  "Extrapolate shelf life with the Arrhenius equation and state its limits.",
]
prerequisites = ["Chapter 1, Section 12 (aroma methods)", "Appendix C, Sections C.1, C.3 and C.7 (first-order kinetics, Arrhenius, gases)"]
+++
A coffee roasted yesterday makes an espresso that foams wildly and tastes sharp and thin. The same coffee two weeks later pulls a calm, balanced shot. Two months later it tastes flat and faintly of cardboard. Three processes are at work here, on three different time scales: gas leaving the bean, aroma compounds leaving or reacting, and oils slowly oxidising.

This chapter separates them. Sections 1 to 3 treat the gas: how much there is, how fast it leaves, and how it disturbs brewing. Sections 4 and 5 treat staling chemistry. Sections 6 to 8 turn the chemistry into practice: packaging, storage after opening, freezing, and predicting shelf life.

## 1. Gas in roasted coffee

Roasting breaks down sugars, amino acids and chlorogenic acids, and many of these reactions release carbon dioxide (Chapter 8). Smaller amounts of carbon monoxide and other gases form too. Part of the gas escapes during roasting; the rest stays trapped in the bean's porous structure, in cells expanded by roasting and connected by narrow channels through their walls. Calculated pressures inside freshly roasted beans are about 550 to 800 kPa, five to eight times atmospheric pressure, and roughly half of the CO<sub>2</sub> formed is estimated to remain in the whole bean when roasting ends [1].

Freshly roasted coffee holds a few milligrams of CO<sub>2</sub> per gram. The ideal gas law (Appendix C.7) converts that to a volume.

~~~example How much gas is in a bag?
Problem. A 250 g bag of fresh coffee holds 6 mg of CO<sub>2</sub> per gram. What volume would that gas occupy at 20 &deg;C and atmospheric pressure?

Plan. Find the mass of CO<sub>2</sub>, convert to moles, and apply pV = nRT.

Solution. Mass: 250 &times; 6 mg = 1.5 g. Amount: 1.5/44.01 = 0.0341 mol. Volume: V = nRT/p = 0.0341 &times; 8.314 &times; 293.15/101,325 = 8.2 &times; 10<sup>&minus;4</sup> m<sup>3</sup>, or 0.82 L.

Check. That is several times the empty space in a typical bag, which is why sealed bags of fresh coffee need a one-way valve (Section 6).
~~~

## 2. How fast gas leaves: degassing kinetics

The loss of CO<sub>2</sub> from roasted coffee over time is called {{degassing}}. Smrke, Yeretzian and colleagues (2018) measured it by weighing: coffee was sealed in a container vented through a fine tube, and the slow loss of mass was logged for weeks [2]. They compared whole and ground arabica and robusta at different roast degrees and speeds. In every case, the cumulative fraction of gas released, F(t), followed a {{Weibull model}}:

~~~eq weibull
formula = "F(t) = 1 &minus; exp[&minus;(t/&tau;)<sup>&beta;</sup>]"
where = [["F", "fraction of the releasable gas already released", "fraction"], ["t", "time since roasting", "h"], ["&tau;", "time at which 63% of the gas has left", "h"], ["&beta;", "shape parameter", "dimensionless"]]
~~~

When &beta; = 1, {eq:weibull} is ordinary first-order release (Appendix C.1), in which a constant fraction of the remaining gas leaves per hour. Values of &beta; below 1 mean the release slows down faster than first order: gas near the surface leaves quickly, while gas deep in the bean lingers. A bean contains gas at many depths, and so many diffusion distances, which produces exactly this shape. Grinding shortens every distance, which is why ground coffee releases most of its gas within minutes to hours. {fig:weibull} compares three values of &beta;.

~~~figure
id = "weibull"
file = "ch05-weibull.gif"
alt = "Three curves of fraction of gas released over 30 days for Weibull shape parameters 0.5, 0.7 and 1.0, all with tau of 120 hours; lower beta rises faster at first and then more slowly."
caption = "Weibull release with &tau; = 120 h. All three curves cross at 63% at 120 h; compare their slopes before and after that point."
~~~

~~~widget degassing
tau = 120
beta = 0.7
minf = 0.6
~~~

### Fitting a Weibull curve

The Weibull model can be turned into a straight line by taking logarithms twice. Rearranging {eq:weibull} gives 1 &minus; F = exp[&minus;(t/&tau;)<sup>&beta;</sup>]. Taking the natural logarithm, &minus;ln(1 &minus; F) = (t/&tau;)<sup>&beta;</sup>. Taking it again:

~~~eq linear
formula = "ln[&minus;ln(1 &minus; F)] = &beta; ln t &minus; &beta; ln &tau;"
~~~

Plotting the left side against ln t gives &beta; as the slope, and &tau; follows from the intercept. There is a catch: computing F requires the total amount of releasable gas, which you do not know until degassing has finished.

~~~example Fitting jar data, and the sensitivity to the total
Problem. A jar of 100 g of whole beans loses 0.05, 0.13, 0.20, 0.28, 0.35 and 0.43 g at 6, 24, 48, 96, 168 and 336 hours after roasting. Fit &beta; and &tau;, assuming the total releasable gas is 0.55 g. Then repeat with 0.50 g.

Plan. Compute F = loss/total at each time, transform with {eq:linear}, and fit a straight line by least squares (the calculator below does this).

Solution. With a total of 0.55 g: &beta; = 0.69 and &tau; = 166 h (6.9 days), with R<sup>2</sup> = 0.997. With 0.50 g: &beta; = 0.73 and &tau; = 128 h, with R<sup>2</sup> = 0.999.

Check. Both fits look excellent, so the quality of the fit cannot tell you which total is right, yet a 10% change in the assumed total shifts &tau; by 30%. Continue measuring until the loss curve flattens, or fit the total as a third parameter by non-linear least squares.
~~~

~~~widget weibull-fit
~~~

## 3. Gas and brewing

Escaping CO<sub>2</sub> gets in the way of water. CO<sub>2</sub> is much less soluble in hot water than in cold (Appendix C.7), so when hot water reaches fresh grounds, gas bubbles out quickly, swells the bed and keeps water away from parts of it. The swelling and bubbling is called the {{bloom}}. Pour-over methods pour a little water first and wait 30 to 45 seconds for the bloom to subside before continuing.

In espresso, CO<sub>2</sub> dissolves in the water under pressure and comes out of solution as the pressure drops at the basket exit. The bubbles, stabilised by dissolved and suspended coffee material, form the foam called {{crema}}. Very fresh coffee gives thick crema; well-rested coffee, less.

Gas also disturbs measurement. Grounds that are still degassing lose mass on the scale, and bubbles in a bed or puck open local channels (Chapter 2B). Many roasters rest coffee for several days before filter brewing and for one to three weeks before espresso, but the right interval depends on the roast and method and is an experimental question (Lab L5.1).

~~~quiz
title = "Check your understanding: Sections 1 to 3"
[[q]]
type = "numeric"
prompt = "For Weibull release with &tau; = 100 h and &beta; = 1, what fraction of the gas has left after 100 h?"
answer = 0.632
tolerance = 0.002
unit = ""
explain = "F = 1 &minus; e<sup>&minus;1</sup> = 0.632."

[[q]]
type = "choice"
prompt = "A Weibull shape parameter &beta; below 1 indicates"
options = ["release that slows faster than first order, with a long tail", "release at a constant rate", "release that speeds up over time"]
answer = 0
explain = "Gas near the surface leaves quickly and gas deep in the bean lingers."
~~~

## 4. Staling chemistry

Degassing is not staling. While gas leaves, a second, slower set of processes changes the aroma: volatile compounds escape or react, and oils oxidise. These changes, together called {{staling}}, are what make old coffee taste flat.

### Measuring freshness with ratios

Marin and colleagues (2008) followed 40 volatile compounds in roasted and ground coffee for a year [3]. Most declined, even at &minus;20 &deg;C. They proposed a {{freshness index}}, the ratio of two compounds that move in opposite directions:

~~~eq index
formula = "I = [2-furfurylthiol] / [hexanal]"
where = [["I", "freshness index; falls as coffee stales", "dimensionless"]]
~~~

2-Furfurylthiol, a sulfur compound with a roasted, coffee-like odour (Chapter 1), declines with storage. Hexanal, a product of oil oxidation with a grassy, cardboard-like odour, rises. The index correlated with panel ratings of aroma freshness (R &ge; 0.80). Ratios of 2-methylfuran to 2-butanone and of methanethiol to hexanal were also useful. A ratio of two compounds moving in opposite directions changes faster than either alone, and analytical errors that affect both, such as small differences in sample mass, partly cancel.

### Why 2-furfurylthiol disappears from the brew

Brewed coffee loses aroma within minutes on a hot plate. Sun, Fisk and colleagues traced the loss of 2-furfurylthiol to two routes [4].

1. Binding to oxidised phenols. Roasting breaks chlorogenic acids into smaller {{phenol|phenols}}, compounds with an OH group on a benzene ring. One of them, hydroxyhydroquinone, reacts with oxygen to form a {{quinone}}, a reactive ring compound that thiols attach to (Appendix B.4). The attached thiol no longer has an odour.
2. Attack by radicals. A {{radical}} is a molecule with an unpaired electron, which makes it highly reactive. Traces of iron react with hydrogen peroxide in the brew to produce hydroxyl radicals (the {{Fenton reaction}}), and these destroy 2-furfurylthiol.

Adding cysteine, which competes for the quinones, and ascorbic acid (vitamin C), which removes radicals, at 0.045 and 0.05 g/L, raised 2-furfurylthiol by 153% and kept 91% of it after 30 minutes. A panel's rating of flavour intensity in the untreated brew fell from 7.5 to 3.9 in those 30 minutes, while the treated brew stayed at 7.1.

~~~evidence
The two loss routes are well established for the brew. In whole beans and grounds the chemistry is slower and also involves oxygen, evaporation and oil oxidation; applying the brew mechanism to storage is reasonable but not demonstrated. The additive experiment shows mechanism; it is not a recommendation to add cysteine to coffee.
~~~

### Oil oxidation

Coffee beans contain 10 to 17% oil, arabica toward the upper end. Unsaturated fatty acids in the oil react with oxygen in a chain reaction carried by radicals, forming aldehydes such as hexanal and, later, rancid and cardboard-like notes. {{Lipid oxidation}} is slower than thiol loss and dominates long-term staling.

## 5. Packaging

Packaging slows staling by keeping oxygen out and aroma in. A {{one-way valve}} lets CO<sub>2</sub> out of a sealed bag without letting air in, so freshly roasted coffee can be sealed immediately. Flushing the bag with nitrogen before sealing lowers the oxygen in the {{headspace}}, the gas-filled space around the beans. Multilayer films containing a metallised or foil layer block oxygen and water vapour; paper and many compostable films do not, and coffee packed in them stales faster.

~~~example Oxygen in a resealed bag
Problem. A half-used bag is resealed with 0.4 L of air in the headspace. How much oxygen is available to react with the coffee at 20 &deg;C?

Plan. Air is 21% oxygen by volume; one mole of gas occupies 24.06 L at 20 &deg;C and atmospheric pressure.

Solution. Oxygen volume: 0.21 &times; 0.4 = 0.084 L. Amount: 0.084/24.06 = 3.5 mmol, or 112 mg.

Check. Every time the bag is opened, that oxygen is replaced. In a tightly sealed bag, CO<sub>2</sub> still leaving the beans dilutes it; in a poorly sealed one, exchange with room air keeps it near 21%.
~~~

## 6. Storage after opening

Smrke and colleagues (2022) compared common ways of storing coffee after opening a valve bag (a screw-cap jar, a clip, adhesive tape and a metal canister) over about nine weeks, with five replicates of each [5]. The ratio of 2-butanone to 2-methylfuran separated the methods best. The screw cap preserved aroma best and the canister worst: the index rose about twice as fast in canisters (slope 0.0667 against 0.0327 per unit time). In canisters and with tape, headspace oxygen reached nearly room-air levels within about a week. Methanethiol fell sharply after opening whatever the method.

~~~misconception
"A metal coffee canister keeps beans fresh." In the 2022 study, canisters preserved aroma worst of the four methods, because their lids let headspace oxygen reach nearly room-air levels within a week. A tight seal matters far more than the container's material or appearance.
~~~

The study measured chemistry, not flavour; no sensory panel was run. Its practical message follows from the oxygen data.

## 7. Freezing

A Penn State study stored beans of six roast levels frozen and at room temperature, analysed their volatiles, and asked 48 consumers to sort ground samples by smell [6]. Frozen dark roasts stayed close to fresh after nine weeks; room-temperature dark roasts did not. Light roasts lost less aroma than dark roasts at room temperature, consistent with their denser, less porous structure.

~~~misconception
"Freezing ruins coffee." Freezing slows every staling reaction, and in the Penn State study frozen dark roasts stayed close to fresh for nine weeks. The real risk is moisture: opening a cold container in warm, humid air makes water condense on the beans. Freeze sealed portions sized for single use, and let each warm to room temperature before opening it.
~~~

Frozen beans ground straight from the freezer also grind finer and with a narrower distribution (Chapter 4).

## 8. Predicting shelf life

Shelf life is often estimated by {{accelerated shelf-life testing}}: samples are stored at raised temperatures so that they stale faster, a quality marker is measured, and the results are extrapolated to the storage temperature with the Arrhenius equation (Appendix C.3). A 2025 study of packaged specialty coffee found that total polyphenols decreased at a constant rate, a {{zero-order}} loss in which the amount lost per day does not depend on how much remains:

~~~eq zero
formula = "C = C<sub>0</sub> &minus; k t"
where = [["C, C<sub>0</sub>", "concentration of the marker now and at the start", "e.g. mg/g"], ["k", "zero-order rate constant", "e.g. mg/(g day)"]]
~~~

The activation energies were about 49 to 118 kJ/mol, depending on the packaging [7].

~~~widget arrhenius
~~~

~~~example Extrapolating to room temperature
Problem. A quality marker's rate constant is 0.010 per day at 25 &deg;C and 0.060 per day at 45 &deg;C. Estimate the activation energy and the rate in a refrigerator at 5 &deg;C.

Plan. Use the two-temperature form of the Arrhenius equation (Appendix C.3) to find E<sub>a</sub>, then extrapolate.

Solution. E<sub>a</sub> = 8.314 &times; ln 6/(1/298.15 &minus; 1/318.15) = 14.90/(2.108 &times; 10<sup>&minus;4</sup>) = 70.7 kJ/mol. At 278.15 K, k = 0.010 &times; exp[&minus;(70,700/8.314)(1/278.15 &minus; 1/298.15)] = 0.010 &times; e<sup>&minus;2.05</sup> = 0.0013 per day.

Your turn. What does this predict for the ratio of shelf lives at 5 &deg;C and 25 &deg;C? Then list one reason the prediction could be wrong (the next paragraph gives two).
~~~

The extrapolation rests on two assumptions that accelerated tests cannot check by themselves: that the same process operates at every temperature, and that the measured marker is the one that limits acceptability. Oil oxidation, aroma loss and moisture uptake have different activation energies, so the process that limits shelf life at 40 &deg;C need not be the one that limits it at 20 &deg;C. Validate the prediction with at least one measurement at the real storage temperature.

## Looking ahead

Everything in Chapters 1 to 5 starts from roasted coffee. The roaster, in turn, starts from green coffee, and the properties of the green bean set the limits of what roasting can do. Chapter 6 asks what the roaster begins with.

## Key equations

~~~equations
[[eq]]
name = "Weibull release"
formula = "F = 1 &minus; exp[&minus;(t/&tau;)<sup>&beta;</sup>]"
note = "Equation 5.1."
[[eq]]
name = "Linearised Weibull"
formula = "ln[&minus;ln(1 &minus; F)] = &beta; ln t &minus; &beta; ln &tau;"
note = "Equation 5.2."
[[eq]]
name = "Freshness index"
formula = "I = [2-furfurylthiol] / [hexanal]"
note = "Equation 5.3. Marin et al."
[[eq]]
name = "Zero-order loss"
formula = "C = C<sub>0</sub> &minus; kt"
note = "Equation 5.4."
[[eq]]
name = "Time to release a fraction F"
formula = "t<sub>F</sub> = &tau; [&minus;ln(1 &minus; F)]<sup>1/&beta;</sup>"
note = "From Equation 5.1."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "choice"
prompt = "Which pair of processes explains the loss of 2-furfurylthiol in brewed coffee?"
options = ["Binding to quinones and destruction by hydroxyl radicals", "Evaporation and hydrolysis", "Oil oxidation and Maillard reactions"]
answer = 0
explain = "Thiols attach to oxidised hydroxyhydroquinone and are destroyed by radicals from the Fenton reaction."

[[q]]
type = "choice"
prompt = "In the 2022 storage study, which method preserved aroma best after opening?"
options = ["Metal canister", "Screw cap", "Adhesive tape"]
answer = 1
explain = "The screw cap kept oxygen out best."

[[q]]
type = "numeric"
prompt = "Using &tau; = 166 h and &beta; = 0.69, how many hours until half the gas has left?"
answer = 97.6
tolerance = 1.5
unit = "h"
explain = "t = 166 &times; (ln 2)<sup>1/0.69</sup> = 166 &times; 0.588 = 97.6 h."
~~~

## Labs

~~~lab
id = "L5.1"
title = "Degassing of two roast degrees by weighing"
aim = "Measure and model CO<sub>2</sub> release from whole beans of two roast degrees."
materials = ["Two roast degrees of one coffee roasted on the same day", "Two jars with lids fitted with a short length of fine tubing", "Balance reading to 0.01 g"]
method = [
  "Seal 100 g of each roast in a jar within an hour of roasting and weigh immediately.",
  "Weigh twice daily for at least 21 days at constant room temperature.",
  "Brew each roast on days 2, 7 and 14 at matched extraction yield and taste them blind.",
]
columns = ["Hours", "Light: mass lost (g)", "Dark: mass lost (g)", "Notes"]
rows = 20
analysis = [
  "Fit &beta; and &tau; for each roast, and test the sensitivity of the fit to the assumed total.",
  "Relate the tasting notes to the fraction of gas released by each brewing day.",
]
expected = "Dark roasts release faster (smaller &tau;); both have &beta; below 1."
~~~

~~~lab
id = "L5.2"
title = "Aroma loss in a standing brew"
aim = "Test whether removing radicals slows aroma loss in hot brewed coffee."
safety = ["Use food-grade ascorbic acid only; taste small amounts and spit."]
materials = ["Batch brewer or large pour-over", "Two thermal carafes, or two cups on a warming plate", "Food-grade ascorbic acid", "Balance reading to 0.01 g"]
method = [
  "Brew 1 L and divide it between two vessels. Add ascorbic acid at 0.05 g/L to one.",
  "Hold both at 70 to 80 &deg;C. At 0, 15 and 30 minutes, run triangle tests between the two and rate aroma intensity.",
]
columns = ["Time (min)", "Triangle correct / n", "Aroma, control", "Aroma, treated"]
rows = 3
analysis = ["Does the difference between treated and control grow with time?"]
~~~

~~~lab
id = "L5.3"
title = "Storage after opening"
aim = "Compare two storage methods over four weeks with replicate bags and blind tasting."
materials = ["Six bags of one roast from one batch", "Clips and screw-cap jars", "Coded cups"]
method = [
  "Open all bags on day 0. Store three closed with clips and decant three into screw-cap jars.",
  "Each week, brew one dose from each container at matched extraction yield and run a triangle test, pooling replicates.",
]
columns = ["Week", "Clip mean aroma", "Jar mean aroma", "Triangle correct / n"]
rows = 5
analysis = ["Plot aroma against week for each method and compare the slopes."]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "2"
prompt = "Explain in one sentence each why ground coffee degasses faster than whole beans and why it stales faster."
solution = "Grinding shortens the distance gas must diffuse from millimetres to tens of micrometres. It also exposes far more surface to oxygen and lets volatiles escape, so oxidation and aroma loss speed up."

[[p]]
tier = "A"
section = "2"
prompt = "What does a Weibull shape parameter of 0.6 tell you that a value of 1 would not?"
solution = "Release slows faster than first order: a fast early loss followed by a long tail, consistent with gas at many different depths."

[[p]]
tier = "A"
section = "7"
prompt = "Why should frozen coffee be warmed to room temperature in its sealed container before opening?"
solution = "Opening cold beans to warm, humid air makes water condense on them, which speeds staling and can affect grinding."

[[p]]
tier = "B"
section = "2"
prompt = "With &tau; = 120 h and &beta; = 0.7, what fraction of the gas has left after 48 h and after 14 days?"
solution = "48 h: 1 &minus; exp[&minus;(0.4)<sup>0.7</sup>] = 1 &minus; exp(&minus;0.527) = 0.410. 336 h: 1 &minus; exp[&minus;(2.8)<sup>0.7</sup>] = 1 &minus; exp(&minus;2.056) = 0.872."

[[p]]
tier = "B"
section = "1"
prompt = "How many litres of CO<sub>2</sub> at 20 &deg;C and atmospheric pressure does 1 kg of coffee at 5 mg/g contain?"
solution = "5 g/44.01 g/mol = 0.1136 mol; V = 0.1136 &times; 0.02406 m<sup>3</sup>/mol = 2.73 &times; 10<sup>&minus;3</sup> m<sup>3</sup>, or 2.7 L."

[[p]]
tier = "B"
section = "4"
prompt = "A freshness index falls linearly from 1.20 to 0.80 in 20 days at 25 &deg;C. Coffee is judged stale at 0.60. Predict the day it becomes stale, and give one reason the prediction might fail."
solution = "The slope is &minus;0.02 per day, so 0.60 is reached after 30 days. The decline may not stay linear (it may slow or speed up as oil oxidation takes over), and staleness may depend on compounds the index does not track."

[[p]]
tier = "B"
section = "8"
prompt = "Using the worked example in Section 8, what is the ratio of shelf lives at 5 &deg;C and 25 &deg;C, assuming shelf life is inversely proportional to k?"
solution = "0.010/0.0013 &asymp; 7.7: about eight times longer in the refrigerator."

[[p]]
tier = "B"
section = "6"
prompt = "In the 2022 storage study the index rose at 0.0667 per unit time in canisters and 0.0327 in screw-cap jars. How much longer does a screw-cap jar take to reach the same change in index?"
solution = "0.0667/0.0327 = 2.04 times longer."

[[p]]
tier = "B"
section = "4"
prompt = "Why is a ratio of two compounds moving in opposite directions a more sensitive freshness index than either compound alone?"
solution = "The ratio changes by the product of the two relative changes, so it moves faster. Analytical errors that affect both compounds, such as sample mass or extraction efficiency, partly cancel."

[[p]]
tier = "C"
section = "4"
prompt = "A café notices that batch brew loses aroma within 30 minutes on a hot plate but keeps better in a thermal carafe. Explain using the two loss routes and temperature."
solution = "A hot plate holds the brew hot with open headspace: the rates of quinone formation, thiol binding and radical reactions all rise with temperature, and volatiles escape. A sealed carafe holds a slightly lower, steady temperature, limits fresh oxygen and keeps volatiles in its headspace, slowing both routes."

[[p]]
tier = "C"
section = "3"
prompt = "Design a study to find the best rest time for an espresso blend. Specify the variables, replication and analysis."
solution = "Roast one batch and store it sealed with a valve. On days 3, 7, 10, 14, 21 and 28, pull three shots at fixed dose and beverage mass, adjusting grind to a fixed time if needed, and record extraction yield. A descriptive panel of five or more tasters rates key attributes and preference blind, in random order. Analyse with ANOVA across days, and plot attributes against the fraction degassed measured in a parallel jar."

[[p]]
tier = "C"
section = "7"
prompt = "Explain why light roasts stale more slowly at room temperature than dark roasts, using structure and chemistry."
solution = "Dark roasts are more porous, with larger pores, so oxygen gets in and volatiles get out faster; their oil migrates to the surface where it oxidises; and they contain more reactive roast products. Light roasts are denser, keep more moisture and keep their oil inside the cells."
~~~

## References

1. Schenker S. (2000), Investigations on the hot air roasting of coffee beans, Dissertation ETH Zurich No. 13620 (literature review of gas formation and pressure).
2. Smrke S. et al. (2018), Time-resolved gravimetric method to assess degassing of roasted coffee, *Journal of Agricultural and Food Chemistry* 66:5293.
3. Marin K., Požrl T., Zlatić E., Plestenjak A. (2008), A new aroma index to determine the aroma quality of roasted and ground coffee during storage, *Food Technology and Biotechnology*.
4. Sun Z., Cui H., Yang N., Ayed C., Zhang X., Fisk I.D., Enhancement of coffee brew aroma through control of the aroma staling pathway of 2-furfurylthiol, University of Nottingham.
5. Smrke S., Adam J., Mühlemann S., Lantz I., Yeretzian C. (2022), Effects of different coffee storage methods on coffee freshness after opening of packages, *Food Packaging and Shelf Life* 33:100893.
6. Cotter A.R., Hopfer H., freezing roasted coffee and aroma retention, *Beverages*; summarised by Penn State News.
7. Fernández Rosillo F. et al. (2025), Polyphenol degradation kinetics of specialty coffee in different presentations, *Foods* 14:3600.
