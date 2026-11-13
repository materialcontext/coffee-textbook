+++
number = 2
label = "2A"
title = "Extraction I: immersion, equilibrium and kinetics"
question = "How do solubles leave the grounds, how far can they go, and how fast?"
objectives = [
  "Write mass balances for immersion and percolation brews and calculate extraction yield from each.",
  "Use the equilibrium desorption model to predict strength and yield from brew ratio, and explain why immersion yield at equilibrium does not depend on the ratio.",
  "Estimate how long solubles take to leave a particle, and explain why small and large particles extract on different time scales.",
  "Fit a two-population kinetic model to measurements of strength over time.",
  "Place a brew on the brewing control chart and predict how grind, ratio, time and temperature move it.",
  "Relate titratable acidity and sourness to position on the chart.",
]
prerequisites = ["Chapter 0, Section 1 (TDS, extraction yield, uncertainty)", "Appendix A, Section A.2 (concentration units)", "Appendix C, Sections C.1 and C.4 (first-order kinetics, diffusion)"]
+++
Leave a French press for eight minutes instead of four, and many baristas will say the coffee is over-extracted. Measure it, and the extraction yield has barely moved. Steep a weak brew for an hour, and its yield still stops near 21%, never approaching the 30% of the bean that is soluble. Something limits how far immersion extraction can go, and something sets how fast it gets there.

This chapter builds the two ideas that explain both observations: equilibrium, which sets the end point, and diffusion, which sets the pace. We use immersion brewing, where all the water and all the coffee sit together, because it is the simplest geometry. Chapter 2B applies the same ideas to water flowing through a bed.

## 1. Strength, extraction and the mass balance

Two numbers describe a brew. The first is its strength, the total dissolved solids S defined in Chapter 0: the mass of dissolved coffee material per unit mass of brew, written as a fraction (0.0135) or a percentage (1.35%). Strength is what a taster perceives as intensity. The second is the extraction yield E: the fraction of the dry coffee that has dissolved. Extraction yield describes how far the process has gone.

The two are linked by a {{mass balance}}, an account of where every gram ends up. We need four masses: the dose of dry coffee m<sub>c</sub>, the brew water m<sub>w</sub>, the beverage collected m<sub>b</sub>, and the dissolved solids m<sub>s</sub>. The {{brew ratio}} is the water per gram of coffee:

~~~eq ratio
formula = "r = m<sub>w</sub> / m<sub>c</sub>"
where = [["r", "brew ratio", "g/g"], ["m<sub>w</sub>", "mass of brew water", "g"], ["m<sub>c</sub>", "mass of dry coffee", "g"]]
~~~

### Percolation

In a {{percolation}} brew (pour-over, batch filter, espresso), fresh water passes through a bed of grounds and leaves as beverage. If we count only the solids in the cup, then m<sub>s</sub> = S m<sub>b</sub>, and

~~~eq perc
formula = "E = m<sub>s</sub> / m<sub>c</sub> = S m<sub>b</sub> / m<sub>c</sub>"
where = [["E", "extraction yield", "fraction"], ["S", "strength (TDS) of the beverage", "fraction"], ["m<sub>b</sub>", "mass of beverage", "g"]]
~~~

The liquid left in the bed at the end is ignored. In percolation that liquid is the last water to pass through, which is weaker than the cup, so ignoring it causes only a small error.

### Immersion

In an {{immersion}} brew (French press, cupping bowl), all the water and all the coffee sit together. At the end, the liquid trapped in the grounds has the same strength as the liquid poured off, so it cannot be ignored. We derive the extraction yield in three steps.

1. All the dissolved solids are in the liquid, and the liquid's mass is the water plus the solids: m<sub>w</sub> + m<sub>s</sub>.
2. The strength is the fraction of that liquid that is solids: S = m<sub>s</sub>/(m<sub>w</sub> + m<sub>s</sub>).
3. Multiplying out gives S m<sub>w</sub> + S m<sub>s</sub> = m<sub>s</sub>, so m<sub>s</sub>(1 &minus; S) = S m<sub>w</sub> and m<sub>s</sub> = S m<sub>w</sub>/(1 &minus; S).

Dividing by the dose and using {eq:ratio}:

~~~eq imm
formula = "E = r S / (1 &minus; S)"
where = [["E", "extraction yield", "fraction"], ["r", "brew ratio", "g/g"], ["S", "strength (TDS) of the liquid", "fraction"]]
~~~

~~~example Two formulas, two answers
Problem. 15.0 g of coffee steeps in 250 g of water. The liquid reads 1.35% TDS, and you pour off 215 g. Find the extraction yield.

Plan. The brew is an immersion brew, so use {eq:imm}. Compare with the result of wrongly using the percolation formula.

Solution. The brew ratio is r = 250/15.0 = 16.7. Then E = 16.7 &times; 0.0135/(1 &minus; 0.0135) = 0.228, or 22.8%. The percolation formula would give 0.0135 &times; 215/15.0 = 19.4%.

Check. The difference comes from the 35 g of liquid left in the grounds, at full strength, whose solids the percolation formula discards. Using the wrong formula understates immersion extraction by more than three percentage points.
~~~

A related quantity is the {{retained liquid}}, r<sub>ret</sub>, the mass of liquid held per gram of grounds after brewing. For filter grinds it is about 2 g/g, which is why a 300 g pour-over from 18 to 20 g of coffee yields only about 260 to 270 g in the cup.

~~~widget recipe
ratio = 16
water = 300
~~~

## 2. How far can extraction go? The equilibrium model

Roughly 30% of a roasted bean can dissolve in hot water, yet immersion brews stop near 21%. Liang, Chan and Ristenpart (2021) explained the gap with a {{desorption}} model [1]. Desorption is the release of a substance from a solid surface into a liquid; its reverse, the uptake of dissolved material back onto the solid, is {{adsorption}}.

The model treats all soluble material as a single substance, an approximation called {{lumped model|lumping}}. Let M be the mass of soluble material in the dose, which is E<sub>max</sub> m<sub>c</sub>, where the {{maximum extractable fraction}} E<sub>max</sub> &asymp; 0.30. At some moment a mass x of it has dissolved and M &minus; x remains in the grounds.

- Material leaves the grounds at a rate proportional to the amount still there: k<sub>D</sub>(M &minus; x).
- Material returns to the grounds at a rate proportional to the amount in solution: k<sub>A</sub> x.

At {{equilibrium}} the two rates are equal, so k<sub>D</sub>(M &minus; x) = k<sub>A</sub> x. Solving for the fraction dissolved:

~~~eq keq
formula = "x / M = K = k<sub>D</sub> / (k<sub>D</sub> + k<sub>A</sub>)"
where = [["K", "extraction equilibrium constant: the fraction of soluble material in solution at equilibrium", "dimensionless"], ["k<sub>D</sub>, k<sub>A</sub>", "rate constants for desorption and adsorption", "1/min"]]
~~~

We call K the {{extraction equilibrium constant}}. The extraction yield at equilibrium is the dissolved mass divided by the dose, x/m<sub>c</sub> = K M/m<sub>c</sub>:

~~~eq eqy
formula = "E<sub>eq</sub> = K E<sub>max</sub>"
where = [["E<sub>eq</sub>", "extraction yield at equilibrium", "fraction"], ["E<sub>max</sub>", "maximum extractable fraction of the dry coffee", "fraction"]]
~~~

The brew ratio does not appear. To find the strength, set {eq:eqy} equal to the immersion mass balance, {eq:imm}: K E<sub>max</sub> = r S/(1 &minus; S). Writing E for K E<sub>max</sub>, multiplying out gives E &minus; E S = r S, so S(r + E) = E, and

~~~eq eqs
formula = "S<sub>eq</sub> = K E<sub>max</sub> / (r + K E<sub>max</sub>)"
where = [["S<sub>eq</sub>", "strength at equilibrium", "fraction"]]
~~~

Because K E<sub>max</sub> &asymp; 0.2 is small compared with any brew ratio, the strength falls roughly as 1/r.

Fitted to one-litre immersion brews over a wide range of ratios, the model gave K = 0.717 &plusmn; 0.007, so K E<sub>max</sub> = 0.215. Measured equilibrium yields averaged 20.7 &plusmn; 1.1% against the predicted 21.5%, and strength was almost exactly proportional to 1/r. In traditional cupping, K was 0.79 for regular and 0.73 for decaffeinated coffee. K did not change between 80 and 99 &deg;C, nor with grind size or roast level over the ranges tested [1]. {fig:equilibrium} shows the predictions.

~~~figure
id = "equilibrium"
file = "ch02a-equilibrium.gif"
alt = "Graph of equilibrium TDS falling with brew ratio while extraction yield stays constant at about 21.5 percent."
caption = "Immersion at equilibrium predicted by the desorption model (K = 0.717, E<sub>max</sub> = 0.30). Notice that the yield line is flat: the ratio changes strength, not yield."
~~~

~~~widget equilibrium
~~~

The model has three practical consequences. In immersion, the ratio sets the strength, and the coffee sets the yield. Waiting longer cannot raise yield once equilibrium is reached. And the familiar target of 18 to 22% extraction is, for immersion at equilibrium, mainly a property of the coffee rather than of the recipe.

~~~misconception
"Steeping a French press longer over-extracts it." If the brew has reached equilibrium, its extraction yield cannot rise further, so a longer steep cannot raise yield. The flavour can still change, through cooling, loss of aroma to the air, oxidation and slow reactions in solution, and a coarse grind may not yet be at equilibrium after four minutes. Measure strength at both times before blaming extraction (Problem 11).
~~~

~~~evidence
The equilibrium model was fitted by one laboratory to one coffee in large batches and to cupping. It treats all solubles as one substance, which cannot be strictly true: caffeine, acids and melanoidins differ in how strongly they bind to the grounds and how fast they move. Its predictions for total dissolved solids are well supported. Its usefulness for individual flavour compounds is untested. E<sub>max</sub> = 0.30 was assumed rather than measured for each coffee.
~~~

~~~quiz
title = "Check your understanding: Sections 1 and 2"
[[q]]
type = "numeric"
prompt = "A French press uses 30 g of coffee in 500 g of water, and the liquid reads 1.30% TDS. What is the extraction yield in percent?"
answer = 21.95
tolerance = 0.1
unit = "%"
explain = "r = 16.67; E = 16.67 &times; 0.0130/0.987 = 0.2195, or 21.95%."

[[q]]
type = "choice"
prompt = "According to the equilibrium model, doubling the brew ratio of an immersion brew at equilibrium will"
options = ["roughly halve the strength and leave the yield unchanged", "halve both strength and yield", "leave strength unchanged and double yield"]
answer = 0
explain = "E<sub>eq</sub> = K E<sub>max</sub> does not depend on r, while S<sub>eq</sub> falls roughly as 1/r."
~~~

## 3. How fast? Diffusion out of a particle

Equilibrium tells us where extraction ends. To know how long it takes, we have to follow solubles out of a particle. Water reaches broken cells on the surface of a coffee particle almost immediately, and their contents dissolve within seconds. Solubles deeper inside must travel through the particle to reach the surface. They travel by {{diffusion}}, the net movement of molecules from regions of high concentration to regions of low concentration, caused by their random thermal motion.

How quickly a substance diffuses is described by its {{diffusivity}} D, in m<sup>2</sup>/s. Appendix C.4 shows that in a time t a molecule typically wanders a distance of order &radic;(Dt). Turning that around, the time to diffuse out of a particle of radius a is of order a<sup>2</sup>/D. For a sphere, solving the diffusion equation exactly (see the box below) gives the precise {{time constant}}:

~~~eq tau
formula = "&tau; = a<sup>2</sup> / (&pi;<sup>2</sup> D)"
where = [["&tau;", "time constant for solute leaving a sphere", "s"], ["a", "particle radius", "m"], ["D", "diffusivity of the solute inside the particle", "m<sup>2</sup>/s"]]
~~~

After the first few seconds, the solute remaining in the particle falls approximately as e<sup>&minus;t/&tau;</sup>: after one time constant 37% remains, after three time constants 5%.

The most important feature of {eq:tau} is the square. Halving the particle diameter quarters the extraction time. A fine particle of 100 &micro;m and a coarse particle of 800 &micro;m differ in time constant by a factor of 8<sup>2</sup> = 64. {fig:diffusion} shows release curves for three sizes.

~~~figure
id = "diffusion"
file = "ch02a-diffusion.gif"
alt = "Three curves of fraction released against time for particles of 100, 400 and 800 micrometres; the smallest reaches nearly complete release in under a minute while the largest is far from complete at ten minutes."
caption = "Release of solute from spheres of three diameters (exact solution, illustrative D = 2 &times; 10<sup>&minus;10</sup> m<sup>2</sup>/s). Compare the times at which each curve reaches 90%: they scale with the square of the diameter."
~~~

~~~deeper The exact solution for a sphere
Fick's second law in spherical coordinates, &part;c/&part;t = D (1/r<sup>2</sup>) &part;/&part;r(r<sup>2</sup> &part;c/&part;r), describes how concentration c changes inside the particle. With a uniform starting concentration and the surface held at zero, separating variables (Appendix C.4) gives the fraction released:

F(t) = 1 &minus; (6/&pi;<sup>2</sup>) &Sigma;<sub>n=1</sub><sup>&infin;</sup> (1/n<sup>2</sup>) exp(&minus;n<sup>2</sup>&pi;<sup>2</sup>Dt/a<sup>2</sup>).

The n = 1 term decays most slowly, with time constant a<sup>2</sup>/(&pi;<sup>2</sup>D), which is {eq:tau}. The higher terms die away within seconds, which is why a single exponential describes the later stage of release.
~~~

~~~widget diffusion
~~~

~~~example Fines against boulders
Problem. Take D = 2 &times; 10<sup>&minus;10</sup> m<sup>2</sup>/s. Find the time constants of a 600 &micro;m particle, a 100 &micro;m fine and a 1,000 &micro;m boulder, and the fraction of solute the boulder still holds after a three-minute brew.

Plan. Apply {eq:tau} with a = d/2, then use the d<sup>2</sup> scaling for the other sizes.

Solution. For d = 600 &micro;m, a = 3 &times; 10<sup>&minus;4</sup> m and &tau; = (3 &times; 10<sup>&minus;4</sup>)<sup>2</sup>/(&pi;<sup>2</sup> &times; 2 &times; 10<sup>&minus;10</sup>) = 45.6 s. For the fine, &tau; = 45.6 &times; (100/600)<sup>2</sup> = 1.27 s. For the boulder, &tau; = 45.6 &times; (1000/600)<sup>2</sup> = 127 s. Using the first term of the exact solution, after 180 s the boulder retains (6/&pi;<sup>2</sup>) e<sup>&minus;180/127</sup> &asymp; 0.15, or 15%.

Check. The fine is effectively empty after a few seconds, so fines set the early strength; boulders set how much a longer contact time adds. Real diffusivities in coffee are poorly known and differ between compounds, so trust the ratios more than the absolute times.
~~~

## 4. Two populations of solubles

A single particle size cannot describe a real grind, which contains fines and boulders together, and each particle has broken surface cells as well as intact cells inside. Moroney and colleagues built models on this picture and checked them against measured extraction curves for filter coffee [2][3]. A simple version with two populations, one fast and one slow, describes the strength S(t) of an immersion brew over time:

~~~eq twopop
formula = "S(t) = S<sub>&infin;</sub> [1 &minus; &phi; e<sup>&minus;t/&tau;<sub>1</sub></sup> &minus; (1 &minus; &phi;) e<sup>&minus;t/&tau;<sub>2</sub></sup>]"
where = [["S<sub>&infin;</sub>", "final strength", "fraction"], ["&phi;", "fraction of extractable material in the fast population", "fraction"], ["&tau;<sub>1</sub>, &tau;<sub>2</sub>", "time constants of the fast and slow populations, &tau;<sub>1</sub> much smaller than &tau;<sub>2</sub>", "min"]]
~~~

### Fitting the slow time constant

At late times the fast term has died away, so 1 &minus; S/S<sub>&infin;</sub> &asymp; (1 &minus; &phi;) e<sup>&minus;t/&tau;<sub>2</sub></sup>. Taking natural logarithms,

ln(1 &minus; S/S<sub>&infin;</sub>) &asymp; ln(1 &minus; &phi;) &minus; t/&tau;<sub>2</sub>,

which is a straight line in t. Its slope is &minus;1/&tau;<sub>2</sub> and its intercept is ln(1 &minus; &phi;). Early points lie above the line, because the fast population has not yet finished dissolving; the gap closes after a few multiples of &tau;<sub>1</sub>. With only four or five samples you can usually estimate &tau;<sub>2</sub> and &phi;, but not &tau;<sub>1</sub>, which is often shorter than the time to your first sample.

~~~example Fitting immersion data
Problem. An immersion brew reads 0.88, 1.06, 1.20, 1.31 and 1.35% TDS at 1, 2, 4, 8 and 16 minutes. Estimate &tau;<sub>2</sub> and &phi;.

Plan. Take S<sub>&infin;</sub> slightly above the last reading, compute ln(1 &minus; S/S<sub>&infin;</sub>) for each point, and fit a straight line to the late points.

Solution. With S<sub>&infin;</sub> = 1.36%, the values of ln(1 &minus; S/S<sub>&infin;</sub>) are &minus;1.04, &minus;1.51, &minus;2.14, &minus;3.30 and &minus;4.91. A least-squares line through the 4, 8 and 16 minute points has slope &minus;0.227 per minute and intercept &minus;1.34. So &tau;<sub>2</sub> = 1/0.227 = 4.4 min and 1 &minus; &phi; = e<sup>&minus;1.34</sup> = 0.26: about three quarters of the extractable solids came out in the fast stage.

Check. The 1 and 2 minute points (&minus;1.04 and &minus;1.51) lie above the line's values at those times (&minus;1.57 and &minus;1.79), as expected while the fast population is still dissolving.

Your turn. Repeat the fit with S<sub>&infin;</sub> = 1.38%. Problem 10 asks how sensitive &tau;<sub>2</sub> is to this choice.
~~~

## 5. Temperature acts on rate

Hot water extracts faster, because diffusivity rises and viscosity falls with temperature. A separate question is whether temperature changes what is extracted when a brew reaches the same strength and yield.

Two results from UC Davis address it. In the equilibrium study, K did not change between 80 and 99 &deg;C [1]. In a sensory study, drip coffee was brewed at 87, 90 and 93 &deg;C to the same strength and extraction yield, and a trained panel found little difference between them [4]. Within the ordinary hot-brewing range, temperature therefore behaves mainly as a rate variable: it changes how quickly a brew reaches a given strength and yield, and brews that arrive at the same point taste similar.

Cold brew is the exception that marks the edge of that range. Steeped for many hours at or below room temperature, it reaches a pH similar to hot brew (4.85 to 5.13 in one study) but lower titratable acidity, and hot brew showed greater antioxidant activity [5]. Low temperatures do change composition.

~~~evidence
The temperature result comes from drip brewing between 87 and 93 &deg;C. It has not been established for espresso, for very light roasts brewed cool, or for individual aroma compounds, whose loss to the air rises with temperature. Treat "temperature is a rate variable" as a well-supported rule for hot filter brewing, not a general law.
~~~

## 6. The brewing control chart

With strength and yield defined, every brew can be drawn as a point on one graph. In the 1950s Ernest Lockhart plotted strength on the vertical axis against extraction yield on the horizontal axis. This {{brewing control chart}} turns a brew recipe into geometry. Rearranging {eq:perc} gives the line on which every percolation brew of a given beverage-to-dose ratio lies:

~~~eq chart
formula = "S = E (m<sub>c</sub> / m<sub>b</sub>)"
where = [["m<sub>b</sub>/m<sub>c</sub>", "beverage-to-dose ratio", "g/g"]]
~~~

Each ratio is a straight line through the origin with slope m<sub>c</sub>/m<sub>b</sub>. At a fixed ratio, changes in grind, time or agitation move a brew along its line; changing the ratio moves it to another line. {fig:chart} shows lines for ratios from 14 to 18.

~~~figure
id = "chart"
file = "ch02a-chart.gif"
alt = "Brewing chart with TDS on the vertical axis and extraction yield on the horizontal axis, showing straight lines for beverage-to-dose ratios from 14 to 18."
caption = "The brewing control chart for percolation, with lines of constant beverage-to-dose ratio. Lockhart's nine labelled zones are deliberately left out (see text)."
~~~

Lockhart divided the chart into nine rectangular zones with sensory labels such as "bitter" and "under-developed", and for decades the zones were taught as fact. In 2023 a UC Davis group re-mapped the chart with a trained descriptive panel and a large consumer study [6]. The panel found attributes changing gradually across the chart rather than switching at zone boundaries. Bitterness was highest at high strength and high extraction, agreeing with Lockhart. Consumers split into two groups that preferred different regions of the chart. The authors cautioned that the consumers were young and North American [7]. The calculator below draws the chart without zones and shades your ratio line.

~~~widget extraction
mode = "filter"
method = "imm"
~~~

### Where sourness sits on the chart

Across 744 drip brews, titratable acidity rose in proportion to strength and showed no relationship with extraction yield: stronger coffee contains proportionally more acid, however far extraction went. Trained-panel sourness followed titratable acidity but also depended strongly on extraction, and was highest at high strength and low extraction [8]. A plausible explanation is that acids dissolve early and fast, so at low extraction they dominate the cup. At higher extraction, bitter and roasted compounds that dissolve later mask sourness. Chapter 1 discussed how one taste can suppress another.

~~~quiz
title = "Check your understanding: Sections 3 to 6"
[[q]]
type = "numeric"
prompt = "If the diffusion time constant of an 800 &micro;m particle is 80 s, what is it for a 200 &micro;m particle of the same material, in seconds?"
answer = 5
tolerance = 0.01
unit = "s"
explain = "&tau; scales with d<sup>2</sup>: 80 &times; (200/800)<sup>2</sup> = 5 s."

[[q]]
type = "choice"
prompt = "Two drip brews have identical strength and extraction yield but were brewed at 87 &deg;C and 93 &deg;C. You should expect"
options = ["a large difference in bitterness", "little sensory difference", "the cooler brew to be more acidic"]
answer = 1
explain = "At fixed strength and extraction, a trained panel found little difference across this range."

[[q]]
type = "numeric"
prompt = "On the percolation chart, what beverage-to-dose ratio passes through 1.35% TDS at 20.25% extraction?"
answer = 15
tolerance = 0.01
unit = ""
explain = "m<sub>b</sub>/m<sub>c</sub> = E/S = 20.25/1.35 = 15.0."
~~~

## 7. Agitation

Stirring does not change the equilibrium, but it speeds the approach to it. Around each particle sits a thin layer of liquid that moves with the particle and is depleted of solute faster than it is replaced. Solute must diffuse across this {{boundary layer}} as well as through the particle. Stirring thins the layer and carries fresh liquid to the particle surface. In immersion brewing, a stir at the start matters most, because it breaks up dry clumps and the floating crust so that all the coffee is wetted. In percolation, the pour itself agitates the bed, as Chapter 2B shows.

## Looking ahead

Immersion is the simplest case: all the water meets all the coffee, and equilibrium sets the end point. Most coffee is brewed differently, by pushing water through a bed of grounds. Then the flow rate decides how long the water and coffee are in contact, and the bed itself resists the flow. Chapter 2B asks what controls that flow, and why espresso behaves so differently from filter coffee.

## Key equations

~~~equations
[[eq]]
name = "Brew ratio"
formula = "r = m<sub>w</sub> / m<sub>c</sub>"
note = "Equation 2A.1."
[[eq]]
name = "Extraction yield, percolation"
formula = "E = S m<sub>b</sub> / m<sub>c</sub>"
note = "Equation 2A.2."
[[eq]]
name = "Extraction yield, immersion"
formula = "E = r S / (1 &minus; S)"
note = "Equation 2A.3."
[[eq]]
name = "Equilibrium yield and strength"
formula = "E<sub>eq</sub> = K E<sub>max</sub>; S<sub>eq</sub> = K E<sub>max</sub> / (r + K E<sub>max</sub>)"
note = "Equations 2A.5 and 2A.6."
[[eq]]
name = "Diffusion time constant, sphere"
formula = "&tau; = a<sup>2</sup> / (&pi;<sup>2</sup> D)"
note = "Equation 2A.7."
[[eq]]
name = "Two-population kinetics"
formula = "S(t) = S<sub>&infin;</sub>[1 &minus; &phi;e<sup>&minus;t/&tau;<sub>1</sub></sup> &minus; (1 &minus; &phi;)e<sup>&minus;t/&tau;<sub>2</sub></sup>]"
note = "Equation 2A.8."
~~~

~~~keyterms
~~~

## Labs

~~~lab
id = "L2.1"
title = "Immersion extraction curve and equilibrium constant"
aim = "Measure an immersion extraction curve, fit its slow time constant, and estimate K from equilibrium brews at several ratios."
safety = ["Hot water and glass: use heat-resistant vessels and pour slowly."]
materials = ["Refractometer (or analytical balance for drying)", "Kettle and thermometer", "Three identical heat-resistant vessels with lids", "Syringe filters or paper filter cones", "One coffee, ground for French press"]
method = [
  "Brew 15 g in 250 g of water at a recorded temperature, stir once at 30 s, and cover.",
  "At 1, 2, 4, 8 and 16 minutes, draw about 5 mL from mid-depth, filter it, seal it and let it cool.",
  "Read all samples at room temperature.",
  "Separately brew 15 g at ratios of 8, 12, 16 and 20, covered, for 20 minutes, and read the final TDS of each.",
]
columns = ["Run", "Ratio", "Time (min)", "TDS (%)", "ln(1 &minus; S/S<sub>&infin;</sub>)"]
rows = 10
analysis = [
  "Fit &tau;<sub>2</sub> and &phi; as in Section 4.",
  "For the ratio series, compute E = rS/(1 &minus; S) for each brew. Is E independent of r? Estimate K E<sub>max</sub> as their mean.",
  "Plot S against 1/r and compare with Equation 2A.6.",
]
expected = "Extraction yields near 20 to 23% at all ratios if equilibrium is reached; strength roughly proportional to 1/r."
~~~

~~~lab
id = "L2.2"
title = "A nine-brew grid on the control chart"
aim = "Map how grind and ratio move brews on the chart, and taste the results blind."
materials = ["Cone dripper and filters", "Grinder", "Refractometer", "Scale and kettle", "Cups labelled with random codes"]
method = [
  "Choose three grind settings and three beverage-to-dose ratios (for example 15, 16.5 and 18).",
  "Brew all nine combinations in random order on one day; record dose, beverage mass and TDS.",
  "Have a partner code the cups; taste blind and score sourness, bitterness and liking on 0 to 15 scales.",
]
columns = ["Code", "Grind", "Ratio", "Beverage (g)", "TDS (%)", "EY (%)", "Sour", "Bitter", "Liking"]
rows = 9
analysis = [
  "Plot each brew on the chart and label it with its scores.",
  "Where is bitterness highest? Where is sourness highest? Compare with Section 6.",
]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "1"
prompt = "State the difference between strength and extraction yield, and describe a brew that is strong but under-extracted."
solution = "Strength is the concentration of dissolved solids in the drink; extraction yield is the fraction of the dose that dissolved. A coarse-ground brew at a low ratio (for example 1:10) can reach high strength while extracting only 16 to 17% of the dose."

[[p]]
tier = "A"
section = "1"
prompt = "Why is the percolation formula a reasonable approximation for pour-over but not for immersion?"
solution = "In pour-over the liquid left in the bed is the last and weakest water to pass through, so its solids are a small share of the total. In immersion the retained liquid has full beverage strength, so ignoring its solids is a large, systematic error."

[[p]]
tier = "B"
section = "2"
prompt = "Derive Equation 2A.6 for the equilibrium strength, starting from Equations 2A.3 and 2A.5."
solution = "Set r S/(1 &minus; S) = K E<sub>max</sub> and write E for K E<sub>max</sub>. Multiply both sides by (1 &minus; S): r S = E &minus; E S. Collect the S terms: S(r + E) = E. Divide: S = E/(r + E)."

[[p]]
tier = "B"
section = "2"
prompt = "Using K = 0.717 and E<sub>max</sub> = 0.30, predict the equilibrium strength for immersion ratios of 10, 15 and 20."
solution = "K E<sub>max</sub> = 0.2151. S = 0.2151/(r + 0.2151): r = 10 gives 2.11%; r = 15 gives 1.41%; r = 20 gives 1.06%. The yield is 21.5% in every case."

[[p]]
tier = "B"
section = "1"
prompt = "A 300 g pour-over from 20 g of coffee yields 262 g of beverage. Estimate the liquid retained per gram of grounds."
solution = "Retained liquid &asymp; 300 &minus; 262 = 38 g, neglecting evaporation and the small mass of dissolved solids. r<sub>ret</sub> &asymp; 38/20 = 1.9 g/g."

[[p]]
tier = "B"
section = "3"
prompt = "If &tau; for a 600 &micro;m particle is 45.6 s, what is it for 400 &micro;m and 1,000 &micro;m particles?"
solution = "&tau; scales as d<sup>2</sup>. 400 &micro;m: 45.6 &times; (400/600)<sup>2</sup> = 20.3 s. 1,000 &micro;m: 45.6 &times; (1000/600)<sup>2</sup> = 126.7 s."

[[p]]
tier = "B"
section = "3"
prompt = "Using only the slow exponential, what fraction of a particle's solute remains after three time constants?"
solution = "The remaining fraction decays as e<sup>&minus;t/&tau;</sup>, so after 3&tau; about e<sup>&minus;3</sup> = 5.0% remains."

[[p]]
tier = "B"
section = "6"
prompt = "A brew lies at 1.20% TDS and 18.0% extraction yield with a beverage-to-dose ratio of 15. You grind finer and reach 20.4% extraction at the same ratio. What is the new strength?"
solution = "On a fixed ratio line, S = E/(m<sub>b</sub>/m<sub>c</sub>) = 20.4/15 = 1.36%."

[[p]]
tier = "B"
section = "6"
prompt = "What beverage-to-dose ratio places a brew at 1.25% TDS and 20.0% extraction yield?"
solution = "m<sub>b</sub>/m<sub>c</sub> = E/S = 20.0/1.25 = 16.0."

[[p]]
tier = "B"
section = "4"
prompt = "Repeat the fitting example with S<sub>&infin;</sub> = 1.38%. How sensitive is &tau;<sub>2</sub> to the assumed final strength?"
solution = "ln(1 &minus; S/1.38) at 4, 8 and 16 min is &minus;2.04, &minus;2.98 and &minus;3.83. The least-squares slope is &minus;0.143 per minute, so &tau;<sub>2</sub> &asymp; 7.0 min instead of 4.4 min. A 1.5% change in the assumed final strength changes &tau;<sub>2</sub> by more than half, so S<sub>&infin;</sub> must be measured, not guessed."

[[p]]
tier = "C"
section = "2"
prompt = "A barista claims that steeping a French press for 8 minutes instead of 4 over-extracts it. Using the equilibrium model and the kinetics of Section 4, evaluate the claim and design a test."
solution = "If the brew is near equilibrium by 4 minutes, its yield cannot rise much further, so over-extraction in the sense of higher yield is limited. Measure strength at both times to check. Flavour may still change through cooling, aroma loss, oxidation and continued slow extraction from large particles if equilibrium was not reached. Test: brew in duplicate, measure TDS at 4 and 8 minutes, and run a triangle test between the two times with samples served at the same temperature."

[[p]]
tier = "C"
section = "2"
prompt = "Why is one equilibrium constant unlikely to describe caffeine, chlorogenic acids and melanoidins equally well? What experiment would test this?"
solution = "The substances differ in size, polarity and binding to the coffee matrix: large melanoidins move slowly and may be partly bound, while small caffeine molecules move quickly. Measure each compound (by HPLC or UV-Vis, Appendix D) in equilibrium brews at several ratios and fit a separate K for each."

[[p]]
tier = "C"
section = "3"
prompt = "Using the d<sup>2</sup> scaling, explain why a grinder that produces more fines at the same median size can produce a stronger brew in a fixed short contact time."
solution = "Fines empty in seconds, so in a short brew they release nearly all their solubles while large particles release only their surface material. More fines at the same median size means more mass in the fast population, raising strength and yield for the same contact time."

[[p]]
tier = "C"
section = "6"
prompt = "Propose an explanation of why sourness is highest at high strength and low extraction, while titratable acidity depends only on strength."
solution = "Acids dissolve quickly, so their amount follows strength. At low extraction the bitter and roasted compounds that dissolve later are scarce, so nothing masks sourness. At higher extraction those compounds increase and suppress perceived sourness, even though the titratable acid is unchanged."
~~~

## References

1. Liang J., Chan K.C., Ristenpart W.D. (2021), An equilibrium desorption model for the strength and extraction yield of full immersion brewed coffee, *Scientific Reports* 11:6904.
2. Moroney K.M. et al. (2015), Modelling of coffee extraction during brewing using multiscale methods: an experimentally validated model, *Chemical Engineering Science*.
3. Moroney K.M. et al. (2016), Coffee extraction kinetics in a well mixed system.
4. Batali M.E., Ristenpart W.D., Guinard J.-X., brew temperature at fixed strength and extraction has little impact on the sensory profile of drip brew coffee, *Scientific Reports*; summarised by the SCA.
5. Rao N.Z., Fuller M. (2018), Acidity and antioxidant activity of cold brew coffee, *Scientific Reports* 8.
6. Batali M.E. et al. (2023), A new Coffee Brewing Control Chart relating sensory properties and consumer liking to brew strength, extraction yield, and brew ratio, *Journal of Food Science* 88:2168.
7. Fresh Cup, Why the new Coffee Brewing Control Chart is a big deal.
8. Batali M.E., Cotter A.R., Frost S.C., Ristenpart W.D., Guinard J.-X., titratable acidity, pH and sourness in drip coffee, *ACS Food Science and Technology*; summarised in SCA *25* magazine, issue 15.
