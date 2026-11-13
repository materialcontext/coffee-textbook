+++
number = 0
label = "D"
title = "Analytical methods"
question = "How are the numbers in coffee research measured, and what limits each method?"
objectives = [
  "Explain the principle, calibration and main sources of error of gravimetry, titration, pH and conductivity measurement.",
  "Apply the Beer-Lambert law to UV-visible absorbance data.",
  "Describe HPLC, gas chromatography and mass spectrometry, and calculate chromatographic resolution.",
  "Explain GC-olfactometry, aroma extract dilution analysis and stable isotope dilution analysis.",
  "Describe NMR, colour measurement and particle sizing as used in coffee studies.",
]
+++
The chapters quote measurements made by many methods: a refractometer reading, a titration, an HPLC result, a mass spectrum, a colour number. To judge a result you need to know what the method measures, how it is calibrated and what can go wrong. This appendix covers each method the book cites, from the simplest to the most specialised.

## D.1 Gravimetry

{{Gravimetry}} measures mass. Oven drying gives moisture content (Chapter 6) and, applied to a brew, dissolved solids (Chapter 0). Its accuracy depends on the balance and on whether drying removes only water. Volatile compounds lost with the water bias the result upward; incomplete drying biases it downward. The usual test of completion is {{constant mass}}: successive weighings after further drying agree within a set tolerance.

## D.2 Titration and pH

In an acid-base {{titration}}, a solution of known concentration is added to a measured sample until an endpoint is reached: pH 8.2 for coffee acidity (Chapter 0) or pH 4.5 for water alkalinity, titrated with acid (Chapter 3). The titrant must be {{standardisation|standardised}}, its exact concentration found by titrating a {{primary standard}}, a pure, stable substance weighed accurately. Potassium hydrogen phthalate is the usual primary standard for sodium hydroxide, which absorbs CO<sub>2</sub> from the air and slowly loses strength.

A {{glass electrode}} measures pH. A thin glass membrane develops an electrical potential that depends on the hydrogen-ion activity on each side. The ideal response is given by the Nernst equation:

~~~eq nernst
formula = "slope = 2.303 RT / F = 59.16 mV per pH unit at 25 &deg;C"
where = [["F", "Faraday constant, 96,485", "C/mol"], ["T", "absolute temperature", "K"]]
~~~

The slope is proportional to absolute temperature, which is why pH meters need temperature compensation. Calibrating with two or three buffers of known pH fixes the meter's zero and slope. A slope below about 95% of the ideal value indicates an ageing or dirty electrode.

~~~example Calibrating a pH meter
Problem. A meter reads +177.0 mV in pH 4.01 buffer and 0.0 mV in pH 7.00 buffer at 25 &deg;C. What is its slope as a percentage of ideal, and what is the pH of a brew reading +118.0 mV?

Plan. The slope is the change in potential per pH unit between the buffers; then interpolate from the pH 7 buffer.

Solution. Slope = 177.0/(7.00 &minus; 4.01) = 59.2 mV per pH, 100.1% of the Nernst value. pH = 7.00 &minus; 118.0/59.2 = 5.01.

Check. A positive reading means the sample is more acidic than pH 7, as a brew should be.
~~~

## D.3 Conductivity and "TDS" meters

{{Conductivity}}, in &micro;S/cm, measures how well a solution carries electric current, which rises with the concentration and mobility of its ions. Handheld "TDS" meters for water multiply conductivity by a fixed factor, typically 0.5 to 0.7, to give mg/L, assuming a particular salt. Waters with equal readings can differ greatly in hardness and alkalinity, which is why Chapter 3 works from individual ion concentrations. A reading of 300 &micro;S/cm with a factor of 0.65 is displayed as 195 mg/L.

## D.4 Refractometry

A refractometer measures the refractive index of a liquid, which rises with dissolved solids. Coffee refractometers convert it to TDS with a calibration specific to coffee. Temperature, filtration of the sample and calibration against standards made by mass are the main controls of accuracy (Chapter 0, Section 4).

## D.5 UV-visible spectroscopy

Molecules with alternating double and single bonds absorb ultraviolet or visible light. A spectrophotometer passes light of one wavelength through a sample and measures the fraction transmitted. {{Absorbance}} is defined as A = log<sub>10</sub>(I<sub>0</sub>/I), where I<sub>0</sub> and I are the intensities entering and leaving. The {{Beer-Lambert law}} relates absorbance to concentration:

~~~eq beer
formula = "A = &epsilon; l c"
where = [["A", "absorbance", "dimensionless"], ["&epsilon;", "molar absorptivity", "L mol<sup>&minus;1</sup> cm<sup>&minus;1</sup>"], ["l", "path length through the sample", "cm"], ["c", "concentration", "mol/L"]]
~~~

The law holds best for dilute solutions, with absorbances between about 0.1 and 1.

~~~example Chlorogenic acids by absorbance
Problem. 0.50 g of ground coffee is extracted into 50 mL of hot water, filtered and diluted 1:50. The diluted extract has an absorbance of 0.300 at 325 nm in a 1 cm cell. Taking &epsilon; = 1.9 &times; 10<sup>4</sup> L mol<sup>&minus;1</sup> cm<sup>&minus;1</sup> for 5-caffeoylquinic acid (values vary with solvent), estimate the chlorogenic acid content of the coffee.

Plan. Find c from {eq:beer}, convert to mass concentration, undo the dilution, then divide by the coffee mass.

Solution. c = 0.300/(1.9 &times; 10<sup>4</sup> &times; 1) = 1.58 &times; 10<sup>&minus;5</sup> mol/L, or 5.6 mg/L as 5-caffeoylquinic acid (354.31 g/mol). Undiluted: 280 mg/L. In 0.050 L: 14.0 mg. In 0.50 g of coffee: 2.8% by mass.

Check. Other compounds also absorb at 325 nm, melanoidins most of all, so the result is an upper estimate of total chlorogenic acids, biased upward more in dark roasts.
~~~

## D.6 Chromatography

{{Chromatography}} separates a mixture by passing it through a column. A {{stationary phase}} is fixed inside the column and a {{mobile phase}} flows through it. Each compound spends a characteristic share of its time held by the stationary phase, so compounds leave the column at different {{retention time|retention times}}.

{{High-performance liquid chromatography}} (HPLC) pumps a liquid mobile phase at high pressure through a packed column. Reversed-phase columns, with a non-polar stationary phase and a water-methanol or water-acetonitrile mobile phase, separate caffeine, trigonelline and the chlorogenic acid isomers. A UV detector measures each compound as it leaves, quantified against standards.

Gas chromatography (GC) uses a gas mobile phase and a heated capillary column, for volatile compounds. Coffee volatiles must first be collected: by solvent extraction, by solvent-assisted flavour evaporation (SAFE), by sampling the gas above the coffee, or by solid-phase microextraction (SPME), in which a coated fibre absorbs volatiles. The choice of method changes which compounds are recovered (Chapter 1).

{{Resolution}} measures how well two neighbouring peaks are separated:

~~~eq rs
formula = "R<sub>s</sub> = 2(t<sub>2</sub> &minus; t<sub>1</sub>) / (w<sub>1</sub> + w<sub>2</sub>)"
where = [["t<sub>1</sub>, t<sub>2</sub>", "retention times of the two peaks", "min"], ["w<sub>1</sub>, w<sub>2</sub>", "widths of the peaks at their base", "min"]]
~~~

R<sub>s</sub> = 1.5 gives essentially complete separation.

## D.7 Mass spectrometry

A {{mass spectrometer}} turns molecules into ions and separates the ions by their mass-to-charge ratio, m/z. In {{electron ionisation}}, the standard source for GC-MS, energetic electrons knock an electron out of each molecule, and the resulting ion often breaks into fragments in a reproducible pattern that can be matched against libraries. 2-Furfurylthiol (C<sub>5</sub>H<sub>6</sub>OS, molar mass 114) shows its molecular ion at m/z 114 and a dominant fragment at m/z 81, the furfuryl cation: the same species from which it forms during roasting (Chapter 8).

Gentler ionisation fragments less. {{Proton-transfer-reaction mass spectrometry}} (PTR-MS) transfers a proton from H<sub>3</sub>O<sup>+</sup> ions to volatile molecules, giving mostly the protonated molecule. It needs no chromatography and can follow roaster exhaust once per second (Chapter 7), but it cannot separate different compounds that have the same mass.

### Stable isotope dilution analysis

Recovery of trace volatiles varies from sample to sample. In {{stable isotope dilution analysis}} (SIDA), a known amount of the target compound labelled with a heavy isotope (deuterium or carbon-13) is added at the start. It behaves chemically like the natural compound through every step but has a different mass, so the mass spectrometer can measure both:

~~~eq sida
formula = "n<sub>analyte</sub> = n<sub>label</sub> &times; (A<sub>analyte</sub> / A<sub>label</sub>) &times; RF"
where = [["n", "amount of analyte or of added label", "ng"], ["A", "peak area of analyte or label", ""], ["RF", "response factor, measured from known mixtures", "dimensionless"]]
~~~

Losses affect the analyte and the label equally and cancel in the ratio, which makes SIDA the reference method for trace odorants such as thiols.

## D.8 GC-olfactometry and dilution analysis

In {{gas chromatography-olfactometry}} (GC-O), the gas leaving the column is split between a detector and a sniffing port, where an assessor records the time and character of each odour. Aroma extract dilution analysis (AEDA) repeats this on a series of dilutions, and a compound's flavour dilution factor is the highest dilution at which it can still be smelled (Chapter 1, Equation 1.3). These methods rank odorants by potency, but the ranking depends on the extraction, the assessors and the column, and says nothing about interactions in the cup. Recombination and omission tests address those.

## D.9 NMR spectroscopy

{{Nuclear magnetic resonance}} (NMR) spectroscopy places a sample in a strong magnetic field and measures how atomic nuclei with magnetic properties, such as <sup>1</sup>H and <sup>13</sup>C, absorb radio-frequency energy. The absorption frequency depends on each nucleus's chemical surroundings; it is reported as a {{chemical shift}} in parts per million. In <sup>1</sup>H NMR, the area of each signal is proportional to the number of hydrogen atoms producing it. Quantitative NMR can therefore measure caffeine, trigonelline, chlorogenic acids, acetic and formic acids and sugars in a brew with little preparation and a single internal standard, which is why roasting studies use it alongside GC-MS (Chapter 7). It is much less sensitive than mass spectrometry: it sees major and minor components, not trace odorants.

## D.10 Colour measurement

Roast colour is measured by reflectance. Near-infrared instruments built for coffee report a number on their own scale. Colorimeters report CIE L&#42;a&#42;b&#42; coordinates: L&#42; is lightness from 0 (black) to 100 (white), a&#42; runs from green (negative) to red (positive), and b&#42; from blue (negative) to yellow (positive). The colour difference between two samples is

~~~eq de
formula = "&Delta;E = &radic;[(&Delta;L&#42;)<sup>2</sup> + (&Delta;a&#42;)<sup>2</sup> + (&Delta;b&#42;)<sup>2</sup>]"
where = [["&Delta;E", "colour difference", "dimensionless"]]
~~~

Readings depend on grind, packing, sample surface and instrument, so comparisons are valid only within one instrument and protocol (Chapters 0 and 7).

## D.11 Particle sizing

Sieving, laser diffraction and image analysis are compared in Chapter 4. They differ chiefly in how particles are weighted (by mass, volume or number) and in how irregular or clumping particles are handled, so results from different methods should not be compared directly.

~~~keyterms
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "D.2"
prompt = "Why must sodium hydroxide titrant be standardised regularly?"
solution = "It absorbs CO<sub>2</sub> from the air, forming carbonate and lowering its effective concentration. It also absorbs water, so its concentration cannot be set accurately by weighing the solid."

[[p]]
tier = "A"
section = "D.3"
prompt = "Two waters both read 150 mg/L on a TDS meter. Can they have different alkalinities? Explain."
solution = "Yes. The meter measures total conductivity and applies one conversion factor, and different mixtures of ions (for example sodium chloride against calcium bicarbonate) can give the same conductivity with very different alkalinity and hardness."

[[p]]
tier = "A"
section = "D.6"
prompt = "What does a resolution of 1.5 mean for two chromatographic peaks?"
solution = "The peaks are separated to the baseline; for equal Gaussian peaks the overlap is about 0.1%."

[[p]]
tier = "B"
section = "D.2"
prompt = "A pH electrode reads +165 mV in pH 4.01 buffer and +2 mV in pH 7.00 buffer at 25 &deg;C. Compute its slope as a percentage of ideal, and the pH of a sample reading +120 mV."
solution = "Slope = (165 &minus; 2)/2.99 = 54.5 mV per pH, 92% of 59.16; the electrode should be cleaned or replaced. pH = 7.00 &minus; (120 &minus; 2)/54.5 = 4.83."

[[p]]
tier = "B"
section = "D.5"
prompt = "A caffeine standard (&epsilon; = 9,700 L mol<sup>&minus;1</sup> cm<sup>&minus;1</sup> at 273 nm) gives A = 0.485 in a 1 cm cell. What is its concentration in mg/L?"
solution = "c = 0.485/9,700 = 5.0 &times; 10<sup>&minus;5</sup> mol/L; &times; 194.19 g/mol = 9.7 mg/L."

[[p]]
tier = "B"
section = "D.6"
prompt = "Peaks at 5.20 and 5.65 min have base widths of 0.30 and 0.32 min. Compute the resolution."
solution = "R<sub>s</sub> = 2(0.45)/0.62 = 1.45, close to baseline separation."

[[p]]
tier = "B"
section = "D.7"
prompt = "In a SIDA measurement, 50 ng of deuterated 2-furfurylthiol is added to a coffee sample. The analyte-to-label peak area ratio is 0.84 and the response factor is 1.05. How much 2-furfurylthiol was in the sample?"
solution = "50 &times; 0.84 &times; 1.05 = 44 ng."

[[p]]
tier = "B"
section = "D.10"
prompt = "Two roasts measure L&#42;a&#42;b&#42; = (24.1, 9.8, 14.2) and (22.6, 9.1, 12.9). Compute &Delta;E."
solution = "&radic;(1.5<sup>2</sup> + 0.7<sup>2</sup> + 1.3<sup>2</sup>) = &radic;(2.25 + 0.49 + 1.69) = 2.10."

[[p]]
tier = "C"
section = "D.7"
prompt = "A study reports no 2-furfurylthiol among its key odorants using SPME and GC-MS. Explain why, and propose a method that would detect it."
solution = "Thiols are present at trace levels, oxidise and bind during sampling, and SPME with full-scan MS may lack the selectivity and sensitivity. Use a thiol-selective enrichment step, SIDA with a labelled standard, and GC with sulfur-selective detection or MS set to record only m/z 81 and 114."

[[p]]
tier = "C"
section = "D.9"
prompt = "Compare NMR and GC-MS for following acetic acid and 2-furfurylthiol during roasting, and choose a method for each."
solution = "Acetic acid is a major component (hundreds of mg/kg or more), so quantitative <sup>1</sup>H NMR measures it directly with an internal standard and little preparation. 2-Furfurylthiol is present at &micro;g/kg levels, far below NMR's sensitivity, so use GC-MS with SIDA after selective enrichment."
~~~

## Further reading

Harris D.C., *Quantitative Chemical Analysis*; Skoog D.A., Holler F.J., Crouch S.R., *Principles of Instrumental Analysis*; papers by Grosch, Schieberle and colleagues on AEDA and SIDA for food odorants.
