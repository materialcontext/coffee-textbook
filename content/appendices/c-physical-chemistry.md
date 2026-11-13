+++
number = 0
label = "C"
title = "Physical chemistry and transport"
question = "Which rate laws and transport equations does the book use, and where do they come from?"
objectives = [
  "Integrate first-order and other simple rate laws and use half-lives.",
  "Describe consecutive reactions and find when an intermediate peaks.",
  "Use the Arrhenius equation and Q<sub>10</sub>, and estimate an activation energy from two temperatures.",
  "State Fick's first law, estimate diffusion distances, and interpret the solution for a sphere.",
  "Write the laws of conduction, convection and radiation, and use the Biot number and a lumped model.",
  "Use the ideal gas law and Henry's law for gases in coffee.",
]
+++
Three ideas carry most of the physical reasoning in this book: how fast reactions go and how that depends on temperature; how molecules and heat spread through a material; and how gases behave. This appendix develops each from first-year calculus and chemistry. Chapters 2A, 2B, 5, 7 and 8 refer to its sections.

## C.1 Rate laws

The {{reaction rate}} of a reaction A &rarr; products is how fast the concentration of A falls: r = &minus;d[A]/dt. Experiments often find that the rate is proportional to a power of the concentration, r = k[A]<sup>n</sup>. The constant k is the {{rate constant}}, and n is the {{reaction order}}. The order is found by experiment; it cannot in general be read from the balanced equation.

The most important case is n = 1, a first-order reaction. Its rate law is a differential equation:

~~~eq first
formula = "d[A]/dt = &minus;k[A]"
where = [["[A]", "concentration of A", "mol/L"], ["k", "first-order rate constant", "1/s or 1/min"]]
~~~

To solve it, separate the variables: d[A]/[A] = &minus;k dt. Integrate from time 0, when the concentration is [A]<sub>0</sub>, to time t: ln[A] &minus; ln[A]<sub>0</sub> = &minus;kt. Taking exponentials,

~~~eq firstsol
formula = "[A] = [A]<sub>0</sub> e<sup>&minus;kt</sup>"
where = [["[A]<sub>0</sub>", "concentration at t = 0", "mol/L"]]
~~~

The {{half-life}}, the time for the concentration to halve, follows by setting [A] = [A]<sub>0</sub>/2: t<sub>1/2</sub> = ln 2/k. For a first-order reaction it does not depend on the starting amount. After three half-lives one eighth remains.

Plotting ln[A] against t gives a straight line of slope &minus;k for a first-order process. Curvature in that plot is the first sign that one exponential is not enough, as with the two particle populations of Chapter 2A and the Weibull degassing of Chapter 5.

Other orders integrate in the same way:

| Order | Rate law | Integrated form | Half-life |
|---|---|---|---|
| 0 | r = k | [A] = [A]<sub>0</sub> &minus; kt | [A]<sub>0</sub>/2k |
| 1 | r = k[A] | [A] = [A]<sub>0</sub>e<sup>&minus;kt</sup> | ln 2/k |
| 2 | r = k[A]<sup>2</sup> | 1/[A] &minus; 1/[A]<sub>0</sub> = kt | 1/(k[A]<sub>0</sub>) |

Many processes in coffee are {{pseudo-first-order}}: a second reactant, such as water or oxygen in the headspace, is present in such excess that its concentration barely changes, so it can be absorbed into k.

## C.2 Consecutive reactions

Roasting forms some compounds and then destroys them. The simplest model is a pair of first-order steps, A &rarr; B &rarr; C, with rate constants k<sub>1</sub> and k<sub>2</sub>, starting from pure A. A falls exponentially, as in {eq:firstsol}. Species B is formed from A and lost to C, so d[B]/dt = k<sub>1</sub>[A] &minus; k<sub>2</sub>[B]. Solving this equation gives

~~~eq consec
formula = "[B] = [A]<sub>0</sub> k<sub>1</sub>/(k<sub>2</sub> &minus; k<sub>1</sub>) (e<sup>&minus;k<sub>1</sub>t</sup> &minus; e<sup>&minus;k<sub>2</sub>t</sup>)"
where = [["k<sub>1</sub>, k<sub>2</sub>", "rate constants of the first and second steps", "1/min"]]
~~~

~~~deeper Solving for B
Substitute [A] = [A]<sub>0</sub>e<sup>&minus;k<sub>1</sub>t</sup> into d[B]/dt + k<sub>2</sub>[B] = k<sub>1</sub>[A]. Multiply both sides by the integrating factor e<sup>k<sub>2</sub>t</sup>, which turns the left side into d([B]e<sup>k<sub>2</sub>t</sup>)/dt. Integrate from 0, where [B] = 0, to t: [B]e<sup>k<sub>2</sub>t</sup> = [A]<sub>0</sub>k<sub>1</sub>(e<sup>(k<sub>2</sub> &minus; k<sub>1</sub>)t</sup> &minus; 1)/(k<sub>2</sub> &minus; k<sub>1</sub>). Multiplying by e<sup>&minus;k<sub>2</sub>t</sup> gives {eq:consec}.
~~~

Setting d[B]/dt = 0 gives the time at which B peaks:

~~~eq tmax
formula = "t<sub>max</sub> = ln(k<sub>1</sub>/k<sub>2</sub>) / (k<sub>1</sub> &minus; k<sub>2</sub>)"
where = [["t<sub>max</sub>", "time at which the intermediate B is greatest", "min"]]
~~~

and the peak amount [B]<sub>max</sub>/[A]<sub>0</sub> = (k<sub>2</sub>/k<sub>1</sub>)<sup>k<sub>2</sub>/(k<sub>1</sub> &minus; k<sub>2</sub>)</sup>.

~~~example An intermediate that peaks
Problem. With k<sub>1</sub> = 0.3 and k<sub>2</sub> = 0.1 per minute at constant temperature, when does B peak and how much is there?

Plan. Apply {eq:tmax} and the peak formula.

Solution. t<sub>max</sub> = ln 3/0.2 = 5.49 min. [B]<sub>max</sub>/[A]<sub>0</sub> = (1/3)<sup>0.1/0.2</sup> = 3<sup>&minus;0.5</sup> = 0.577.

Check. Stopping before 5.5 min leaves A unconverted; stopping after it loses B to C. Chapter 8 treats acids formed and destroyed during roasting this way, with temperature rising so that both rate constants change with time.
~~~

~~~widget kinetics
~~~

## C.3 Temperature dependence: the Arrhenius equation

Rate constants rise steeply with temperature. The {{Arrhenius equation}} describes the rise:

~~~eq arr
formula = "k = A exp(&minus;E<sub>a</sub> / RT)"
where = [["A", "pre-exponential factor", "same unit as k"], ["E<sub>a</sub>", "activation energy", "J/mol"], ["R", "gas constant, 8.314", "J/(mol K)"], ["T", "absolute temperature", "K"]]
~~~

The {{activation energy}} is the energy barrier that colliding molecules must overcome to react. The exponential factor is the fraction of molecules with enough energy, and it grows rapidly with temperature. Taking logarithms gives ln k = ln A &minus; (E<sub>a</sub>/R)(1/T), so a plot of ln k against 1/T is a straight line of slope &minus;E<sub>a</sub>/R.

Writing the logarithmic form at two temperatures and subtracting removes A:

~~~eq ea
formula = "E<sub>a</sub> = R ln(k<sub>2</sub>/k<sub>1</sub>) / (1/T<sub>1</sub> &minus; 1/T<sub>2</sub>)"
where = [["k<sub>1</sub>, k<sub>2</sub>", "rate constants at temperatures T<sub>1</sub> and T<sub>2</sub>", ""]]
~~~

The {{Q10}} factor, the ratio of rates 10 K apart, expresses the same information over a narrow range: Q<sub>10</sub> = exp[(E<sub>a</sub>/R)(1/T &minus; 1/(T + 10))]. The rule of thumb that rates double every 10 &deg;C near room temperature corresponds to E<sub>a</sub> &asymp; 53 kJ/mol.

~~~figure
id = "arrhenius"
file = "appc-arrhenius.gif"
alt = "Logarithmic plot of rate constant relative to its value at 190 degrees against temperature, for activation energies 60, 100 and 150 kJ per mol; steeper lines for higher activation energy."
caption = "Arrhenius sensitivity across the roasting range. Compare the slopes: a higher activation energy makes a reaction more sensitive to temperature, so changing the heating path changes the balance between reactions."
~~~

~~~example Accelerated storage
Problem. A staling marker forms with first-order rate constants of 0.437 per day at 40 &deg;C and 2.10 per day at 60 &deg;C. Find the activation energy, the rate constant and half-life at 25 &deg;C, and Q<sub>10</sub> between 25 and 35 &deg;C.

Plan. Apply {eq:ea}, then {eq:arr} in ratio form to extrapolate.

Solution. E<sub>a</sub> = 8.314 &times; ln(4.805)/(1/313.15 &minus; 1/333.15) = 68.1 kJ/mol. At 25 &deg;C: k = 0.437 exp[&minus;(68,080/8.314)(1/298.15 &minus; 1/313.15)] = 0.117 per day, a half-life of ln 2/0.117 = 5.9 days. Q<sub>10</sub> between 25 and 35 &deg;C is 2.44.

Check. The extrapolation assumes the same mechanism at all three temperatures, which accelerated tests cannot verify on their own (Chapter 5, Section 8).
~~~

~~~widget arrhenius
~~~

At roasting temperatures the spread between reactions matters more than any single activation energy. From 190 to 200 &deg;C, a reaction with E<sub>a</sub> = 60 kJ/mol speeds up by a factor of 1.39, one with 100 kJ/mol by 1.73, and one with 150 kJ/mol by 2.28. Reactions with higher activation energies gain more from heat, so the temperature path decides which products dominate.

~~~quiz
title = "Check your understanding: Sections C.1 to C.3"
[[q]]
type = "numeric"
prompt = "A first-order process has a half-life of 12 days. What fraction remains after 30 days?"
answer = 0.177
tolerance = 0.002
unit = ""
explain = "k = ln 2/12 = 0.0578 per day; e<sup>&minus;0.0578 &times; 30</sup> = 0.177."

[[q]]
type = "numeric"
prompt = "A rate constant doubles between 180 and 190 &deg;C. What is the activation energy in kJ/mol?"
answer = 121
tolerance = 1
unit = "kJ/mol"
explain = "8.314 ln 2/(1/453.15 &minus; 1/463.15) = 121 kJ/mol."
~~~

## C.4 Diffusion

{{Diffusion}} is the net movement of molecules from high to low concentration caused by their random thermal motion. {{Fick's first law}} states that the flux of molecules is proportional to the concentration gradient:

~~~eq fick
formula = "J = &minus;D dc/dx"
where = [["J", "diffusive flux: amount crossing unit area per unit time", "mol/(m<sup>2</sup> s)"], ["D", "diffusivity", "m<sup>2</sup>/s"], ["dc/dx", "concentration gradient", "mol/m<sup>4</sup>"]]
~~~

The minus sign says that molecules move down the gradient. Combining Fick's first law with conservation of mass gives Fick's second law, &part;c/&part;t = D &part;<sup>2</sup>c/&part;x<sup>2</sup>, which describes how a concentration profile changes over time.

A useful result needs no solution at all. A molecule moving randomly travels a net distance that grows with the square root of time, not in proportion to it. The {{diffusion length}} in time t is of order

~~~eq dlen
formula = "L &asymp; &radic;(D t)"
where = [["L", "typical distance diffused", "m"], ["t", "time", "s"]]
~~~

With D = 2 &times; 10<sup>&minus;10</sup> m<sup>2</sup>/s, typical for small molecules in a wet plant tissue, three minutes gives &radic;(2 &times; 10<sup>&minus;10</sup> &times; 180) = 1.9 &times; 10<sup>&minus;4</sup> m, about 190 &micro;m. Particles much larger than this cannot be emptied in a three-minute brew. Turning {eq:dlen} around, the time to diffuse a distance a is of order a<sup>2</sup>/D: doubling the distance quadruples the time.

~~~deeper The solution for a sphere
In a sphere of radius a with uniform initial concentration and its surface held at zero, Fick's second law in spherical coordinates is &part;c/&part;t = D (1/r<sup>2</sup>) &part;/&part;r(r<sup>2</sup> &part;c/&part;r). Substituting u = rc turns it into the one-dimensional equation &part;u/&part;t = D &part;<sup>2</sup>u/&part;r<sup>2</sup>, with u = 0 at r = 0 and r = a. Separating variables gives modes sin(n&pi;r/a) that decay as exp(&minus;n<sup>2</sup>&pi;<sup>2</sup>Dt/a<sup>2</sup>). Matching the initial condition and integrating over the volume gives the fraction still inside:

M(t)/M<sub>0</sub> = (6/&pi;<sup>2</sup>) &Sigma;<sub>n=1</sub><sup>&infin;</sup> (1/n<sup>2</sup>) exp(&minus;n<sup>2</sup>&pi;<sup>2</sup>Dt/a<sup>2</sup>).

At t = 0 the sum equals &pi;<sup>2</sup>/6, so the fraction is 1. The n = 1 term has time constant a<sup>2</sup>/(&pi;<sup>2</sup>D), Equation 2A.7; the higher terms vanish quickly.
~~~

## C.5 Viscosity

{{Viscosity}} is a fluid's resistance to flowing. Water's viscosity falls steeply with temperature: about 1.00 mPa s at 20 &deg;C, 0.89 at 25 &deg;C, 0.47 at 60 &deg;C and 0.30 near 93 &deg;C. Darcy's law (Chapter 2B) makes flow through a coffee bed inversely proportional to viscosity, so at fixed permeability water near 93 &deg;C flows about three times faster than at room temperature.

## C.6 Heat transfer

Heat moves by three routes, each with its own law.

{{Conduction}} carries heat through a material: q = &minus;&lambda; dT/dx, where &lambda; is the thermal conductivity in W/(m K). It has the same form as Fick's first law.

{{Convection}} carries heat between a surface and a moving fluid: q = h(T<sub>&infin;</sub> &minus; T<sub>s</sub>), where h is the {{heat-transfer coefficient}}, T<sub>&infin;</sub> the fluid temperature and T<sub>s</sub> the surface temperature. Forced flow of hot air gives h from tens to a few hundred W/(m<sup>2</sup> K).

{{Radiation}} carries heat as infrared light: q = &epsilon;&sigma;(T<sub>surr</sub><sup>4</sup> &minus; T<sub>s</sub><sup>4</sup>), with &sigma; = 5.67 &times; 10<sup>&minus;8</sup> W/(m<sup>2</sup> K<sup>4</sup>), temperatures in kelvin and &epsilon; the emissivity (Chapter 7).

### Lumped bodies and the Biot number

If heat spreads through the inside of a body much faster than it arrives at the surface, the body's temperature stays nearly uniform. An energy balance on the whole body is then enough: m c<sub>p</sub> dT/dt = hA(T<sub>&infin;</sub> &minus; T). This is a first-order equation like {eq:first}, with solution

~~~eq lumped
formula = "T<sub>&infin;</sub> &minus; T = (T<sub>&infin;</sub> &minus; T<sub>0</sub>) e<sup>&minus;t/&tau;<sub>h</sub></sup>, with &tau;<sub>h</sub> = m c<sub>p</sub> / (hA)"
where = [["&tau;<sub>h</sub>", "thermal time constant", "s"], ["m", "mass of the body", "kg"], ["c<sub>p</sub>", "specific heat capacity", "J/(kg K)"], ["A", "surface area", "m<sup>2</sup>"]]
~~~

The Biot number, Bi = hL<sub>c</sub>/&lambda; with L<sub>c</sub> = volume/area (a third of the radius for a sphere), decides whether this {{lumped model}} is valid (Equation 7.3). For Bi below about 0.1 it is accurate; larger values mean the surface runs ahead of the centre. The time for heat to reach the centre is of order r<sup>2</sup>/&alpha;, where &alpha; = &lambda;/(&rho;c<sub>p</sub>) is the thermal diffusivity, the heat-flow counterpart of D in {eq:dlen}.

~~~example Is a coffee bean lumped?
Problem. Model a green bean as a sphere of volume 0.125 cm<sup>3</sup> with &lambda; = 0.15 W/(m K), density 1,200 kg/m<sup>3</sup> and c<sub>p</sub> = 1.7 kJ/(kg K), heated by air with h = 50 W/(m<sup>2</sup> K). Find the Biot number and the internal heating time.

Plan. Find the radius from the volume, then L<sub>c</sub> = r/3, Bi and r<sup>2</sup>/&alpha;.

Solution. r = (3V/4&pi;)<sup>1/3</sup> = 3.1 mm, so L<sub>c</sub> = 1.03 mm and Bi = 50 &times; 0.00103/0.15 = 0.34. The thermal diffusivity is &alpha; = 0.15/(1,200 &times; 1,700) = 7.4 &times; 10<sup>&minus;8</sup> m<sup>2</sup>/s, so r<sup>2</sup>/&alpha; &asymp; 130 s.

Check. Bi exceeds 0.1, so the bean is not lumped: during a fast roast the centre can lag the surface by tens of degrees for a minute or more. Every input is uncertain by up to a factor of two and the bean is not a sphere, so the existence of the lag is reliable while its size is not. Chapter 7 compares this estimate with measurements.
~~~

A batch of beans in a drum is a different system. Heat reaches the pile through the drum wall and the air, the beans mix, and a lumped balance for the whole batch (Chapter 7) describes the average bean temperature with an effective hA per kilogram that bundles all three routes.

## C.7 Gases

The {{ideal gas law}} gives the volume of a known amount of gas:

~~~eq gas
formula = "pV = nRT"
where = [["p", "pressure", "Pa"], ["V", "volume", "m<sup>3</sup>"], ["n", "amount of gas", "mol"]]
~~~

{{Henry's law}} gives how much gas dissolves in a liquid:

~~~eq henry
formula = "c = k<sub>H</sub> p"
where = [["c", "dissolved concentration", "mol/L"], ["k<sub>H</sub>", "Henry's law constant", "mol/(L atm)"], ["p", "partial pressure of the gas above the liquid", "atm"]]
~~~

For CO<sub>2</sub> in water, k<sub>H</sub> is about 0.034 mol/(L atm) at 25 &deg;C and several times smaller near 90 &deg;C. Hot brewing water therefore holds little CO<sub>2</sub>, and gas released from fresh grounds forms bubbles: the bloom (Chapter 5).

~~~example Gas from an espresso dose
Problem. How much CO<sub>2</sub>, as a volume at 20 &deg;C and 1 atm, is released by 18 g of coffee containing 4 mg/g?

Plan. Convert mass to moles, then apply {eq:gas}.

Solution. The amount is 72 mg/44.01 g/mol = 1.64 &times; 10<sup>&minus;3</sup> mol, so the volume is V = nRT/p = 1.64 &times; 10<sup>&minus;3</sup> &times; 8.314 &times; 293.15/101,325 = 3.9 &times; 10<sup>&minus;5</sup> m<sup>3</sup>, or 39 mL.

Check. That is roughly the volume of the shot itself, which is why fresh espresso foams so heavily.
~~~

~~~keyterms
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "C.1"
prompt = "Derive the first-order half-life t<sub>1/2</sub> = ln 2/k from Equation C.2."
solution = "Set [A] = [A]<sub>0</sub>/2: 1/2 = e<sup>&minus;kt<sub>1/2</sub></sup>. Take natural logarithms: &minus;ln 2 = &minus;kt<sub>1/2</sub>, so t<sub>1/2</sub> = ln 2/k."

[[p]]
tier = "A"
section = "C.3"
prompt = "Convert 190 &deg;C and 225 &deg;C to kelvin and compute 1/T for each."
solution = "463.15 K and 498.15 K; 1/T = 2.159 &times; 10<sup>&minus;3</sup> and 2.007 &times; 10<sup>&minus;3</sup> K<sup>&minus;1</sup>."

[[p]]
tier = "A"
section = "C.4"
prompt = "Estimate how far a solute with D = 5 &times; 10<sup>&minus;10</sup> m<sup>2</sup>/s diffuses in 25 s, the length of an espresso shot."
solution = "&radic;(5 &times; 10<sup>&minus;10</sup> &times; 25) = 1.1 &times; 10<sup>&minus;4</sup> m, about 110 &micro;m."

[[p]]
tier = "B"
section = "C.2"
prompt = "For A &rarr; B &rarr; C with k<sub>1</sub> = 0.5 and k<sub>2</sub> = 0.2 per minute, find t<sub>max</sub> and [B]<sub>max</sub>/[A]<sub>0</sub>."
solution = "t<sub>max</sub> = ln 2.5/0.3 = 3.05 min. [B]<sub>max</sub>/[A]<sub>0</sub> = 0.4<sup>0.2/0.3</sup> = 0.4<sup>0.667</sup> = 0.543."

[[p]]
tier = "B"
section = "C.3"
prompt = "Two reactions have activation energies of 80 and 140 kJ/mol and equal rate constants at 200 &deg;C. What is the ratio of their rate constants at 220 &deg;C?"
solution = "The ratio is exp[((140,000 &minus; 80,000)/8.314)(1/473.15 &minus; 1/493.15)] = exp(7,217 &times; 8.571 &times; 10<sup>&minus;5</sup>) = exp(0.619) = 1.86. The 140 kJ/mol reaction is 1.86 times faster at 220 &deg;C."

[[p]]
tier = "B"
section = "C.3"
prompt = "What activation energy corresponds to Q<sub>10</sub> = 2 between 25 and 35 &deg;C?"
solution = "E<sub>a</sub> = R ln 2/(1/298.15 &minus; 1/308.15) = 5.763/1.088 &times; 10<sup>&minus;4</sup> = 53 kJ/mol."

[[p]]
tier = "B"
section = "C.4"
prompt = "Show that the sphere series gives a fraction remaining of 1 at t = 0, using &Sigma; 1/n<sup>2</sup> = &pi;<sup>2</sup>/6."
solution = "At t = 0 every exponential equals 1, so the fraction is (6/&pi;<sup>2</sup>)(&pi;<sup>2</sup>/6) = 1."

[[p]]
tier = "B"
section = "C.6"
prompt = "Compute the lumped time constant m c<sub>p</sub>/(hA) for the bean in the worked example (mass 0.15 g, c<sub>p</sub> 1.7 kJ/(kg K), r = 3.1 mm, h = 50 W/(m<sup>2</sup> K)). Why is a real drum roast far slower?"
solution = "A = 4&pi;r<sup>2</sup> = 1.21 &times; 10<sup>&minus;4</sup> m<sup>2</sup>; m c<sub>p</sub> = 0.255 J/K; hA = 6.0 &times; 10<sup>&minus;3</sup> W/K; time constant 42 s. A drum roast takes minutes because the roaster's heat supply is limited and shared by the batch, most beans sit inside a pile rather than in free air, and evaporating water absorbs much of the early heat."

[[p]]
tier = "B"
section = "C.7"
prompt = "How many moles of CO<sub>2</sub> dissolve in 1 L of water at 25 &deg;C in equilibrium with air containing 0.042% CO<sub>2</sub>?"
solution = "Partial pressure = 4.2 &times; 10<sup>&minus;4</sup> atm. c = 0.034 &times; 4.2 &times; 10<sup>&minus;4</sup> = 1.4 &times; 10<sup>&minus;5</sup> mol/L, about 0.6 mg/L."

[[p]]
tier = "C"
section = "C.3"
prompt = "A storage study measured a staling rate at 40 and 60 &deg;C and extrapolated to 20 &deg;C. List two ways the extrapolation could fail and an experiment to check it."
solution = "The mechanism may change with temperature: oil migration or softening of the bean matrix may matter only at higher temperatures. More than one process may contribute, each with a different activation energy, so the Arrhenius plot curves. Check by adding storage at 30 &deg;C and at 20 &deg;C (the latter over a longer period) and testing whether ln k against 1/T is a straight line."
~~~

## Further reading

Atkins P., de Paula J., *Physical Chemistry*, chapters on kinetics and transport; Crank J., *The Mathematics of Diffusion*, for the sphere solution; Incropera F.P., DeWitt D.P., *Fundamentals of Heat and Mass Transfer*, for the Biot number and lumped analysis.
