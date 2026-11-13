+++
number = 8
title = "Roasting II: chemistry"
question = "What reactions does heat drive in the bean, and how do they explain the cup?"
objectives = [
  "Recognise the functional groups involved in roasting chemistry and the reaction types they undergo.",
  "Outline the steps of the Maillard reaction, the Strecker degradation and the formation of chlorogenic acid lactones.",
  "Trace roast acids, bitter compounds, key aroma compounds, melanoidins and contaminants to their precursors.",
  "Apply first-order and consecutive kinetics along a rising temperature.",
  "Predict how species and roast profile change the products at equal roast colour.",
  "Explain a cup attribute chemically from roast data.",
]
prerequisites = ["Chapter 6, Section 1 (composition)", "Chapter 7", "Appendix B (organic chemistry)", "Appendix C, Sections C.1 to C.3 (rate laws, consecutive reactions, Arrhenius)"]
+++
Titrate brews made from samples pulled at intervals through a single roast, and something unexpected appears. Titratable acidity rises through the early roast, peaks around first crack, and falls back by second crack. Roasting both makes acid and destroys it. Which reactions do each, and why does the balance between them depend on how fast the roast ran?

This chapter answers by following the main classes of compound that matter in the cup (acids, bitter compounds, aroma, colour) back to the green-bean precursors described in Chapter 6. Section 1 sets out the chemical vocabulary. Section 2 shows how to apply rate laws to a roast whose temperature keeps rising. Sections 3 to 10 take the product classes in turn. Section 11 puts them together to explain why profile changes flavour at equal colour.

## 1. The chemical vocabulary of roasting

Roasting chemistry is organic chemistry: the chemistry of carbon compounds. A few {{functional group|functional groups}}, small arrangements of atoms that react in characteristic ways, account for most of it. Appendix B treats each in detail; this section introduces what the chapter needs.

- A {{carbonyl group}} is a carbon atom double-bonded to an oxygen, C=O. Oxygen pulls electrons toward itself, leaving the carbon slightly positive and open to attack. Sugars in their open-chain form contain a carbonyl group.
- An {{amino group}}, &ndash;NH<sub>2</sub>, carries a pair of electrons that it can share. Amino acids and proteins contain amino groups.
- A {{nucleophile}} is a molecule or group that donates an electron pair to form a new bond with an electron-poor atom. The amino group is a nucleophile, and the carbonyl carbon is its natural target.
- A {{carboxylic acid}}, &ndash;COOH, is the acidic group of organic acids such as acetic and citric acid.
- An {{ester}} forms when a carboxylic acid joins an alcohol (&ndash;OH) and releases water. A {{lactone}} is an ester in which both groups belong to the same molecule, so the product is a ring.

Two reaction types recur. In {{decarboxylation}}, a carboxylic acid loses carbon dioxide; it is one source of the gas in roasted coffee (Chapter 5). In {{fragmentation}}, a molecule breaks into smaller pieces, as sugars do when heated.

## 2. Kinetics along a rising temperature

Roasting runs hundreds of coupled reactions at once, at temperatures that change every second. We need a way to apply rate laws to it.

At constant temperature, a {{first-order reaction}} removes a fixed fraction of the remaining reactant per unit time, so its concentration C falls as C = C<sub>0</sub>e<sup>&minus;kt</sup> (Appendix C.1). The rate constant k depends on temperature through the Arrhenius equation (Appendix C.3). In a roast the temperature rises, so k changes with time. For a linear temperature ramp at heating rate &gamma; (&deg;C per minute), each small time interval dt corresponds to a temperature interval dT = &gamma; dt. Substituting into the first-order rate law, &minus;dC/C = k dt = k(T) dT/&gamma;, and integrating from the starting to the final temperature gives

~~~eq ramp
formula = "ln(C / C<sub>0</sub>) = &minus;(1/&gamma;) &int; k(T) dT"
where = [["C, C<sub>0</sub>", "final and initial concentrations", ""], ["&gamma;", "heating rate", "&deg;C/min"], ["k(T)", "rate constant at temperature T", "1/min"]]
~~~

The integral depends only on the temperature range. The heating rate divides it. Halving the heating rate doubles the extent of every first-order reaction between the same two temperatures.

~~~example Chlorogenic acid loss on two ramps
Problem. A first-order loss has k = 0.1 per minute at 200 &deg;C and an activation energy of 100 kJ/mol (illustrative values). What fraction of the starting material remains after a ramp from 150 to 220 &deg;C at 10 &deg;C/min, and at 5 &deg;C/min?

Plan. Evaluate the integral in {eq:ramp} numerically, then divide by each heating rate.

Solution. Integrating k(T) from 150 to 220 &deg;C gives 5.18 (with k in 1/min and T in &deg;C, so the result is in &deg;C/min). At 10 &deg;C/min, ln(C/C<sub>0</sub>) = &minus;0.518, so 59.5% remains. At 5 &deg;C/min, ln(C/C<sub>0</sub>) = &minus;1.037, so 35.5% remains.

Check. Both roasts end at 220 &deg;C, but the slower one has destroyed almost 40% more of the starting material. Perrone and colleagues fitted first-order Arrhenius kinetics to individual chlorogenic acids during roasting and obtained models that fitted light to moderately dark roasts closely (correlation 0.98) [1].
~~~

Many roast compounds are first formed and then destroyed. The simplest model is a pair of {{consecutive reactions}}, A &rarr; B &rarr; C, in which an intermediate B builds up and then disappears (Appendix C.2). When formation and destruction have different activation energies, the whole temperature path decides how much B is present at the end.

~~~widget kinetics
~~~

## 3. Sugars: caramelisation and the Maillard reaction

Sucrose, the main sugar of green coffee, splits into glucose and fructose and fragments as the bean heats. Without nitrogen present, sugars lose water and fragment into furans (ring compounds containing oxygen, such as furfural), small reactive compounds with two carbonyl groups such as diacetyl and methylglyoxal, and small acids. Breakdown of sugars without nitrogen is called {{caramelisation}}.

When amino groups are present, sugars enter a far larger network, the {{Maillard reaction}}, which produces most of coffee's colour and much of its aroma. It proceeds in three stages.

1. Condensation. The amino group of an amino acid attacks the carbonyl carbon of a reducing sugar (Chapter 6). The product loses water to form an {{imine}}, a compound with a carbon-nitrogen double bond, which closes into a ring.
2. Rearrangement. The ring opens and rearranges to a more stable product. From an aldose (a sugar whose carbonyl is at the end of the chain, such as glucose) the result is an {{Amadori product}}; from a ketose (carbonyl inside the chain, such as fructose) it is a {{Heyns product}}. These intermediates are stable enough to accumulate early in a roast.
3. Breakdown. The Amadori and Heyns products lose water and fragment into furans, pyrroles and especially {{dicarbonyl|dicarbonyls}}, compounds with two carbonyl groups. The dicarbonyls drive the next stage.

## 4. Strecker degradation and pyrazines

In the {{Strecker degradation}}, a dicarbonyl reacts with a free amino acid. The amino acid loses carbon dioxide and becomes an aldehyde with one fewer carbon atom, while the dicarbonyl becomes an aminoketone (a compound carrying both an amino group and a carbonyl). Each amino acid gives a characteristic aldehyde:

| Amino acid | Strecker aldehyde | Odour |
|---|---|---|
| Methionine | methional | cooked potato |
| Leucine | 3-methylbutanal | malty |
| Isoleucine | 2-methylbutanal | malty, chocolate |
| Valine | 2-methylpropanal | malty |
| Phenylalanine | phenylacetaldehyde | honey, floral |

Two aminoketones then join and are oxidised to a {{pyrazine}}, a six-membered ring containing two nitrogen atoms. Pyrazines with small carbon side chains give coffee's roasty, nutty and earthy notes; which pyrazines form depends on which dicarbonyls and amino acids were available [2].

~~~example Carbon dioxide from the Strecker degradation
Problem. How much CO<sub>2</sub> does 1 g of methionine (149.21 g/mol) release on complete Strecker degradation?

Plan. One mole of CO<sub>2</sub> (44.01 g) forms per mole of amino acid.

Solution. 1/149.21 = 6.70 &times; 10<sup>&minus;3</sup> mol, giving 6.70 &times; 10<sup>&minus;3</sup> &times; 44.01 = 0.295 g of CO<sub>2</sub>.

Check. Free amino acids are a small fraction of green coffee, so the Strecker degradation is a minor source of CO<sub>2</sub> compared with sugar and chlorogenic acid breakdown. Its importance is the aroma it makes.
~~~

~~~quiz
title = "Check your understanding: Sections 1 to 4"
[[q]]
type = "choice"
prompt = "Which reaction converts an amino acid into an aldehyde with one fewer carbon atom and releases CO<sub>2</sub>?"
options = ["Amadori rearrangement", "Strecker degradation", "Lactone formation"]
answer = 1
explain = "In the Strecker degradation the amino acid loses CO<sub>2</sub> and becomes an aldehyde."

[[q]]
type = "numeric"
prompt = "A first-order roast reaction leaves 60% of a compound on a 10 &deg;C/min ramp. What fraction remains on a 5 &deg;C/min ramp between the same temperatures?"
answer = 0.36
tolerance = 0.005
unit = ""
explain = "Halving the heating rate doubles the integral: 0.60<sup>2</sup> = 0.36."
~~~

## 5. Acids: made and destroyed

We can now explain the opening observation. Roasting both creates and destroys acids.

Formation from sucrose. Ginz, Balzer, Bradbury and Maier (2000) showed that formic, acetic, glycolic and lactic acids, which account for much of the acidity created in roasting, form mainly from sucrose [3]. Green beans spiked with sucrose, glucose or fructose before roasting produced more of these acids. Arabinose, erythrose and 1,6-anhydroglucose, which are themselves sucrose breakdown products, were also precursors. Arabica's higher sucrose content (Chapter 6) is a direct reason that it develops more roast acidity than robusta.

Destruction. Citric and malic acids, the main organic acids of green coffee, break down during roasting. Quinic acid is released as chlorogenic acids break apart [4].

The net result follows the shape of a formation-and-destruction sequence. In a drum-roaster study, titratable acidity rose to a peak around first crack and fell back to its starting level by second crack, and the pattern held across roast profiles [5].

~~~misconception
"Roasting steadily destroys a coffee's acidity, so lighter is always more acidic." Titratable acidity rises during the first part of a roast as sucrose fragments into new acids, and falls only later as acids are destroyed or evaporate. A very light roast stopped before the peak can have less titratable acid than a slightly darker one.
~~~

~~~example How much acetic acid could sucrose make?
Problem. Find an upper limit on the acetic acid that 100 g of green arabica (11% moisture, 7.3% sucrose on a dry basis) could form.

Plan. Count carbon atoms. Each sucrose molecule, C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> (342.3 g/mol), contains 12 carbons; each acetic acid (60.05 g/mol) contains two.

Solution. At most six acetic acids per sucrose: 6 &times; 60.05/342.3 = 1.05 g of acetic acid per gram of sucrose. The sample holds 100 &times; 0.89 &times; 0.073 = 6.5 g of sucrose, so at most 6.8 g of acetic acid.

Check. Measured acetic acid in roasted coffee is far below this. Most sucrose carbon goes to other products (furans, other acids, CO<sub>2</sub>, melanoidins), and the acids formed are partly destroyed or evaporate.
~~~

## 6. Chlorogenic acids, lactones and bitterness

{{Chlorogenic acids}} are esters formed from quinic acid and a hydroxycinnamic acid such as caffeic acid; 5-caffeoylquinic acid is the most abundant. During roasting they change in three ways. They isomerise, as the caffeoyl group moves between positions on the quinic acid ring. They {{hydrolysis|hydrolyse}}, splitting with water back into quinic and caffeic acids. And they lose water internally to form lactones called caffeoylquinides, which are bitter.

In light and medium roasts the lactones are the main bitter compounds. In dark roasts they break down further, through caffeic acid and compounds with two neighbouring OH groups on a benzene ring, called {{catechol|catechols}}, to {{phenylindane|phenylindanes}}, whose bitterness is harsher and lingers longer [6]. Chapter 1 described the receptor-level picture: several roast products activate the bitter receptors TAS2R43 and TAS2R46 at far lower concentrations than caffeine.

Chlorogenic acid breakdown also yields smoky, spicy phenolic volatiles such as guaiacol and 4-vinylguaiacol [2], and catechols that later oxidise to the quinones that trap thiols in the brew (Chapter 5).

## 7. Trigonelline, caffeine and nitrogen-containing aroma

Trigonelline breaks down substantially. Schouten and colleagues found that it fell by about 60% in arabica and 40% in robusta at dark roast, while nicotinic acid (vitamin B3), one of its products, rose by only about 10% and 40% respectively [7]. Most of the trigonelline lost therefore became other nitrogen-containing products, including pyridines and pyrroles. Caffeine is largely stable through roasting [8].

## 8. Sulfur aroma: 2-furfurylthiol

{{2-Furfurylthiol}}, a sulfur compound with a roasty, coffee-like odour and a threshold near 0.01 &micro;g/L, is among the most important odorants in fresh coffee (Chapter 1). Cerny and colleagues used molecules labelled with deuterium (heavy hydrogen) to trace its origin [9]. It forms when furfuryl alcohol loses water to become a reactive furfuryl cation, which then reacts with the sulfur of cysteine bound in protein. Furfural did not contribute. Furfuryl alcohol itself comes from sugar breakdown, linking this key odorant to the carbohydrate chemistry of Section 3.

## 9. Melanoidins

{{Melanoidins}} are the brown, high-molecular-weight, nitrogen-containing polymers formed in the late stages of the Maillard reaction. In coffee they incorporate polysaccharides from the cell walls, proteins and chlorogenic acids [10]. Their exact structures and formation mechanisms remain unresolved, partly because they are mixtures that cannot be crystallised or cleanly analysed. They contribute colour, body and astringency (Chapter 1), bind some aroma compounds and metal ions, and act as antioxidants.

## 10. Process contaminants

Some roast products are undesirable. {{Acrylamide}} forms from the amino acid asparagine and reducing sugars by a Maillard route. In Schouten and colleagues' roasting series it peaked at light-medium roast (about 730 &micro;g/kg in arabica and 1,130 &micro;g/kg in robusta) and then fell by 85 to 88% by dark roast [7]: another formation-and-destruction pattern. The EU sets a benchmark level, a reference value that producers are expected to work below, of 400 &micro;g/kg for roasted coffee [11]. Other roasting contaminants include furan and methylfurans, dicarbonyls, 4-methylimidazole and chloropropanols [12]. Ochratoxin A is partly destroyed by roasting (Chapter 6).

## 11. Competing pathways and roast profile

We can now say why profile changes flavour at equal colour. Each reaction has its own activation energy, and reactions with higher activation energies speed up more as temperature rises (Appendix C.3). The path a roast takes therefore shifts the balance between products at the same end point. The calculator below models two competing systems along a linear ramp to a fixed drop temperature: a precursor forms acids, which a second reaction destroys, while chlorogenic acid forms bitter lactones by a third.

~~~widget parallel
~~~

~~~figure
id = "pathways"
file = "ch08-pathways.gif"
alt = "Relative amounts of acids and lactones against bean temperature for two heating rates; the faster ramp ends with more acid and fewer lactones at the same drop temperature."
caption = "Teaching model of competing pathways at 6 and 15 &deg;C/min to the same drop temperature (illustrative rate constants and activation energies). Follow each pair of curves to the drop temperature and compare where they end."
~~~

With the default values, a ramp at 6 &deg;C/min ends with an acid-to-lactone ratio of 0.57; a ramp at 15 &deg;C/min ends at 2.03, as {fig:pathways} shows. The slow ramp spends longer at the temperatures where acids are destroyed and lactones form. The direction agrees with the development-time results of Chapter 7, where shorter development was associated with fruity and acidic attributes and longer development with roasty and bitter ones [13]. The numbers depend entirely on the assumed constants.

~~~misconception
"Two roasts with the same colour have the same chemistry." Colour reflects the extent of browning, one family of reactions among many. Because reactions with different activation energies respond differently to the temperature path, two roasts can match in colour and differ in acids, lactones, aroma and structure (Chapter 7, Section 7).
~~~

Online measurements of roaster exhaust show aroma-related signals rising sharply after the onset of heat-releasing reactions, with heavier volatiles appearing later [14]. The exhaust carries a real-time record of the reaction network this chapter describes.

## 12. Back to the cup

To explain an attribute chemically, work backward through the book. Identify the class of compound likely to be responsible (Chapter 1). Identify its precursors and the reactions that make and destroy it (this chapter). Ask how the roast path shifted those reactions (Chapter 7), and how brewing extracted the products and water modified them (Chapters 2 and 3). Then test the explanation against measurements: titratable acidity, colour, mass loss, degassing and blind tasting.

## Looking ahead

Every chapter has ended with claims of three kinds: well supported, resting on one study, and untested. Chapter 9 shows how to test one yourself: how to design an experiment that changes one thing in the roaster and shows, with numbers and a blind panel, what it did to the cup.

## Key equations

~~~equations
[[eq]]
name = "First-order loss on a ramp"
formula = "ln(C/C<sub>0</sub>) = &minus;(1/&gamma;) &int; k(T) dT"
note = "Equation 8.1."
[[eq]]
name = "Arrhenius"
formula = "k(T) = k<sub>ref</sub> exp[&minus;(E<sub>a</sub>/R)(1/T &minus; 1/T<sub>ref</sub>)]"
note = "Appendix C.3."
[[eq]]
name = "Strecker stoichiometry"
formula = "amino acid + dicarbonyl &rarr; aldehyde (one carbon fewer) + CO<sub>2</sub> + aminoketone"
note = "One CO<sub>2</sub> per amino acid."
[[eq]]
name = "Acetic acid upper bound"
formula = "6 &times; 60.05 / 342.3 = 1.05 g per g of sucrose"
note = "Carbon limit."
~~~

~~~keyterms
~~~

~~~quiz
title = "Chapter review"
[[q]]
type = "choice"
prompt = "Which green-coffee component is the main precursor of the formic, acetic, glycolic and lactic acids formed in roasting?"
options = ["Sucrose", "Chlorogenic acids", "Trigonelline"]
answer = 0
explain = "Ginz et al. showed these acids form mainly from sucrose."

[[q]]
type = "choice"
prompt = "Acrylamide in Schouten et al.'s roasting series"
options = ["rose steadily with roast degree", "peaked at light-medium roast and fell sharply by dark roast", "was absent"]
answer = 1
explain = "It peaked at light-medium roast and fell by 85 to 88% by dark roast."

[[q]]
type = "choice"
prompt = "Why does a faster ramp to the same drop temperature keep more acid relative to lactones in the teaching model?"
options = ["It spends less time at temperatures where acids are destroyed and lactones form", "It reaches a higher final temperature", "It contains more sucrose"]
answer = 0
explain = "Every first-order reaction proceeds less far when the heating rate is higher (Equation 8.1)."
~~~

## Labs

~~~lab
id = "L8.1"
title = "Titratable acidity across a roast"
aim = "Measure titratable acidity at five roast degrees and locate its peak."
safety = ["Sampling from a running roaster needs a sample trier and heat-resistant gloves; follow the safety rules of Chapter 7."]
materials = ["Roaster with a sample trier", "pH meter and 0.0200 mol/L NaOH", "Grinder, brewer and refractometer"]
method = [
  "During one roast, pull about 15 g samples at the end of drying, mid-browning, first crack, 90 s after first crack, and near second crack.",
  "Rest the samples for 24 h, then brew each at the same ratio, adjusting grind to similar extraction yields.",
  "Titrate 50.0 mL of each brew to pH 8.2.",
]
columns = ["Sample", "Bean temp (&deg;C)", "Mass loss (%)", "EY (%)", "TDS (%)", "TA (mmol/L)", "TA per TDS"]
rows = 5
analysis = ["Plot titratable acidity per unit TDS against bean temperature. Where is the peak?", "Compare with the formation-and-destruction model of Section 2."]
expected = "A peak around first crack, falling toward second crack."
~~~

~~~lab
id = "L8.2"
title = "Colour and mass loss through a roast"
aim = "Follow browning and mass loss through a roast and fit a simple kinetic curve."
materials = ["Roaster with sample trier", "Colour meter or a standardised photographic setup", "Balance"]
method = ["Sample at one-minute intervals after the turning point; weigh each sample and measure its ground colour after cooling."]
columns = ["Time (min)", "Bean temp (&deg;C)", "Colour", "Mass loss (%)"]
rows = 12
analysis = ["Plot colour against time and against bean temperature. Is colour a function of temperature alone?"]
~~~

~~~lab
id = "L8.3"
title = "Chlorogenic acids by UV-visible spectroscopy"
aim = "Estimate total chlorogenic acids across roast degrees from absorbance near 325 nm."
safety = ["If methanol is used it is toxic and flammable: work in a fume cupboard, or use water-only extraction."]
materials = ["UV-visible spectrophotometer with quartz or UV-grade cuvettes", "5-caffeoylquinic acid standard (if available)", "Samples from L8.1"]
method = ["Extract 0.5 g of finely ground coffee in 50 mL of hot water, filter, and dilute 1:50.", "Measure absorbance at 325 nm and build a calibration from the standard (Appendix D.5)."]
columns = ["Sample", "Absorbance", "CGA (mg/g, as 5-CQA)"]
rows = 6
analysis = ["Plot chlorogenic acid content against mass loss. Is the decline consistent with first-order kinetics?", "Melanoidins also absorb near 325 nm. How does this bias the dark roasts?"]
~~~

~~~lab
id = "L8.4"
title = "Sorting a roast series by aroma"
aim = "Use a panel to group roast samples by aroma and relate the groups to the chemistry."
materials = ["Ground samples from L8.1 in coded, lidded cups"]
method = ["Each panellist sniffs every sample and sorts them into groups by similarity, naming each group."]
columns = ["Panellist", "Group 1", "Group 2", "Group 3", "Descriptors"]
rows = 6
analysis = ["Which samples are grouped together most often? Do the descriptors match the reaction classes expected at each stage?"]
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "4"
prompt = "Name the Strecker aldehydes formed from leucine and methionine and describe their odours."
solution = "Leucine gives 3-methylbutanal (malty); methionine gives methional (cooked potato)."

[[p]]
tier = "A"
section = "3"
prompt = "What is the difference between an Amadori product and a Heyns product?"
solution = "Both are rearranged products of a sugar-amine condensation. Amadori products come from aldoses such as glucose; Heyns products come from ketoses such as fructose."

[[p]]
tier = "A"
section = "5"
prompt = "Why does arabica typically develop more roast acidity than robusta?"
solution = "Roast acids form mainly from sucrose, and arabica contains two to several times more sucrose."

[[p]]
tier = "B"
section = "2"
prompt = "A compound breaks down with first-order kinetics and an activation energy of 120 kJ/mol. By what factor does its rate constant increase from 190 to 210 &deg;C?"
solution = "exp[(120,000/8.314)(1/463.15 &minus; 1/483.15)] = exp(14,433 &times; 8.94 &times; 10<sup>&minus;5</sup>) = exp(1.290) = 3.63."

[[p]]
tier = "B"
section = "2"
prompt = "A first-order reaction leaves 70% of its starting material on an 8 &deg;C/min ramp. What remains at 12 &deg;C/min between the same temperatures?"
solution = "ln(0.70) = &minus;0.357 at 8 &deg;C/min. At 12 &deg;C/min the integral is multiplied by 8/12, giving &minus;0.238, so 78.8% remains."

[[p]]
tier = "B"
section = "4"
prompt = "How much CO<sub>2</sub> do 2.0 g of leucine (131.17 g/mol) release on complete Strecker degradation?"
solution = "2.0/131.17 = 0.01525 mol; &times; 44.01 = 0.671 g."

[[p]]
tier = "B"
section = "7"
prompt = "Green arabica contains 1.0% trigonelline and 0.02% nicotinic acid (dry basis). Using Schouten et al.'s figures, how much trigonelline remains at dark roast, and what share of the lost trigonelline does the 10% rise in nicotinic acid account for?"
solution = "Remaining: 0.40%, so 0.60 percentage points are lost. Nicotinic acid rises by 0.002 points, a negligible share of the 0.60 lost (their molar masses are similar), so nearly all the trigonelline becomes other products."

[[p]]
tier = "B"
section = "10"
prompt = "A roasted coffee contains 600 &micro;g/kg of acrylamide. By how much must it fall to meet the EU benchmark, and what roasting change would reduce it?"
solution = "To 400 &micro;g/kg, a reduction of 33%. Acrylamide peaks at light-medium roast and falls with darker roasting, so a darker roast reduces it, at the cost of a change in flavour. Treating green coffee with the enzyme asparaginase is an industrial alternative."

[[p]]
tier = "B"
section = "11"
prompt = "In the parallel-pathway calculator, raise the activation energy of acid destruction from 150 to 170 kJ/mol. Predict, then check, the effect on the acid-to-lactone ratio at 6 and 15 &deg;C/min."
solution = "With the reference rate fixed at 190 &deg;C, a higher activation energy makes destruction slower below 190 &deg;C and faster above it. With the default drop at 215 &deg;C, both ramps spend time above 190 &deg;C, so more acid is destroyed late, and the slow ramp, which spends longer there, loses relatively more. With the defaults, the ratio falls from 0.57 to 0.45 at 6 &deg;C/min and from 2.03 to 1.86 at 15 &deg;C/min, so the fast ramp's advantage grows from 3.6-fold to 4.1-fold."

[[p]]
tier = "B"
section = "6"
prompt = "Why does absorbance at 325 nm overestimate chlorogenic acids in dark roasts?"
solution = "Melanoidins and other brown products absorb broadly in the near ultraviolet, adding to the 325 nm signal. Their share rises with roast degree, so a calibration with pure 5-caffeoylquinic acid overestimates chlorogenic acids in dark roasts. Separating the compounds by HPLC first (Appendix D.6) avoids this."

[[p]]
tier = "C"
section = "6"
prompt = "Explain, with mechanisms, why the same end temperature can give different bitterness in a fast and a slow roast."
solution = "Lactone formation from chlorogenic acids and lactone breakdown to phenylindanes are separate reactions with different activation energies. The slow roast spends more time at intermediate temperatures, forming more lactones and, near the end, converting more of them to harsher phenylindanes; the fast roast forms less of both. The character of the bitterness therefore depends on the path."

[[p]]
tier = "C"
section = "5"
prompt = "Design an experiment to test whether adding sucrose to green coffee raises titratable acidity in the cup. State the controls."
solution = "Soak matched green samples in sucrose solution or in plain water (a control for soaking itself), dry them back to the same moisture, roast them together to the same colour and mass loss, brew at matched extraction yield, and measure titratable acidity and sourness blind. Replicate the roasts. A rise in titratable acidity with sucrose would reproduce Ginz et al.'s result at the level of the cup."

[[p]]
tier = "C"
section = "8"
prompt = "A green coffee has unusually little cysteine. Predict the effect on 2-furfurylthiol and propose a measurement."
solution = "2-Furfurylthiol forms from furfuryl alcohol and protein-bound cysteine, so less available cysteine should give less of it. Measure it in the roasted coffee with a thiol-selective extraction and GC-MS with stable isotope dilution (Appendix D.7), against a control coffee with normal cysteine roasted to the same degree."

[[p]]
tier = "C"
section = "5"
prompt = "Titratable acidity peaks during first crack and returns to its starting level by second crack. Build a two-step kinetic explanation and predict how a slower approach to first crack would change the peak."
solution = "Acids form from sucrose in one step and are destroyed or evaporate in a second, with the second more temperature-sensitive. Titratable acidity rises while formation dominates and falls once destruction overtakes it. A slower approach allows more formation at moderate temperatures but also more destruction later. The kinetics calculator shows that a slower ramp peaks at a lower temperature; the relative heights depend on the constants, so the prediction must be tested, for example by running Lab L8.1 at two ramps."
~~~

## References

1. Perrone D., Donangelo R., Donangelo C.M., Farah A. (2010), Modeling weight loss and chlorogenic acids content in coffee during roasting, *Journal of Agricultural and Food Chemistry* 58(23).
2. Schwartzberg H., Roasting chemistry and profile roasting (lecture notes).
3. Ginz M., Balzer H.H., Bradbury A.G.W., Maier H.G. (2000), Formation of aliphatic acids by carbohydrate degradation during roasting of coffee, *European Food Research and Technology*.
4. Yeager S.E., Batali M.E., Guinard J.-X., Ristenpart W.D., Acids in coffee: a review of sensory measurements and meta-analysis of chemical composition, *Critical Reviews in Food Science and Nutrition*.
5. Anokye-Bempah L. et al. (2024), roast profiles, titratable acidity and sourness, *Scientific Reports* 14:8237.
6. Hofmann T. and colleagues (2007), bitter compounds in roasted coffee, American Chemical Society national meeting.
7. Schouten M.A. et al. (2021), acrylamide, trigonelline and nicotinic acid across roast degree, *Food Chemistry* 343:128514.
8. Bioactive compounds in Ethiopian coffees across roasting levels (tbs.scione.com, full text).
9. Cerny C. et al. (2021), formation of 2-furfurylthiol in coffee, Proceedings of the 16th Weurman Flavour Research Symposium.
10. Moreira A.S.P., Nunes F.M., Domingues M.R., Coimbra M.A. (2012), Coffee melanoidins: structures, mechanisms of formation and potential health impacts, *Food and Function*.
11. Commission Regulation (EU) 2017/2158 establishing mitigation measures and benchmark levels for the reduction of the presence of acrylamide in food.
12. Silva da Costa D. et al. (2023), Thermal contaminants in coffee induced by roasting: a review, *International Journal of Environmental Research and Public Health* 20:5586.
13. Alstrup J., Petersen M.A., Larsen F.H., Münchow M. (2020), *Beverages* 6:70.
14. Wieland F. et al. (2012), *Analytical and Bioanalytical Chemistry* 402:2531.
