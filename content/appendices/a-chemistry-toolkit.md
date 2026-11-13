+++
number = 0
label = "A"
title = "Chemistry toolkit"
question = "Which general chemistry does the rest of the book assume, and how does it apply to a cup?"
objectives = [
  "Convert between mass fraction, mass concentration, molarity, ppm and 'as CaCO<sub>3</sub>' units.",
  "Use molar masses and balanced equations to move between mass and amount of substance.",
  "Write an equilibrium constant and use K<sub>a</sub>, pK<sub>a</sub> and the fraction dissociated at a given pH.",
  "Derive and apply the Henderson-Hasselbalch equation, and explain buffering.",
  "Explain how bicarbonate in brew water neutralises coffee acids.",
  "Use a solubility product to decide whether scale will form.",
]
+++
This appendix collects the general chemistry that the chapters use, explained through coffee examples. It assumes you met moles and equations in a first chemistry course and supplies what has faded. Each section is referred to from the chapters where it is needed; read it then, or straight through as a refresher. A free general chemistry textbook such as OpenStax *Chemistry 2e* gives a fuller treatment.

## A.1 Amount of substance

Chemists count particles in {{mole|moles}}. One mole is 6.022 &times; 10<sup>23</sup> particles, a number chosen so that one mole of carbon-12 atoms weighs exactly 12 g. The {{molar mass}} M of a compound, in g/mol, is the sum of the masses of the atoms in its formula. The amount of substance follows from the mass:

~~~eq amount
formula = "n = m / M"
where = [["n", "amount of substance", "mol"], ["m", "mass", "g"], ["M", "molar mass", "g/mol"]]
~~~

The compounds used most often in this book:

| Compound | Formula | M (g/mol) | Where it appears |
|---|---|---|---|
| Caffeine | C<sub>8</sub>H<sub>10</sub>N<sub>4</sub>O<sub>2</sub> | 194.19 | Bitterness, Chapter 1 |
| 5-Caffeoylquinic acid | C<sub>16</sub>H<sub>18</sub>O<sub>9</sub> | 354.31 | Main chlorogenic acid, Chapter 8 |
| Sucrose | C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> | 342.30 | Green coffee sugar, Chapters 6 and 8 |
| Calcium carbonate | CaCO<sub>3</sub> | 100.09 | Hardness and alkalinity units, Chapter 3 |
| Bicarbonate ion | HCO<sub>3</sub><sup>&minus;</sup> | 61.02 | Alkalinity, Chapter 3 |
| Epsom salt | MgSO<sub>4</sub>&middot;7H<sub>2</sub>O | 246.47 | Water recipes, Chapter 3 |
| Sodium hydroxide | NaOH | 40.00 | Titration, Chapter 0 |

Some salts crystallise with water molecules built into their structure, called {{water of crystallisation}}. The formula and molar mass of such a {{hydrate}} include that water. Epsom salt, MgSO<sub>4</sub>&middot;7H<sub>2</sub>O, is 9.86% magnesium by mass (24.305/246.47), whereas anhydrous MgSO<sub>4</sub> is 20.2%. Using the wrong one doubles the magnesium in a water recipe.

~~~example Caffeine in a cup
Problem. A 240 mL cup contains 95 mg of caffeine. What is the caffeine concentration in mmol/L?

Plan. Convert mass to moles with {eq:amount}, then divide by volume in litres.

Solution. n = 0.095 g / 194.19 g/mol = 4.89 &times; 10<sup>&minus;4</sup> mol. Concentration = 4.89 &times; 10<sup>&minus;4</sup> mol / 0.240 L = 2.04 &times; 10<sup>&minus;3</sup> mol/L, or 2.04 mmol/L.

Check. Units: mol divided by L gives mol/L, and 1 mol/L = 1,000 mmol/L.
~~~

## A.2 Concentration units

Coffee work uses several concentration units at once, and most errors in water recipes and brew calculations are unit errors.

{{Mass fraction}} is the mass of solute divided by the mass of solution. TDS is a mass fraction quoted in percent: a brew at 1.35% TDS has 1.35 g of dissolved solids per 100 g of brew.

{{Mass concentration}} is mass of solute per volume of solution, in g/L or mg/L. For a dilute solution whose density is close to 1.00 g/mL, a mass fraction of 1.35% is about 13.5 g/L. Brewed coffee is about 1.00 to 1.01 g/mL at room temperature, so the difference is under 1%. Espresso at 10% TDS is near 1.04 g/mL, and the difference matters.

{{Molarity}}, in mol/L, counts particles per litre. It is the natural unit for reactions and equilibria, because reactions combine particles in fixed numbers.

{{Parts per million}} (ppm) is a mass ratio: 1 ppm = 1 mg of solute per kg of solution. For dilute water, 1 kg occupies very nearly 1 L, so 1 ppm &asymp; 1 mg/L.

### Equivalents and "as CaCO<sub>3</sub>"

Some quantities depend on electric charge rather than on the number of particles. An {{equivalent}} is one mole of charge, and a milliequivalent (meq) is a thousandth of that. One millimole of Ca<sup>2+</sup> carries 2 meq of positive charge; one millimole of HCO<sub>3</sub><sup>&minus;</sup> carries 1 meq of negative charge.

Water hardness and alkalinity are reported in mg/L "as CaCO<sub>3</sub>". The convention asks: how many milligrams of calcium carbonate per litre would carry the same number of equivalents? Calcium carbonate has a molar mass of 100.09 g/mol and carries 2 equivalents per mole, so one equivalent of CaCO<sub>3</sub> weighs 50.04 g. Two consequences follow.

- Hardness counts Ca<sup>2+</sup> and Mg<sup>2+</sup>, each carrying 2 meq per mmol, so each mmol/L of either is reported as 2 &times; 50.04 = 100.09 mg/L as CaCO<sub>3</sub>.
- Alkalinity counts the capacity to neutralise acid. Each mmol/L of HCO<sub>3</sub><sup>&minus;</sup> neutralises 1 mmol/L of hydrogen ions (1 meq/L), and is reported as 50.04 mg/L as CaCO<sub>3</sub>.

~~~example Hardness from a water report
Problem. A water contains Ca<sup>2+</sup> 25 mg/L and Mg<sup>2+</sup> 8.0 mg/L. What is its hardness as CaCO<sub>3</sub>?

Plan. Convert each ion to mmol/L, add, and multiply by 100.09.

Solution. Calcium: 25/40.08 = 0.624 mmol/L. Magnesium: 8.0/24.31 = 0.329 mmol/L. Total: 0.953 mmol/L, so hardness = 0.953 &times; 100.09 = 95.4 mg/L as CaCO<sub>3</sub>.

Check. Per milligram, magnesium contributes 40.08/24.31 = 1.65 times as much hardness as calcium, because it is the lighter atom: each milligram contains more atoms.
~~~

~~~quiz
title = "Check your understanding: Sections A.1 and A.2"
[[q]]
type = "numeric"
prompt = "A 30 mL espresso contains 65 mg of caffeine. What is the caffeine concentration in mmol/L?"
answer = 11.2
tolerance = 0.1
unit = "mmol/L"
explain = "0.065/194.19 = 3.35 &times; 10<sup>&minus;4</sup> mol in 0.030 L gives 11.2 mmol/L."

[[q]]
type = "numeric"
prompt = "A water contains 61 mg/L of bicarbonate. What is its alkalinity in mg/L as CaCO<sub>3</sub>?"
answer = 50.0
tolerance = 0.2
unit = "mg/L"
explain = "61/61.02 = 1.00 mmol/L = 1.00 meq/L; &times; 50.04 = 50.0."
~~~

## A.3 Balanced equations and stoichiometry

A balanced chemical equation states the ratios in which particles react, and {{stoichiometry}} is the arithmetic that follows from those ratios. Bicarbonate in brew water reacts with a coffee acid, written HA:

~~~eq neut
formula = "HA + HCO<sub>3</sub><sup>&minus;</sup> &rarr; A<sup>&minus;</sup> + CO<sub>2</sub> + H<sub>2</sub>O"
where = [["HA", "an acid with one acidic hydrogen", ""], ["A<sup>&minus;</sup>", "the acid's conjugate base", ""]]
~~~

One mole of bicarbonate removes one mole of acidic hydrogen. A water with 40 mg/L alkalinity contains 40/50.04 = 0.80 mmol/L of bicarbonate, and therefore neutralises 0.80 mmol of acid per litre before the coffee is ever tasted. Chapter 0 gives a typical titratable acidity for drip coffee of about 1.6 mmol/L, measured after brewing. Moderately alkaline water therefore neutralises an amount of acid comparable to what remains, which is the stoichiometric reason that alkalinity dominates the effect of water on acidity (Chapter 3).

## A.4 Chemical equilibrium

Many reactions do not run to completion. They reach a state of {{chemical equilibrium}} in which the forward and reverse reactions proceed at equal rates, so concentrations stop changing. For a reaction aA + bB &harr; cC + dD at equilibrium,

~~~eq k
formula = "K = [C]<sup>c</sup>[D]<sup>d</sup> / ([A]<sup>a</sup>[B]<sup>b</sup>)"
where = [["K", "equilibrium constant", "depends on the reaction"], ["[X]", "equilibrium concentration of species X", "mol/L"], ["a, b, c, d", "coefficients in the balanced equation", ""]]
~~~

Strictly, the concentrations should be replaced by {{activity|activities}}, effective concentrations that account for interactions between ions. In dilute brew water the difference is a few percent; in espresso it can be larger. K depends on temperature but not on the concentrations themselves.

The {{reaction quotient}} Q has the same form as K but uses the concentrations present at any moment. If Q is less than K, the reaction proceeds forward; if Q is greater, it proceeds backward. {{Le Chatelier's principle}} summarises the consequence: a system at equilibrium responds to a change by partly reversing it. Removing a product, such as CO<sub>2</sub> escaping from a hot brew, drives the reaction forward.

## A.5 Acids, bases and pH

An {{acid}} is a substance that donates a hydrogen ion (a proton, H<sup>+</sup>); a {{base}} accepts one. The {{conjugate base}} of an acid HA is the species A<sup>&minus;</sup> left after the proton is lost.

The concentration of free hydrogen ions is expressed on a logarithmic scale:

~~~eq ph
formula = "pH = &minus;log<sub>10</sub>[H<sup>+</sup>]"
where = [["[H<sup>+</sup>]", "concentration of hydrogen ions (strictly, activity)", "mol/L"]]
~~~

A brew at pH 5.0 has [H<sup>+</sup>] = 1.0 &times; 10<sup>&minus;5</sup> mol/L. Because the scale is logarithmic, a brew at pH 4.9 has 10<sup>0.1</sup> = 1.26 times as many free hydrogen ions.

A {{weak acid}} gives up its proton only partly. For HA &harr; H<sup>+</sup> + A<sup>&minus;</sup>, the equilibrium constant is called the {{acid dissociation constant}}:

~~~eq ka
formula = "K<sub>a</sub> = [H<sup>+</sup>][A<sup>&minus;</sup>] / [HA], and pK<sub>a</sub> = &minus;log<sub>10</sub> K<sub>a</sub>"
where = [["K<sub>a</sub>", "acid dissociation constant", "mol/L"], ["pK<sub>a</sub>", "its negative logarithm; smaller pK<sub>a</sub> means a stronger acid", ""]]
~~~

### What fraction of an acid is dissociated?

The fraction present as the conjugate base, &alpha;, follows from {eq:ka} in three steps. First, rearrange to [A<sup>&minus;</sup>]/[HA] = K<sub>a</sub>/[H<sup>+</sup>]. Second, write &alpha; = [A<sup>&minus;</sup>]/([HA] + [A<sup>&minus;</sup>]) and divide top and bottom by [A<sup>&minus;</sup>]: &alpha; = 1/(1 + [HA]/[A<sup>&minus;</sup>]) = 1/(1 + [H<sup>+</sup>]/K<sub>a</sub>). Third, express the ratio in logarithms: [H<sup>+</sup>]/K<sub>a</sub> = 10<sup>pK<sub>a</sub> &minus; pH</sup>. So

~~~eq alpha
formula = "&alpha; = 1 / (1 + 10<sup>pK<sub>a</sub> &minus; pH</sup>)"
where = [["&alpha;", "fraction of the acid present as its conjugate base", "fraction"]]
~~~

When pH equals pK<sub>a</sub>, &alpha; = 1/2: the acid is half dissociated. Each pH unit above pK<sub>a</sub> multiplies the ratio of conjugate base to acid by 10.

Approximate pK<sub>a</sub> values at 25 &deg;C for the acids of coffee:

| Acid | pK<sub>a</sub> values |
|---|---|
| Phosphoric | 2.15, 7.20, 12.35 |
| Citric | 3.13, 4.76, 6.40 |
| Malic | 3.40, 5.20 |
| Chlorogenic (5-caffeoylquinic, acid group) | about 3.4 |
| Quinic | about 3.5 |
| Lactic | 3.86 |
| Acetic | 4.76 |
| Carbonic | 6.35, 10.33 |

Acids with more than one value are {{polyprotic acid|polyprotic}}: they can give up more than one proton, each with its own pK<sub>a</sub>.

~~~example Speciation at brew pH
Problem. At pH 5.0, what fractions of acetic acid, 5-caffeoylquinic acid and citric acid (each of its three steps) are dissociated?

Plan. Apply {eq:alpha} to each pK<sub>a</sub>, treating the steps of citric acid as independent, a good approximation when the pK<sub>a</sub> values are well separated.

Solution. Acetic acid (4.76): 1/(1 + 10<sup>&minus;0.24</sup>) = 0.635, so 36.5% remains as the neutral molecule. 5-Caffeoylquinic acid (3.4): 1/(1 + 10<sup>&minus;1.6</sup>) = 0.975. Citric acid: first step 0.987, second 0.635, third 0.038.

Check. Two consequences follow. Most of the acid in a brew is already present as conjugate base, which is why the brew resists changes in pH. And the neutral fraction differs greatly between acids: acetic acid can be smelled only in its neutral, volatile form, and that form is a larger share for acetic acid than for any other main coffee acid.
~~~

## A.6 Buffers

A {{buffer}} is a mixture of a weak acid and its conjugate base. Taking logarithms of {eq:ka} and rearranging gives the {{Henderson-Hasselbalch equation}}:

~~~eq hh
formula = "pH = pK<sub>a</sub> + log<sub>10</sub>([A<sup>&minus;</sup>] / [HA])"
where = [["[A<sup>&minus;</sup>]/[HA]", "ratio of conjugate base to acid", ""]]
~~~

The pH depends on the ratio of base to acid, not on their total amount. Diluting a buffer leaves the ratio almost unchanged, so the pH barely moves, while the total amount of acid that can be titrated falls in proportion. Brewed coffee is a buffer of many acids and their salts. That is why stronger and weaker brews of the same coffee have almost the same pH but different titratable acidity, and why titratable acidity tracks sourness better (Chapters 0 and 1).

{{Buffer capacity}}, the amount of strong acid or base needed to change the pH by one unit, is greatest when pH equals pK<sub>a</sub> and grows with the total concentration of the buffer. Near brew pH 5, the second step of citric acid and acetic acid buffer most effectively.

~~~widget speciation
~~~

The calculator applies the same fractions to carbonic acid, whose forms control the behaviour of brew water (Chapter 3). Between pH 6.5 and 8.5, almost all dissolved inorganic carbon is bicarbonate.

## A.7 Solubility and scale

A sparingly soluble salt such as calcite (crystalline CaCO<sub>3</sub>) dissolves until an equilibrium is reached: CaCO<sub>3</sub>(s) &harr; Ca<sup>2+</sup> + CO<sub>3</sub><sup>2&minus;</sup>. Because the solid's concentration does not change, it is left out of the equilibrium constant, which is called the {{solubility product}}:

~~~eq ksp
formula = "K<sub>sp</sub> = [Ca<sup>2+</sup>][CO<sub>3</sub><sup>2&minus;</sup>] &asymp; 3.3 &times; 10<sup>&minus;9</sup> at 25 &deg;C"
where = [["K<sub>sp</sub>", "solubility product of calcite", "mol<sup>2</sup>/L<sup>2</sup>"]]
~~~

If the product of the actual concentrations, the {{ion product}}, exceeds K<sub>sp</sub>, the solution is {{supersaturated}} and calcite can deposit as scale.

Calcite becomes less soluble as temperature rises, unlike most salts. Heating also drives CO<sub>2</sub> out of solution, and bicarbonate then converts to carbonate: 2HCO<sub>3</sub><sup>&minus;</sup> &rarr; CO<sub>3</sub><sup>2&minus;</sup> + CO<sub>2</sub> + H<sub>2</sub>O. Both effects raise the ion product in a boiler, so water that is stable in a pipe can scale a coffee machine. Chapter 3 introduces the Langelier saturation index, which packages this calculation.

~~~example Will this water scale?
Problem. Boiler water contains Ca<sup>2+</sup> at 40 mg/L, and heating has raised carbonate to 1.0 mg/L. Is the water supersaturated with calcite?

Plan. Convert both ions to mol/L, multiply, and compare with K<sub>sp</sub>.

Solution. [Ca<sup>2+</sup>] = 0.040/40.08 = 9.98 &times; 10<sup>&minus;4</sup> mol/L. [CO<sub>3</sub><sup>2&minus;</sup>] = 0.0010/60.01 = 1.67 &times; 10<sup>&minus;5</sup> mol/L. Ion product = 1.66 &times; 10<sup>&minus;8</sup>, five times K<sub>sp</sub> at 25 &deg;C and further above it at boiler temperature. Calcite will deposit.

Check. Activity corrections lower the effective ion product by tens of percent at this concentration, not by a factor of five, so the conclusion holds.
~~~

## A.8 Logarithms and significant figures

pH, pK<sub>a</sub>, Arrhenius plots and first-order kinetics all use logarithms. The rules needed are log(ab) = log a + log b; log(a<sup>n</sup>) = n log a; ln x = 2.303 log<sub>10</sub> x; and e<sup>ln x</sup> = x. A pH quoted to two decimal places supports two significant figures in [H<sup>+</sup>]: pH 5.03 means [H<sup>+</sup>] = 9.3 &times; 10<sup>&minus;6</sup> mol/L. Report results to the precision the measurement supports (Chapter 0).

~~~keyterms
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "A.2"
prompt = "Convert 1.40% TDS to g/L, taking the density as 1.00 g/mL. Then estimate the error of the same approximation for an espresso at 10% TDS with density 1.04 g/mL."
solution = "1.40 g per 100 g; 100 g occupies 100 mL, so 14.0 g/L. For espresso, 10 g per 100 g; 100 g occupies 96.2 mL, so 104 g/L rather than 100 g/L, a 4% error."

[[p]]
tier = "A"
section = "A.5"
prompt = "How many times more free hydrogen ions does a brew at pH 4.80 have than one at pH 5.10?"
solution = "10<sup>0.30</sup> = 2.0 times."

[[p]]
tier = "B"
section = "A.1"
prompt = "You dissolve 0.120 g of Epsom salt in 1.000 L of water. Compute the magnesium concentration in mg/L and the hardness it contributes."
solution = "0.120/246.47 = 4.87 &times; 10<sup>&minus;4</sup> mol of MgSO<sub>4</sub>&middot;7H<sub>2</sub>O, so the same amount of Mg: 4.87 &times; 10<sup>&minus;4</sup> &times; 24.305 g = 11.8 mg/L. Hardness = 0.487 mmol/L &times; 100.09 = 48.7 mg/L as CaCO<sub>3</sub>."

[[p]]
tier = "B"
section = "A.2"
prompt = "How much sodium bicarbonate (84.01 g/mol) is needed per litre to make water with 40 mg/L alkalinity as CaCO<sub>3</sub>?"
solution = "40/50.04 = 0.799 meq/L = 0.799 mmol/L of NaHCO<sub>3</sub>; mass = 0.799 &times; 84.01 = 67.1 mg/L."

[[p]]
tier = "B"
section = "A.5"
prompt = "What fraction of malic acid's first acidic group is dissociated at pH 4.9? What fraction of its second?"
solution = "First (pK<sub>a</sub> 3.40): 1/(1 + 10<sup>&minus;1.5</sup>) = 0.969. Second (pK<sub>a</sub> 5.20): 1/(1 + 10<sup>0.3</sup>) = 0.334, treating the steps as independent."

[[p]]
tier = "B"
section = "A.6"
prompt = "An acetate buffer contains twice as much acetate as acetic acid. What is its pH? What happens to the pH if the buffer is diluted tenfold?"
solution = "pH = 4.76 + log 2 = 5.06. Dilution leaves the ratio unchanged, so the pH stays near 5.06 (small activity effects aside), while the buffer capacity falls tenfold."

[[p]]
tier = "B"
section = "A.6"
prompt = "Derive the Henderson-Hasselbalch equation from the definition of K<sub>a</sub>."
solution = "K<sub>a</sub> = [H<sup>+</sup>][A<sup>&minus;</sup>]/[HA]. Take log<sub>10</sub> of both sides: log K<sub>a</sub> = log[H<sup>+</sup>] + log([A<sup>&minus;</sup>]/[HA]). Multiply by &minus;1 and use pH = &minus;log[H<sup>+</sup>] and pK<sub>a</sub> = &minus;log K<sub>a</sub>: pK<sub>a</sub> = pH &minus; log([A<sup>&minus;</sup>]/[HA]). Rearrange: pH = pK<sub>a</sub> + log([A<sup>&minus;</sup>]/[HA])."

[[p]]
tier = "B"
section = "A.3"
prompt = "A 250 mL brew is made with water of 80 mg/L alkalinity. How many millimoles of acidic hydrogen does the water neutralise? Compare with a titratable acidity of 1.6 mmol/L in the finished cup."
solution = "80/50.04 = 1.60 meq/L; in 0.250 L that is 0.40 mmol. The finished cup's titratable acid is 1.6 &times; 0.25 = 0.40 mmol, so the water has neutralised as much acid as remains: half of the original titratable acid."

[[p]]
tier = "B"
section = "A.7"
prompt = "A water contains Ca<sup>2+</sup> at 20 mg/L and CO<sub>3</sub><sup>2&minus;</sup> at 0.30 mg/L. Is it supersaturated with calcite at 25 &deg;C?"
solution = "[Ca<sup>2+</sup>] = 0.020/40.08 = 4.99 &times; 10<sup>&minus;4</sup> mol/L; [CO<sub>3</sub><sup>2&minus;</sup>] = 0.00030/60.01 = 5.0 &times; 10<sup>&minus;6</sup> mol/L. Ion product = 2.5 &times; 10<sup>&minus;9</sup>, slightly below K<sub>sp</sub> = 3.3 &times; 10<sup>&minus;9</sup>: just undersaturated at 25 &deg;C, but heating could tip it over."

[[p]]
tier = "C"
section = "A.3"
prompt = "Explain why adding bicarbonate to brew water lowers perceived sourness more than adding the same number of millimoles of calcium chloride, even though both raise the mineral content."
solution = "Bicarbonate is a base and consumes coffee acid protons in a 1:1 ratio, converting acid to conjugate base and CO<sub>2</sub>, so titratable acidity and sourness fall. Calcium chloride is a neutral salt with no proton acceptor, so it barely changes titratable acidity. Any effect of calcium on flavour comes by other routes, not by neutralising acid."
~~~

## Further reading

OpenStax, *Chemistry 2e*, chapters on stoichiometry, equilibrium and acid-base equilibria (free online, CC BY-NC-SA). Any general chemistry textbook covers the same ground.
