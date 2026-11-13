+++
number = 7
title = "Roasting I: heat and mass transfer"
question = "How does heat get into a bean, where does it go, and what can a roast log tell you?"
objectives = [
  "Write an energy balance for a roasting batch, solve it numerically, and explain the shape of a roast curve.",
  "Estimate the Biot number of a bean and the lag between its surface and centre.",
  "Compute rate of rise from logged data and explain the trade-off between noise and delay.",
  "Explain bean expansion, porosity, the glass transition and internal pressure.",
  "Describe the acoustic and chemical signals used to monitor roasting.",
  "Explain common roast defects from the energy balance.",
  "Design an experiment that separates development time from roast colour.",
]
prerequisites = ["Chapter 6, Section 4 (heat budget)", "Appendix C, Sections C.3 and C.6 (Arrhenius, heat transfer)"]
+++
A roaster logs two batches of the same green coffee. Both reach the same end temperature and the same colour, but one took nine minutes and the other fourteen, and they taste different. A third batch reached first crack sooner than expected and scorched. The roaster's software shows each batch as a curve of temperature against time. What does the shape of that curve say about what happened inside the beans?

This chapter treats a roast as a problem in heat and mass transfer: heat flows into the beans from the machine, and water and gas flow out. Sections 1 to 3 set out the routes by which heat arrives and build an energy balance that reproduces the shape of a roast curve. Sections 4 to 7 look inside the bean at temperature gradients, water, reaction heat and structure. Sections 8 to 12 turn to measurement, development time, defects and profile design.

## 1. Safety

Roasting produces smoke, carbon monoxide and chaff that can catch fire. Roast outdoors or under extraction, never leave a running roaster, keep a lid or extinguisher at hand, and remember that roaster bodies and freshly dropped beans stay hot enough to burn skin for several minutes. Carbon monoxide has no smell; commercial roasteries use detectors. A modified popcorn popper runs its motor and heater beyond their design limits, so watch it throughout.

## 2. How heat reaches a bean

Heat reaches a bean by three routes (Appendix C.6). {{Conduction}} carries heat through solid contact, from hot metal or neighbouring beans. {{Convection}} carries heat from a moving fluid, here hot gas, to a surface. {{Radiation}} carries heat as infrared light from any hot surface to cooler ones. Their shares depend on the machine.

A {{drum roaster}} tumbles beans inside a heated rotating cylinder while hot gas is drawn through it. All three routes contribute, and the beans spend much of their time in a moving pile, exchanging heat with each other. A {{fluid-bed roaster}} suspends beans in a fast jet of hot air and heats them almost entirely by convection.

Each route has its own law. Convection delivers a heat flux q = h(T<sub>gas</sub> &minus; T<sub>s</sub>), where h is the heat-transfer coefficient and T<sub>s</sub> the surface temperature. Radiation from a wall at temperature T<sub>w</sub> to a bean surface delivers

~~~eq rad
formula = "q = &epsilon; &sigma; (T<sub>w</sub><sup>4</sup> &minus; T<sub>s</sub><sup>4</sup>)"
where = [["q", "heat flux", "W/m<sup>2</sup>"], ["&epsilon;", "emissivity of the surface, 0 to 1", ""], ["&sigma;", "Stefan-Boltzmann constant, 5.67 &times; 10<sup>&minus;8</sup>", "W/(m<sup>2</sup> K<sup>4</sup>)"], ["T<sub>w</sub>, T<sub>s</sub>", "wall and bean-surface temperatures", "K"]]
~~~

{{Emissivity}} measures how effectively a surface emits and absorbs radiation compared with a perfect absorber.

~~~example Radiation against convection in a drum
Problem. A drum wall at 300 &deg;C radiates to beans at 150 &deg;C with emissivity 0.9. Hot air with h = 17 W/(m<sup>2</sup> K) flows past the same beans 100 K hotter than them. Compare the two heat fluxes.

Plan. Convert to kelvin for {eq:rad}; the convective flux is h&Delta;T.

Solution. Radiation: 0.9 &times; 5.67 &times; 10<sup>&minus;8</sup> &times; (573<sup>4</sup> &minus; 423<sup>4</sup>) = 3.9 kW/m<sup>2</sup>. Convection: 17 &times; 100 = 1.7 kW/m<sup>2</sup>.

Check. The fourth power makes radiation grow rapidly with wall temperature. A 2025 study fitted overall coefficients of 60 to 103 W/(m<sup>2</sup> K) at air temperatures of 200 to 280 &deg;C [1]. The convective part alone was only 16 to 17, so radiation dominated in that roaster at low air speed.
~~~

## 3. The batch energy balance

To predict how bean temperature changes, we write an {{energy balance}}: the rate at which a system's stored heat rises equals the rate at which heat enters minus the rate at which it leaves, plus any heat generated inside. Following the approach of Chapter 2A, we treat the whole batch as one lump at a single temperature T. Heat enters through an effective transfer term, is spent warming dry matter and water and evaporating water, and is added by reactions that release heat:

~~~eq balance
formula = "C dT/dt = hA (T<sub>air</sub> &minus; T) &minus; L<sub>v</sub> (&minus;dm<sub>w</sub>/dt) + Q<sub>rxn</sub>"
where = [["C", "heat capacity of the batch, m<sub>dry</sub>c<sub>p,dry</sub> + m<sub>w</sub>c<sub>p,water</sub>", "J/K"], ["hA", "effective heat-transfer coefficient times area for the whole batch", "W/K"], ["T<sub>air</sub>", "effective temperature of the heat source", "&deg;C"], ["L<sub>v</sub>", "latent heat of vaporisation, 2,257", "kJ/kg"], ["m<sub>w</sub>", "mass of water in the beans", "kg"], ["Q<sub>rxn</sub>", "rate of heat release by reactions", "W"]]
~~~

The left side is the rate at which the batch stores heat. The first term on the right is heat in; the second is heat used to evaporate water (dm<sub>w</sub>/dt is negative while water leaves, so &minus;dm<sub>w</sub>/dt is positive); the third is reaction heat. Chapter 6 supplied c<sub>p,dry</sub> &asymp; 1.4 kJ/(kg K) and L<sub>v</sub>. The total heat released by roasting reactions has been estimated at about 232 kJ per kg of dry matter, starting near 150 &deg;C [1]. A reaction that releases heat is {{exothermic}}; one that absorbs heat is {{endothermic}}.

~~~example The first minute
Problem. A 1 kg batch at 11% moisture is charged at 25 &deg;C into a roaster with an effective hA of 6 W/K per kg and T<sub>air</sub> = 250 &deg;C. What is the initial heating rate?

Plan. At charge no water is evaporating yet and no reaction heat is released, so only the first term of {eq:balance} acts.

Solution. The batch heat capacity is C = 0.89 &times; 1.4 + 0.11 &times; 4.18 = 1.71 kJ/K. Heat flow: 6 &times; (250 &minus; 25) = 1,350 W. Heating rate: 1,350/1,710 = 0.79 K/s, or 47 &deg;C/min.

Check. As T rises the driving difference T<sub>air</sub> &minus; T shrinks, so the rate must fall; the simulator below shows it doing so.
~~~

Solving {eq:balance} by hand is impractical, because the evaporation and reaction terms themselves depend on temperature. The calculator below solves it step by step: it computes every term at the current temperature, advances time by a small step, updates the temperature and water content, and repeats.

~~~widget roast-sim
~~~

{fig:roastmodel} shows the model with and without its evaporation and reaction terms. Early on, the large temperature difference drives a fast rise. Between about 100 and 160 &deg;C, evaporation absorbs much of the incoming heat and the rise slows to a plateau. Above 150 &deg;C the reactions add heat. Bustos-Vanegas and colleagues found that leaving evaporation out of their bean model gave larger errors than leaving out the reaction heat [1], which the figure reproduces.

~~~figure
id = "roastmodel"
file = "ch07-roastmodel.gif"
alt = "Simulated bean temperature against time for three versions of the lumped model: full model, model without reaction heat, and model without evaporation; the no-evaporation curve rises much faster through the middle of the roast."
caption = "The teaching model with and without its evaporation and reaction terms (1 kg, 11% moisture, T<sub>air</sub> 250 &deg;C). Compare the middle of the roast, where evaporation matters most, with the end, where reaction heat shortens the roast."
~~~

The teaching model also shows its limits. With a constant air temperature, its heating rate climbs at the end as reaction heat accelerates, and nearly all the water leaves. Real roasters reduce heat late in the roast, and real beans finish with 1.5 to 4% moisture, because removing the last water is limited by vapour diffusing out of the bean rather than by heat supply [1]. The model's effective hA of 6 W/K per kg corresponds, at about 1.1 m<sup>2</sup> of bean surface per kg, to only about 5.5 W/(m<sup>2</sup> K), far below single-bean values. In a drum, most bean surface is hidden inside the pile at any moment, and the roaster's heat supply is shared by the whole batch.

~~~quiz
title = "Check your understanding: Sections 2 and 3"
[[q]]
type = "choice"
prompt = "According to Bustos-Vanegas et al., omitting which term from the bean energy balance gave the larger error?"
options = ["Evaporation", "The reaction heat", "Radiation"]
answer = 0
explain = "Evaporation, which begins near 100 &deg;C, is the largest heat sink in the middle of the roast."

[[q]]
type = "numeric"
prompt = "In the lumped model, what is the initial heating rate in &deg;C/min of a 1 kg batch at 13% moisture with hA = 6 W/K, T<sub>air</sub> = 250 &deg;C and T = 25 &deg;C?"
answer = 46
tolerance = 1
unit = "&deg;C/min"
explain = "C = 0.87 &times; 1.4 + 0.13 &times; 4.18 = 1.761 kJ/K. 1,350 W/1,761 J/K = 0.767 K/s = 46 &deg;C/min."
~~~

## 4. Inside the bean: the Biot number

The energy balance treats each bean as having one temperature. Whether that is reasonable depends on a comparison: how fast heat arrives at the surface against how fast it travels inward. The {{Biot number}} expresses the comparison:

~~~eq biot
formula = "Bi = h L<sub>c</sub> / &lambda;"
where = [["Bi", "Biot number", "dimensionless"], ["h", "heat-transfer coefficient at the surface", "W/(m<sup>2</sup> K)"], ["L<sub>c</sub>", "characteristic length, volume divided by surface area", "m"], ["&lambda;", "thermal conductivity of the bean", "W/(m K)"]]
~~~

Below about 0.1, the inside keeps pace with the surface and one temperature is enough. Chapter 6 estimated Bi for a green bean at 0.2 to 0.8 depending on h; the 2025 study reported 0.48 to 0.82 [1]. The surface therefore heats ahead of the centre. A computational study cited there found about 16 &deg;C between bean centre and surface during roasting [1].

How long does the centre take to catch up? Heat spreads through a solid by a diffusion process like the one in Chapter 2A, with the {{thermal diffusivity}} &alpha; = &lambda;/(&rho;c<sub>p</sub>) in place of D. The time scale is about r<sup>2</sup>/&alpha;, which for a bean of radius 3 mm is roughly two minutes (Appendix C.6). In a fast roast, the surface can reach temperatures where browning proceeds quickly while the centre is still drying, which is the physical basis of several defects (Section 11).

## 5. Water and the drying phase

At first, water moves to the bean surface and evaporates. As the outer layers dry, the remaining water must leave as vapour through the bean, so the drying rate falls even though heat keeps arriving. Most of the green bean's 10 to 12% moisture leaves in the first two thirds of a roast. The drying phase consumes about half the heat a roast requires (Chapter 6, Section 4) and builds the steam pressure that later drives expansion.

## 6. Reaction heat

Once the bean is hot enough, the reactions of Chapter 8 (browning, sugar breakdown and breakdown of chlorogenic acids) release heat overall. In the model this appears as a rate that rises exponentially with temperature, following the Arrhenius equation (Appendix C.3). Roasters see it as a change in the slope of the temperature curve near and after first crack. Controlling it is one reason heat input is reduced late in a roast. The figure of 232 kJ/kg comes from one kinetic model; published values vary, and some studies find that parts of the roast absorb heat overall.

## 7. Structure, expansion and gas

Roasting turns a dense, moist, tough bean into a porous, dry, brittle one. Schenker's study at ETH Zurich compared two kinds of hot-air roast matched on colour and mass loss [2]. One used a high temperature for a short time (260 &deg;C, 155 to 180 s); the other a low temperature for a long time (220 &deg;C, 540 to 720 s). That work and the literature it reviews report four findings.

- Beans expand more at higher roasting temperature: their volume grew by a factor of 1.59 to 1.84 across 220 to 295 &deg;C for a Colombian arabica.
- Higher-temperature roasts have larger pores at equal roast degree.
- The porosity of roasted beans is about 0.38 to 0.49.
- The cell-wall material passes through a {{glass transition}}, cited near 216 &deg;C under roasting conditions, above which it softens from rigid and glassy to rubbery and can stretch under internal pressure. Internal pressures from steam and CO<sub>2</sub> reach about 550 to 800 kPa.

Expansion therefore depends on the path as well as the end point: a fast roast expands and opens the bean more. That changes degassing (Chapter 5), grinding (Chapter 4) and extraction (Chapter 2A), even at equal colour.

## 8. First and second crack

{{First crack}} is an audible popping as internal pressure fractures the bean structure, often starting near 195 to 205 &deg;C on typical probes. {{Second crack}}, in darker roasts, comes from further breakdown of the weakened cell walls. Wilson recorded both acoustically and found first-crack events about 15% louder in peak pressure than second-crack events, at a frequency nearly 19 times lower, with second-crack events about five times more frequent [3]. The difference is large enough for automatic detection.

## 9. Measuring the process

### Probes and the roast log

A bean-temperature probe sits in the tumbling mass of beans and reads somewhere between bean surfaces, the air between them and its own lagging metal. It reads neither the bean's centre nor its surface. Probes of different thickness on one roaster give different curves, and position matters. Treat logged bean temperature as a reproducible index for one machine and one probe.

A roast log uses standard landmarks. {{Charge}} is the moment green coffee enters. The probe reading falls as cold beans surround it, reaches a {{turning point}}, then rises. {{Rate of rise}} (RoR) is the slope of the bean-temperature curve, the heating rate &gamma; in &deg;C per minute. {{Drop}} is the end of the roast. {{Development time}} is the time from first crack to drop, and the {{development time ratio}} is that time as a fraction of the whole roast:

~~~eq dtr
formula = "DTR = (t<sub>drop</sub> &minus; t<sub>FC</sub>) / t<sub>drop</sub>"
where = [["DTR", "development time ratio", "fraction"], ["t<sub>drop</sub>", "time from charge to drop", "s"], ["t<sub>FC</sub>", "time from charge to first crack", "s"]]
~~~

~~~widget roastlog
~~~

### Rate of rise and noise

Rate of rise is a derivative, and differentiating noisy data amplifies the noise. The usual remedy is to fit a straight line by least squares through a window of recent readings and report its slope.

~~~example Noise against smoothing
Problem. A probe logs once per second with random noise of standard deviation 0.2 &deg;C. Compare the noise in a rate of rise computed from consecutive readings with one computed by least squares over a 31-second window.

Plan. For consecutive readings, the difference of two noisy values has standard deviation 0.2&radic;2. For a least-squares slope through n evenly spaced points, the standard deviation is &sigma;/&radic;[n(n<sup>2</sup> &minus; 1)/12] (Appendix E.5).

Solution. Consecutive readings: 0.2 &times; &radic;2 = 0.28 &deg;C/s, or 17 &deg;C/min, larger than the signal itself. With n = 31: 0.2/&radic;(31 &times; 960/12) = 0.004 &deg;C/s, or 0.24 &deg;C/min.

Check. The cost is delay: a 31-second window reports the average slope centred 15 s in the past, and smooths away changes shorter than the window, such as the brief jump at first crack.
~~~

~~~widget ror
~~~

### Colour

Roast colour is measured by reflectance on whole beans or ground coffee (Chapter 0, Section 7). Low-cost optical sensors have been validated against reference methods for consistency within a roastery [4]. Ground colour reflects the whole bean; whole-bean colour mostly reflects the surface. The difference between them is itself a measure of how far the centre lags the surface.

### Watching the chemistry

{{Proton-transfer-reaction mass spectrometry}} (PTR-MS) identifies volatile compounds in a gas stream by giving each molecule a proton and weighing the resulting ion (Appendix D.7). Wieland and colleagues sampled roaster exhaust once per second across 42 drum roasts [5]. The signals changed sharply at the switch from heat-absorbing drying to heat-releasing reactions (about 625 s in one dark roast) and again at first crack (about 730 s). Hot-air temperature mainly set the speed of the process. A statistical model built from 23 ion signals separated roast degrees and predicted the final roast degree to within about one colour unit after first crack.

## 10. Development time: what the evidence shows

Development time is widely used to describe and control roasts, but its effect is easily confused with others. Lengthening development at a fixed end temperature also changes the whole time-temperature history, and usually the colour.

Alstrup and colleagues (2020) roasted one coffee with four profiles that differed subtly in development time and analysed the brews by descriptive sensory analysis and two chemical methods [6]. Longer development shifted the chemical and sensory profiles significantly: shorter development was associated with fruity, sweet and acidic attributes, longer with roasty, nutty and bitter ones. Related work by the same group examined how development time and roast colour relate [7]. Development time is therefore a demonstrated control variable. What remains open is how much of its effect is separate from colour and from the total heat the bean receives.

~~~misconception
"A development time ratio of 20 to 25% is what makes a roast well developed." No controlled study has established a universal target ratio. The ratio depends on how long the roast took to reach first crack, so the same development time gives different ratios in fast and slow roasts. The evidence supports development time as a variable that changes flavour, to be tuned for each coffee, not a fixed target.
~~~

~~~example Separating development time from colour
Problem. Design an experiment to test whether development time changes sourness at fixed roast colour.

Plan. Development time and colour are linked, so they must be crossed deliberately in a factorial design (Chapter 9).

Solution. Cross end colour at two levels (for example ground reflectance 65 and 55 on one instrument) with development time at two levels (75 and 150 s), giving four profiles. Reaching the same colour with different development times requires changing the heat earlier in the roast: more heat before first crack and a gentler finish for the long-development profile. Roast each profile three times on separate days in random order, verify colour on ground coffee, and analyse sourness scores and titratable acidity with a two-way ANOVA (Appendix E.4).

Check. A significant development-time effect at fixed colour shows an effect beyond colour. A significant interaction shows that the effect depends on roast degree. Chapter 9 works through this design with data.
~~~

## 11. Defects explained by the energy balance

Each common defect points to a term in the energy balance.

- Scorching: dark patches on the flat face of beans, from intense conduction where beans rest against a very hot drum early in the roast, while they are still moist and moving slowly.
- Tipping: burnt ends, where thin geometry and escaping steam concentrate heating; associated with very high early heat input.
- Facing: burns on the flat face from contact with the drum, in roasters with too little bean movement or too high a drum temperature.
- Baking: a flat, papery, bready flavour, associated with a roast whose rate of rise stalls or falls too low for too long. It is described from experience; controlled sensory studies of it are scarce.
- Underdevelopment: grassy, sour, cereal flavours and a pale centre, when the surface reaches the target colour but the centre lags (high Biot number, short roast). Ground colour is then noticeably lighter than whole-bean colour.

Scorching, tipping and facing come from excessive heat flux at the surface. Baking comes from too little heat at the wrong time. Underdevelopment comes from too little time for the centre to follow the surface.

## 12. Designing a profile

A {{roast profile}} is a plan for heat input over time. The energy balance identifies the controls. Charge temperature and early heat set the drying phase. Heat in the middle of the roast sets the pace of browning. Reducing heat before and through first crack controls the reaction heat and development. Drop temperature, or colour, sets the end point. Because different profiles can reach the same colour, specify a profile by its measured outcomes (colour, mass loss, development time) and evaluate it by tasting and, where possible, by titratable acidity and extraction yield at a fixed recipe.

## Looking ahead

This chapter treated the reactions inside the bean as a source of heat. Chapter 8 opens them up: which reactions run, what they make and destroy, and how the temperature path decides the balance between acids, bitter compounds and aroma.

## Key equations

~~~equations
[[eq]]
name = "Radiative heat flux"
formula = "q = &epsilon;&sigma;(T<sub>w</sub><sup>4</sup> &minus; T<sub>s</sub><sup>4</sup>)"
note = "Equation 7.1. Kelvin."
[[eq]]
name = "Batch energy balance"
formula = "C dT/dt = hA(T<sub>air</sub> &minus; T) &minus; L<sub>v</sub>(&minus;dm<sub>w</sub>/dt) + Q<sub>rxn</sub>"
note = "Equation 7.2."
[[eq]]
name = "Biot number"
formula = "Bi = h L<sub>c</sub> / &lambda;"
note = "Equation 7.3. One temperature is adequate if Bi &lt; 0.1."
[[eq]]
name = "Development time ratio"
formula = "DTR = (t<sub>drop</sub> &minus; t<sub>FC</sub>) / t<sub>drop</sub>"
note = "Equation 7.4."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "numeric"
prompt = "A roast has first crack at 9:30 and drop at 11:30. What is the development time ratio, in percent?"
answer = 17.4
tolerance = 0.1
unit = "%"
explain = "120 s/690 s = 17.4%."

[[q]]
type = "choice"
prompt = "At equal roast colour, a high-temperature, short-time roast usually gives"
options = ["more expansion and larger pores", "less expansion", "identical structure"]
answer = 0
explain = "Higher temperatures expand the bean more and create larger pores."

[[q]]
type = "choice"
prompt = "Widening the rate-of-rise window from 15 to 60 seconds will"
options = ["reduce noise and increase delay", "increase noise and reduce delay", "change neither"]
answer = 0
explain = "Fitting more points reduces noise but reports a slope centred further in the past."
~~~

## Labs

~~~lab
id = "L7.1"
title = "Two profiles, one colour"
aim = "Roast one coffee by two profiles to the same colour and mass loss, and test whether they differ in the cup."
safety = ["Follow Section 1: extraction, supervision, fire control and carbon monoxide awareness."]
materials = ["Roaster with logging", "Colour meter or a fixed reference for ground colour", "Balance", "Cupping or brewing equipment"]
method = [
  "Profile A is fast (higher charge temperature and early heat); profile B is slow (lower charge temperature, gentler throughout). Aim for the same ground colour.",
  "Roast each profile three times; record the log, mass loss, colour, and first crack and drop times.",
  "Rest both equally, brew at matched extraction yield, and run triangle tests and descriptive scoring blind.",
]
columns = ["Roast", "Profile", "Total time", "Dev. time", "Mass loss (%)", "Ground colour", "Notes"]
rows = 6
analysis = ["Did the profiles match on colour and mass loss? If so, did tasters detect a difference?"]
~~~

~~~lab
id = "L7.2"
title = "Rate of rise from a log"
aim = "Compute rate of rise from your own logged data and choose a window."
materials = ["A roast log exported at 1 to 5 s resolution", "The rate-of-rise calculator or a spreadsheet"]
method = ["Compute rate of rise with windows of 3, 15, 30 and 60 s.", "Mark the turning point, first crack and drop."]
columns = ["Window (s)", "Noise before first crack (SD)", "RoR at first crack", "First crack visible?"]
rows = 4
analysis = ["Which window best balances noise against delay for your roaster?"]
~~~

~~~lab
id = "L7.3"
title = "Listening to cracks"
aim = "Record first and second crack and compare their event rates and frequencies."
materials = ["Phone with a recording app", "Free audio software with a spectrogram view"]
method = ["Record a roast through first crack and into second crack, with the phone at a fixed distance.", "Count crack events per 10 s and inspect spectrograms of each type."]
columns = ["Interval", "First-crack events", "Second-crack events", "Dominant frequency"]
rows = 8
analysis = ["Compare the event rates and frequencies with Wilson's results."]
~~~

~~~lab
id = "L7.4"
title = "Expansion and density"
aim = "Measure the change in volume and density on roasting for two profiles."
materials = ["1 L cylinder", "Balance", "Green and roasted beans from L7.1"]
method = ["Measure the bulk density of the green coffee and of each roasted batch; compute mass loss and relative volume per gram of green coffee."]
columns = ["Sample", "Bulk density (g/L)", "Mass loss (%)", "Relative volume"]
rows = 3
analysis = ["Relative volume = (green density/roasted density) &times; (1 &minus; mass loss). Which profile expanded more?"]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "2"
prompt = "Name the three routes of heat transfer in a drum roaster, and say which dominates in a fluid-bed roaster."
solution = "Conduction, convection and radiation. A fluid-bed roaster heats mainly by convection."

[[p]]
tier = "A"
section = "9"
prompt = "Why should logged bean temperatures from two different roasters not be compared directly?"
solution = "The probe reads a mixture of bean surface, air and its own lagging metal. Probe size, position and roaster design all change the reading, so the numbers are indices for one setup."

[[p]]
tier = "A"
section = "9"
prompt = "A batch of 12.0 kg of green coffee yields 10.25 kg roasted. Compute the mass loss."
solution = "(12.0 &minus; 10.25)/12.0 = 14.6%."

[[p]]
tier = "B"
section = "2"
prompt = "Compute the radiative flux from a drum wall at 280 &deg;C to beans at 180 &deg;C with emissivity 0.85."
solution = "q = 0.85 &times; 5.67 &times; 10<sup>&minus;8</sup> &times; (553.15<sup>4</sup> &minus; 453.15<sup>4</sup>) = 0.85 &times; 5.67 &times; 10<sup>&minus;8</sup> &times; (9.362 &times; 10<sup>10</sup> &minus; 4.217 &times; 10<sup>10</sup>) = 2.48 kW/m<sup>2</sup>."

[[p]]
tier = "B"
section = "3"
prompt = "Use the simulator to compare drop times at T<sub>air</sub> = 250 and 230 &deg;C, and at 11% and 13% moisture. Which change has the larger effect, and why?"
solution = "With the defaults, 250 &deg;C and 11% gives about 13.2 min; 230 &deg;C gives about 16.1 min; 13% moisture at 250 &deg;C gives about 14.3 min. Air temperature has the larger effect because it sets the driving difference throughout the roast; extra moisture adds a fixed evaporation load early."

[[p]]
tier = "B"
section = "9"
prompt = "Estimate the standard deviation of a rate of rise computed by least squares over 11 readings at 1 s spacing, with probe noise of 0.3 &deg;C."
solution = "n(n<sup>2</sup> &minus; 1)/12 = 11 &times; 120/12 = 110 s<sup>2</sup>. Standard deviation = 0.3/&radic;110 = 0.0286 &deg;C/s = 1.7 &deg;C/min."

[[p]]
tier = "B"
section = "7"
prompt = "Green bulk density is 700 g/L and roasted bulk density 380 g/L after a 15% mass loss. Compute the relative volume per gram of green coffee."
solution = "(700/380) &times; 0.85 = 1.57."

[[p]]
tier = "B"
section = "6"
prompt = "Using 232 kJ per kg of dry matter, how much reaction heat would a 15 kg batch at 11% moisture release if all of it were released by drop? Compare with the heat needed for evaporation."
solution = "Dry matter: 13.35 kg &times; 232 = 3,097 kJ. Water: 1.65 kg &times; 2,257 = 3,724 kJ. The reaction heat is of the same order as the evaporation load; in practice only part of it is released by a typical drop."

[[p]]
tier = "B"
section = "3"
prompt = "Why does the teaching model finish with almost no moisture while real roasted beans keep 1.5 to 4%?"
solution = "The model's drying rate depends only on temperature and remaining water. Real removal of the last water is limited by vapour diffusing out through a drying, hardening bean, and some water is bound to the solid, so drying slows far more than the model allows."

[[p]]
tier = "C"
section = "4"
prompt = "Explain, using the Biot number and the internal time scale, why ground colour can be lighter than whole-bean colour after a fast roast, and what that predicts for the cup."
solution = "With Bi above 0.1 and an internal time scale near two minutes, a fast roast heats the surface ahead of the centre. The surface browns to the target colour while the centre is less developed. Grinding mixes centre and surface, so ground colour is lighter. The cup may show underdeveloped grassy, sour or cereal notes alongside roasty surface notes."

[[p]]
tier = "C"
section = "11"
prompt = "A roaster claims that rate of rise must decline continuously to avoid baked coffee. Frame this as a testable hypothesis and design the test."
solution = "Hypothesis: at matched colour, mass loss and development time, profiles with a flat or rising rate of rise over a stated segment produce lower sensory scores, or more frequent 'baked' descriptors, than profiles with a continuously declining rate. Design two profiles differing only in the shape of that segment, matched on the outcome measures, with three roasts each in random order; run blind triangle tests followed by descriptive analysis including a 'baked/bready' attribute anchored by a reference."

[[p]]
tier = "C"
section = "7"
prompt = "Why does a high-temperature, short-time roast change extraction yield at equal colour?"
solution = "It expands the bean more and creates larger pores, so the roasted structure is more porous and brittle. It grinds differently, often finer and with more fines at a given setting, and water penetrates more easily. Both effects raise extraction at a given grind and recipe."

[[p]]
tier = "C"
section = "9"
prompt = "Propose how online exhaust measurements could replace or supplement bean probes for profile control."
solution = "Exhaust ion signals respond to chemistry rather than to a probe's contact with beans. Sharp changes mark the onset of heat-releasing reactions and first crack, and a statistical model of the signals can predict roast degree in real time. A controller could end the roast on predicted colour, keeping the probe for safety and early-roast control."
~~~

## References

1. Bustos-Vanegas J.D. et al. (2025), Modeling and simulation of coffee bean heating during roasting: effect of heat generation, *Frontiers in Food Science and Technology*.
2. Schenker S. (2000), Investigations on the hot air roasting of coffee beans, Dissertation ETH Zurich No. 13620.
3. Wilson P.S. (2014), Coffee roasting acoustics, *Journal of the Acoustical Society of America*; summarised by Daily Coffee News.
4. Roasted coffee beans characterization through optoelectronic color sensing (Brazilian coffee research repository).
5. Wieland F., Gloess A.N., Keller M., Wetzel A., Schenker S., Yeretzian C. (2012), Online monitoring of coffee roasting by proton transfer reaction time-of-flight mass spectrometry, *Analytical and Bioanalytical Chemistry* 402:2531.
6. Alstrup J., Petersen M.A., Larsen F.H., Münchow M. (2020), The effect of roast development time modulations on the sensory profile and chemical composition of the coffee brew as measured by NMR and DHS-GC-MS, *Beverages* 6:70.
7. Münchow M., Alstrup J., Flavor development: the relationship between time and color in coffee roasting, *Roast Magazine*.
