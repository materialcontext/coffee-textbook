+++
number = 4
title = "Grinding"
question = "What does a grinder make, how do we describe it, and which features of the grind matter for the cup?"
objectives = [
  "Explain why roasted coffee breaks by brittle fracture and estimate relative grinding energy from the comminution laws.",
  "Convert a sieve analysis into volume, surface-area and number distributions, and report d<sub>10</sub>, d<sub>50</sub>, d<sub>90</sub> and span.",
  "Calculate specific surface area and relative permeability from a distribution.",
  "Explain what does and does not change a grinder's output, including bean temperature and moisture.",
  "Explain fines, static charge and clumping, and their effects on brewing.",
  "Evaluate grinder comparisons, knowing that distribution measurements alone do not predict taste.",
]
prerequisites = ["Chapter 2A, Section 3 (diffusion time)", "Chapter 2B, Section 1 (Kozeny-Carman)"]
+++
A barista turns the grinder one step finer and the espresso slows from 28 to 40 seconds. A café owner buys a grinder that the manufacturer's charts show produces almost the same particle sizes as the old one, and the coffee tastes different. Both experiences say that "grind size" hides more than it shows.

This chapter opens the box. Section 1 explains how a roasted bean breaks. Sections 2 to 4 describe what a grinder produces and how to measure it. Sections 5 to 7 cover the factors that change a grind: temperature, fine particles and static electricity. Section 8 brings the pieces together into a method for comparing grinders.

## 1. How a roasted bean breaks

Roasting dries the bean's cell walls and fills its cells with gas, turning a tough, moist seed into a {{brittle}} solid: one that breaks without first bending or stretching. A grinder does not cut the bean into pieces of a chosen size. It squeezes and strikes the bean until cracks run through it, and the pieces fall out of the grinding zone once they are small enough to pass.

Why do brittle solids break at stresses far below the strength of their chemical bonds? Every real solid contains tiny flaws, and force concentrates at the tip of a flaw. In 1921 A. A. Griffith showed that a crack grows when the elastic energy released by its growth exceeds the energy needed to create the two new surfaces it opens. This {{Griffith criterion}} predicts that larger flaws need less force to grow. Large particles contain larger flaws and break easily; small particles contain only small flaws and resist. Each further reduction in size therefore costs more energy per gram.

Fracture produces a wide spread of sizes. Cracks running along cell walls release fragments a few cells across (cells in roasted coffee are about 20 to 50 &micro;m wide), and broken cell-wall material adds even smaller debris. Most grinders therefore produce a main population of particles near the burr gap and a second population of {{fines}}, particles smaller than about 100 &micro;m. A distribution with two peaks like this is called {{bimodal}}.

### How much energy does grinding take?

Reducing a material to smaller particles is called {{comminution}}. Three classical laws relate the energy E per unit mass to the feed size d<sub>1</sub> and the product size d<sub>2</sub>. Each assumes a different dominant cost.

~~~eq rittinger
formula = "E = C<sub>R</sub>(1/d<sub>2</sub> &minus; 1/d<sub>1</sub>)"
where = [["E", "energy per unit mass", "J/kg"], ["d<sub>1</sub>, d<sub>2</sub>", "feed and product particle sizes", "m"], ["C<sub>R</sub>", "Rittinger constant for the material and machine", "J m/kg"]]
~~~

{eq:rittinger}, Rittinger's law, assumes the energy goes into creating new surface. Since the surface area of a gram of particles is proportional to 1/d, it suits fine grinding.

~~~eq kick
formula = "E = C<sub>K</sub> ln(d<sub>1</sub>/d<sub>2</sub>)"
where = [["C<sub>K</sub>", "Kick constant", "J/kg"]]
~~~

{eq:kick}, Kick's law, assumes the energy goes into elastic deformation, so each halving of size costs the same energy. It suits coarse crushing.

~~~eq bond
formula = "E = C<sub>B</sub>(1/&radic;d<sub>2</sub> &minus; 1/&radic;d<sub>1</sub>)"
where = [["C<sub>B</sub>", "Bond constant", "J m<sup>1/2</sup>/kg"]]
~~~

{eq:bond}, Bond's law, lies between the other two and fits intermediate sizes. The constants depend on the material and the machine, but ratios of energies do not.

~~~example Espresso against filter grinding energy
Problem. How much more energy per gram does it take to grind a 6 mm bean to a median of 300 &micro;m (espresso) than to 800 &micro;m (filter)?

Plan. Take the ratio of each law at the two product sizes; the constants cancel.

Solution. Rittinger: (1/300 &minus; 1/6000)/(1/800 &minus; 1/6000) = 2.9. Bond: (300<sup>&minus;1/2</sup> &minus; 6000<sup>&minus;1/2</sup>)/(800<sup>&minus;1/2</sup> &minus; 6000<sup>&minus;1/2</sup>) = 2.0. Kick: ln(20)/ln(7.5) = 1.5.

Check. All three laws agree that espresso grinding costs more energy per gram, by a factor between 1.5 and 3. That energy ends up as heat in the burrs and grounds, and Section 5 shows that temperature changes the grind.
~~~

## 2. Burrs and grinders

{{Flat burrs}} are two parallel toothed discs, one rotating. Beans enter at the centre, are broken progressively as they move outward, and leave at the rim once they fit through the gap. {{Conical burrs}} are a toothed cone turning inside a toothed ring; beans are drawn downward through a tapering gap. Blade grinders strike the beans with a spinning blade and have no gap to set the size; the result depends on grinding time and is very broad.

The gap sets the upper end of the distribution. Tooth shape, burr alignment, rotation speed and the rate at which beans are fed shape the rest. Burrs wear with use, rounding their edges, which typically increases fines and heat.

## 3. Describing a distribution

A dose of ground coffee contains millions of particles, so it cannot be described by one size. The {{particle size distribution}} gives the share of material in each size range. The share can be counted in three ways, and the choice changes the picture dramatically.

- {{Number weighting}} counts every particle equally.
- {{Volume weighting}} counts each particle in proportion to its volume, which scales as d<sup>3</sup>. For particles of uniform density this is the same as weighting by mass, which is what sieving measures.
- Area weighting counts each particle in proportion to its surface area, which scales as d<sup>2</sup>.

Converting a volume distribution to a number distribution divides each size class by the volume of one particle in it:

~~~eq number
formula = "n<sub>i</sub> &prop; w<sub>i</sub> / d<sub>i</sub><sup>3</sup>"
where = [["n<sub>i</sub>", "number of particles in size class i (relative)", ""], ["w<sub>i</sub>", "mass or volume fraction in class i", "fraction"], ["d<sub>i</sub>", "representative diameter of class i", "m"]]
~~~

The effect is large. One gram of 100 &micro;m spheres at a density of 1.2 g/cm<sup>3</sup> contains about 1.6 million particles; one gram of 800 &micro;m spheres, about 3,100. A grind that is a few percent fines by mass is mostly fines by count. {fig:weighting} shows one grind both ways.

~~~figure
id = "weighting"
file = "ch04-weighting.gif"
alt = "Two panels for the same bimodal grind: by volume a large peak near 700 micrometres with a small fines shoulder; by number a single dominant peak in the fines."
caption = "One illustrative bimodal grind weighted by volume (left) and by number (right). Look at where the peak sits in each panel: the fines that barely register by volume dominate by number."
~~~

### Summary statistics

A volume distribution is summarised by {{percentile|percentiles}}. The {{d50}}, or volume median, is the size below which half the volume lies; d<sub>10</sub> and d<sub>90</sub> are defined in the same way for 10% and 90%. The width of the distribution is summarised by its {{span}}:

~~~eq span
formula = "span = (d<sub>90</sub> &minus; d<sub>10</sub>) / d<sub>50</sub>"
where = [["d<sub>10</sub>, d<sub>50</sub>, d<sub>90</sub>", "sizes below which 10%, 50% and 90% of the volume lies", "&micro;m"]]
~~~

A fines content is often reported as the volume fraction below 100 &micro;m. Always state the weighting: a median particle size without it is ambiguous by an order of magnitude.

### Specific surface area

The part of extraction that happens at particle surfaces depends on the {{specific surface area}}, the surface per unit mass. For spheres of diameter d and density &rho;, the area per unit mass is 6/(&rho;d). Summing over size classes:

~~~eq ssa
formula = "S<sub>A</sub> = &Sigma; 6 w<sub>i</sub> / (&rho; d<sub>i</sub>)"
where = [["S<sub>A</sub>", "specific surface area", "m<sup>2</sup>/kg"], ["&rho;", "particle density", "kg/m<sup>3</sup>"]]
~~~

Because d is in the denominator, small particles dominate the sum.

~~~example From a sieve table to a distribution
Problem. A 20.0 g sample is shaken through a stack of sieves. The masses retained, in grams, are 0.9 below 100 &micro;m, 1.1 at 100 to 200, 2.6 at 200 to 400, 6.8 at 400 to 600, 5.4 at 600 to 800 and 3.2 at 800 to 1,200. Find d<sub>10</sub>, d<sub>50</sub>, d<sub>90</sub>, the span and the specific surface area.

Plan. Convert masses to fractions, accumulate them, and interpolate linearly for the percentiles. Then apply {eq:ssa} using class midpoints and &rho; = 1,200 kg/m<sup>3</sup>.

Solution. Fractions: 4.5, 5.5, 13.0, 34.0, 27.0 and 16.0%. Cumulative: 4.5, 10.0, 23.0, 57.0, 84.0 and 100%. So d<sub>10</sub> = 200 &micro;m. d<sub>50</sub> = 400 + 200 &times; (0.50 &minus; 0.23)/0.34 = 559 &micro;m. d<sub>90</sub> = 800 + 400 &times; (0.90 &minus; 0.84)/0.16 = 950 &micro;m. Span = (950 &minus; 200)/559 = 1.34. Specific surface area: 14.6 m<sup>2</sup>/kg, of which the finest class, 4.5% of the mass, supplies 31%.

Check. By {eq:number}, the finest class makes up 94% of the particles by count, consistent with the figure.
~~~

~~~widget sieve
~~~

The calculator treats particles as spheres at the midpoint of each class. Real coffee particles are irregular and porous, so absolute surface areas are underestimates; comparisons between grinds are more reliable than single values.

~~~quiz
title = "Check your understanding: Sections 1 to 3"
[[q]]
type = "choice"
prompt = "A grind is 5% fines (below 100 &micro;m) by mass. By number, the fines are likely to be"
options = ["about 5% of the particles", "about half of the particles", "the large majority of the particles"]
answer = 2
explain = "Number scales as mass divided by d<sup>3</sup>, so small particles are vastly more numerous per gram."

[[q]]
type = "numeric"
prompt = "Volume percentiles are d<sub>10</sub> = 150, d<sub>50</sub> = 500 and d<sub>90</sub> = 900 &micro;m. What is the span?"
answer = 1.5
tolerance = 0.001
unit = ""
explain = "(900 &minus; 150)/500 = 1.5."
~~~

## 4. Measuring a distribution

Three methods are in common use, and they disagree in predictable ways.

{{Sieving}} shakes a sample through a stack of sieves with decreasing openings and weighs what each retains. It measures a mass distribution directly. It handles fines poorly, because fines cling to larger particles and to the mesh through static and moisture, and results depend on shaking time and vigour.

{{Laser diffraction}} disperses particles in air or liquid and shines a laser through them. The pattern of scattered light is converted to a volume distribution by assuming the particles are spheres. It is fast and repeatable, but irregular particles are treated as equivalent spheres, and the dispersion step may break up clumps or fail to.

{{Image analysis}} photographs particles spread on a contrasting surface and measures each one. A phone camera and free software resolve particles down to about 50 to 100 &micro;m. The result is a number or area distribution that can be converted to volume by assuming a shape.

Different methods give different numbers for the same grind. Compare grinders only by one method, with fixed settings and sample preparation.

## 5. What changes the distribution

With measurement settled, we can ask which variables actually change a grinder's output. Uman and colleagues (2016) ground beans of several origins and roast conditions at one grinder setting, at temperatures from room temperature down to &minus;196 &deg;C [1]. At a fixed setting, origin and processing made little difference. Colder beans ground finer and with a narrower distribution. Cold makes the bean more brittle, so cracks run cleanly rather than blunting.

Temperature also changes during use: burrs and grounds warm over a busy session, and the first doses from a cold grinder differ from later ones.

Gagné analysed about 300 distributions from 24 espresso grinders [2]. Conical burrs were on average less uniform than flat burrs, though the two groups overlapped heavily. Burr diameter showed no clear trend. Rotation speed, feed rate and the measuring instrument all changed the data.

~~~misconception
"Turning an espresso grinder finer moves the whole distribution to smaller sizes." In Gagné's survey, adjusting espresso grinders finer mostly increased the fines fraction and left the main peak nearly in place [2]. That is why a small adjustment changes shot time so much: it changes the fines, and the fines control permeability.
~~~

## 6. Fines

Fines matter out of proportion to their mass, for two reasons that pull in different directions.

They extract fast. Diffusion time scales as d<sup>2</sup> (Chapter 2A), so a 50 &micro;m particle empties about 100 times faster than a 500 &micro;m one. In a short brew, fines release nearly all their solubles.

They control flow. Permeability depends on an effective particle size set largely by the fines (Chapter 2B). Fines slow drawdown in a pour-over, lengthen espresso shots, and can migrate through the bed to collect at the filter.

Espresso experiments that added fines at fixed dose and beverage mass found that the fines mainly lengthened shot time; extraction yield followed one common curve against time, and added fines did not lower sensory scores [3]. In filter brewing, fines raise extraction in a fixed contact time and slow the flow, so their net effect depends on the method and recipe.

~~~example Permeability and grind
Problem. A grind adjustment halves the effective particle diameter at constant porosity. What happens to espresso flow at fixed pressure, and what would restore it?

Plan. Apply the d<sup>2</sup> dependence of the Kozeny-Carman relation (Equation 2B.2) and Darcy's law (Equation 2B.1).

Solution. Permeability falls to (1/2)<sup>2</sup> = 1/4, so the flow falls to a quarter. Restoring it would need four times the pressure, which is unavailable and would in any case be offset by compaction (Chapter 2B, Section 5), or a thinner puck from a smaller dose.

Check. A quarter of the flow means roughly four times the shot time for the same beverage mass, matching the large time changes baristas see after small adjustments.
~~~

## 7. Static charge and clumping

Grinding charges coffee particles by friction and fracture, a process called {{triboelectric charging}}. Méndez Harper, Hendon and colleagues (2024) measured the charge on grounds from many coffees and conditions [4]. Charge fell as the internal moisture of the beans rose, so lighter roasts, which keep more water, charged less. Coarser grinding also charged less, and the charge could even reverse sign with moisture content. Adding a small amount of water to the beans before grinding suppressed charging. That reduced clumping, and in espresso it gave denser pucks, slower flow at the same grind, higher extraction and more reproducible shots.

Spraying beans with a drop of water before grinding a single dose applies this result directly. Charge also explains why grounds spray from the chute, cling to plastic, and resist sieving.

~~~evidence
The charging study was rigorous on the physics of charging and on its effect on espresso extraction. It did not establish a general improvement in taste, and its espresso tests used a limited range of coffees and grinders. Water added before grinding also wets the burrs; effects on burr corrosion and hygiene were not studied.
~~~

## 8. Choosing and comparing grinders

In Gagné's survey, the grinders rated best in blind tastings sat in the middle of the range of distribution uniformity, not at the extreme, and one poorly rated grinder had a distribution resembling those of highly rated ones [2]. Distribution measurements are needed to describe a grinder but are not enough to predict how its coffee tastes. Other plausible contributors are particle shape, clumping, heating during grinding, and old grounds retained in the grinder.

~~~misconception
"The grinder with the narrowest distribution makes the best coffee." Neither the survey above nor any controlled study has shown a simple relation between distribution width and taste. Treat distribution data as a description, and judge taste by blind tests.
~~~

A sound comparison has four parts:

1. Match the brew outcome, not the dial setting: adjust each grinder to the same extraction yield at the same recipe, or to the same shot time and beverage mass.
2. Measure each distribution by one method, recording rotation speed, feed and temperature.
3. Taste blind, first with triangle tests to establish whether there is a difference, then with descriptive or preference tests to characterise it (Chapter 1).
4. Measure variability: the standard deviation of extraction yield across repeated brews often separates grinders better than the mean does.

## Looking ahead

The particles leaving the grinder are full of gas and aroma that begin to escape as soon as the bean is roasted, and faster still once it is ground. Chapter 5 asks what changes in roasted coffee after it leaves the roaster, how fast, and what slows it down.

## Key equations

~~~equations
[[eq]]
name = "Rittinger"
formula = "E = C<sub>R</sub>(1/d<sub>2</sub> &minus; 1/d<sub>1</sub>)"
note = "Equation 4.1."
[[eq]]
name = "Kick"
formula = "E = C<sub>K</sub> ln(d<sub>1</sub>/d<sub>2</sub>)"
note = "Equation 4.2."
[[eq]]
name = "Bond"
formula = "E = C<sub>B</sub>(d<sub>2</sub><sup>&minus;1/2</sup> &minus; d<sub>1</sub><sup>&minus;1/2</sup>)"
note = "Equation 4.3."
[[eq]]
name = "Number from volume"
formula = "n<sub>i</sub> &prop; w<sub>i</sub> / d<sub>i</sub><sup>3</sup>"
note = "Equation 4.4."
[[eq]]
name = "Span"
formula = "(d<sub>90</sub> &minus; d<sub>10</sub>) / d<sub>50</sub>"
note = "Equation 4.5. Volume-weighted percentiles."
[[eq]]
name = "Specific surface area, spheres"
formula = "S<sub>A</sub> = &Sigma; 6w<sub>i</sub> / (&rho; d<sub>i</sub>)"
note = "Equation 4.6."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "choice"
prompt = "In Uman et al., grinding colder beans at a fixed setting gave"
options = ["a finer, narrower distribution", "a coarser distribution", "no change"]
answer = 0
explain = "Cold beans fracture more cleanly, giving a finer and narrower grind."

[[q]]
type = "choice"
prompt = "Adding a few drops of water to beans before grinding mainly"
options = ["reduces static charge and clumping", "softens the beans so they grind coarser", "extracts some solubles before brewing"]
answer = 0
explain = "Water suppresses triboelectric charging, reducing clumping and spray."

[[q]]
type = "choice"
prompt = "Why do sieves usually under-report fines compared with laser diffraction in liquid?"
options = ["Fines pass through every sieve", "Dry fines cling to larger particles and to the mesh", "Laser diffraction counts air bubbles as fines"]
answer = 1
explain = "Static and cohesion hold dry fines on coarser sieves; liquid dispersion separates them."
~~~

## Labs

~~~lab
id = "L4.1"
title = "Sieve analysis of two grinders"
aim = "Compare the distributions of two grinders set to the same volume median."
safety = ["Sieves have sharp edges; mechanical shakers can trap fingers."]
materials = ["Test sieves (for example 100, 200, 400, 600 and 800 &micro;m)", "Balance reading to 0.01 g", "Two grinders", "Antistatic brush"]
method = [
  "Grind 20 g from each grinder at settings you expect to give similar medians.",
  "Sieve each sample for a fixed time (5 minutes) with consistent motion, and weigh each fraction.",
  "Repeat each grinder twice.",
]
columns = ["Grinder", "Run", "&lt;100", "100 to 200", "200 to 400", "400 to 600", "600 to 800", "&gt;800"]
rows = 6
analysis = [
  "Enter each run in the sieve calculator. Report d<sub>10</sub>, d<sub>50</sub>, d<sub>90</sub>, span and fines fraction.",
  "Are the differences between grinders larger than the differences between repeats?",
]
~~~

~~~lab
id = "L4.2"
title = "Image analysis with a phone"
aim = "Measure a number distribution from a photograph and compare it with sieving."
materials = ["Phone camera", "White paper and a ruler for scale", "Free image analysis software (for example ImageJ)"]
method = [
  "Spread 0.1 g of grounds thinly on paper so that particles do not touch, and photograph them with the ruler in the frame.",
  "Convert the image to black and white, separate touching particles, and measure the diameter of the circle with the same area as each particle.",
]
columns = ["Size class (&micro;m)", "Count", "Number %", "Volume % (computed)"]
rows = 8
analysis = [
  "Convert number fractions to volume fractions using d<sup>3</sup> weighting and compare with L4.1.",
  "What is the smallest particle you can reliably detect, and how does that bias the comparison?",
]
~~~

~~~lab
id = "L4.3"
title = "Cold grinding"
aim = "Test the effect of bean temperature on the distribution."
safety = ["Handle frozen beans with gloves; grind straight from a sealed container to avoid condensation."]
materials = ["Sealed container of beans frozen overnight", "Room-temperature beans of the same lot", "Sieves or image analysis"]
method = ["Grind 20 g of each at the same setting, the frozen beans straight from the freezer.", "Measure both distributions."]
columns = ["Temperature", "d<sub>10</sub>", "d<sub>50</sub>", "d<sub>90</sub>", "Span", "Fines %"]
rows = 2
analysis = ["Compare with Uman et al. Is the frozen grind finer and narrower?"]
~~~

~~~lab
id = "L4.4"
title = "Water before grinding"
aim = "Measure the effect of a drop of water on retention, clumping and extraction."
materials = ["Single-dose grinder", "Spray bottle or pipette", "Scale", "Refractometer"]
method = [
  "Grind ten 18 g doses, alternating five dry and five with 0.1 mL of water stirred into the beans.",
  "Weigh the grounds that come out to find retention; brew each by one fixed recipe and measure extraction yield.",
]
columns = ["Dose", "Water?", "Grounds out (g)", "Retention (g)", "EY (%)", "Clumps observed"]
rows = 10
analysis = ["Compare the mean and standard deviation of extraction yield and retention between conditions."]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "Why do roasted beans fracture rather than deform under burr loading?"
solution = "Roasting dries and embrittles the cell walls and creates a porous, gas-filled structure full of flaws. Force concentrates at crack tips and cracks grow before the material can deform."

[[p]]
tier = "A"
section = "3"
prompt = "State what d<sub>50</sub> means for a volume-weighted distribution, and why it differs from the median size counted by number."
solution = "Half the volume (or mass) is in particles smaller than d<sub>50</sub>. The number median is far smaller, because fines vastly outnumber larger particles while contributing little volume."

[[p]]
tier = "A"
section = "3"
prompt = "How many 50 &micro;m spheres of density 1.2 g/cm<sup>3</sup> are there in one gram?"
solution = "Volume of one particle = (&pi;/6)(5 &times; 10<sup>&minus;3</sup> cm)<sup>3</sup> = 6.54 &times; 10<sup>&minus;8</sup> cm<sup>3</sup>; mass = 7.85 &times; 10<sup>&minus;8</sup> g; so about 1.3 &times; 10<sup>7</sup> particles per gram."

[[p]]
tier = "B"
section = "3"
prompt = "A sieve analysis gives cumulative volume fractions of 0.08 at 200 &micro;m, 0.30 at 400, 0.62 at 600 and 0.91 at 800. Find d<sub>50</sub> and d<sub>90</sub> by linear interpolation."
solution = "d<sub>50</sub> = 400 + 200 &times; (0.50 &minus; 0.30)/0.32 = 525 &micro;m. d<sub>90</sub> = 600 + 200 &times; (0.90 &minus; 0.62)/0.29 = 793 &micro;m."

[[p]]
tier = "B"
section = "3"
prompt = "Treating particles as spheres of density 1,200 kg/m<sup>3</sup>, compute the specific surface area of a grind that is 10% by mass at 50 &micro;m and 90% at 600 &micro;m. What share of the area do the fines supply?"
solution = "Fines: 0.10 &times; 6/(1200 &times; 5 &times; 10<sup>&minus;5</sup>) = 10.0 m<sup>2</sup>/kg. Main population: 0.90 &times; 6/(1200 &times; 6 &times; 10<sup>&minus;4</sup>) = 7.5 m<sup>2</sup>/kg. Total 17.5 m<sup>2</sup>/kg, of which the fines supply 57%."

[[p]]
tier = "B"
section = "1"
prompt = "By Rittinger's law, how much more energy per gram does grinding from 6 mm to 200 &micro;m take than grinding from 6 mm to 600 &micro;m?"
solution = "(1/200 &minus; 1/6000)/(1/600 &minus; 1/6000) = 0.004833/0.0015 = 3.2 times."

[[p]]
tier = "B"
section = "6"
prompt = "A grinder change reduces the effective, permeability-controlling diameter by 15% at constant porosity. By how much does espresso flow change at fixed pressure and dose?"
solution = "&kappa; scales as d<sup>2</sup>: 0.85<sup>2</sup> = 0.72. The flow falls by 28%."

[[p]]
tier = "B"
section = "6"
prompt = "If diffusion time scales as d<sup>2</sup>, how much faster does a 70 &micro;m particle empty than a 700 &micro;m particle?"
solution = "(700/70)<sup>2</sup> = 100 times faster."

[[p]]
tier = "B"
section = "4"
prompt = "Why do sieve analyses typically under-report fines compared with laser diffraction in liquid?"
solution = "Dry fines clump and cling to larger particles and to the mesh through static and cohesion, so they are retained on coarser sieves. Dispersion in liquid separates them, and diffraction detects particles down to a few micrometres."

[[p]]
tier = "C"
section = "8"
prompt = "Two grinders have nearly identical volume distributions, yet blind tasters consistently prefer one. List four mechanisms that could explain this and a measurement for each."
solution = "Particle shape (image analysis of aspect ratio); clumping and static (charge measurement, or sieving dry against wet); heating during grinding (temperature of the grounds immediately after grinding); retention and mixing of stale grounds (weigh retention, or grind a distinctive coffee and count carry-over). The distributions may also differ below the instrument's resolution."

[[p]]
tier = "C"
section = "7"
prompt = "Using charge and moisture, explain why light roasts tend to clump and spray less than dark roasts when ground."
solution = "Light roasts keep more internal moisture, and charging decreases as moisture rises. Light-roast grounds therefore carry less charge, so they attract and repel each other and surrounding surfaces less, and clump and spray less."

[[p]]
tier = "C"
section = "8"
prompt = "Design an experiment to decide whether a new burr set improves espresso reproducibility. Include the outcome measure and your reasoning about sample size."
solution = "The main outcome is the standard deviation of extraction yield at fixed dose and beverage mass, with grind matched by taste or by shot time. Pull shots alternately with old and new burrs (or two matched grinders) in random order, 25 or more per condition, and compare variances with an F-test or Levene's test. Detecting a twofold difference in variance with good power needs roughly 25 to 30 shots per condition. Record shot times and distributions to interpret the result."

[[p]]
tier = "C"
section = "1"
prompt = "A grinder maker claims its burrs produce zero fines. Explain why this is implausible for a fracture process, and what claim would be testable."
solution = "Brittle fracture of a cellular solid releases cell-wall debris and fragments near cell size regardless of the gap, so some fines are unavoidable. A testable claim states a fines fraction (volume below 100 &micro;m) measured by a named method and sample preparation, at a stated median size, compared with named grinders."
~~~

## References

1. Uman E. et al. (2016), The effect of bean origin and temperature on grinding roasted coffee, *Scientific Reports* 6:24483.
2. Gagné J. (2023), What I learned from analyzing 300 particle size distributions for 24 espresso grinders, *Coffee ad Astra*.
3. The role of fines in espresso extraction dynamics (2024), *Scientific Reports*.
4. Méndez Harper J. et al. (2024), Moisture-controlled triboelectrification during coffee grinding, *Matter* 7:266.
